use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use parking_lot::Mutex;
use reqwest::{Client, Response};
use reqwest::header::{AsHeaderName, HeaderMap};
use serde::de::DeserializeOwned;
use tokio::task::JoinHandle;
use uuid::Uuid;
use crate::api::error::HypixelApiError;
use crate::api::throttler::RequestThrottler;

#[derive(Debug)]
pub struct RequestHandler {
    client: Client,
    api_key: Uuid,
    throttler: Arc<Mutex<RequestThrottler>>,
}

impl RequestHandler {
    /// Creates a new RequestHandler instance using an
    /// [api_key](https://api.hypixel.net/#section/Authentication)
    /// obtained from Hypixel.
    ///
    /// [`RequestHandler::request`] can be used to queue as many
    /// requests as required for execution without ever going over
    /// the limit set by Hypixel's API. This limit is derived
    /// automatically and thus can be completely avoided by user code.
    ///
    /// # Examples
    /// ```rust
    /// use hypixel_api::RequestHandler;
    /// # use uuid::Uuid;
    /// # use std::str::FromStr;
    ///
    /// # fn main() {
    /// let api_key = Uuid::from_str(env!("HYPIXEL_API_KEY")).unwrap();
    /// let request_handler = RequestHandler::new(api_key);
    ///
    /// // Send requests ...
    /// # }
    /// ```
    pub fn new(api_key: Uuid) -> Self {
        RequestHandler {
            client: Client::new(),
            api_key,
            throttler: RequestThrottler::new(),
        }
    }

    /// Queues a new request for execution and returns a [`JoinHandle`] to it.
    ///
    /// ## Arguments
    /// `path` should be a relative path to the API (without leading `/`), such as `"key"`
    /// or `"status?uuid=..."`. See the [API](https://api.hypixel.net/).
    ///
    /// # Errors
    ///
    /// If any part of the execution process fails, a [`HypixelApiError`] will be returned.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use uuid::Uuid;
    /// # use std::str::FromStr;
    /// # use hypixel_api::StatusReply;
    /// use hypixel_api::RequestHandler;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let api_key = Uuid::from_str(env!("HYPIXEL_API_KEY")).unwrap();
    /// let request_handler = RequestHandler::new(api_key);
    /// let request1 = request_handler.request::<StatusReply>("status?uuid=069a79f4-44e9-4726-a5be-fca90e38aaf5");
    ///
    /// // send more requests ...
    ///
    /// let reply: StatusReply = request1.await.unwrap().unwrap();
    /// // use reply ...
    /// # }
    /// ```
    #[tracing::instrument(name = "queue_req", skip(self))]
    pub fn request<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> JoinHandle<Result<T, HypixelApiError>> {
        let url = format!("https://api.hypixel.net/{}", path);
        let api_key = self.api_key.to_hyphenated().to_string();
        let client = self.client.clone();
        let throttler = Arc::clone(&self.throttler);
        tokio::spawn(async move {
            loop {
                match RequestHandler::try_request(client.clone(), url.clone(), api_key.clone(), Arc::clone(&throttler)).await {
                    Ok(Some(response)) => break response.json::<T>().await.map_err(|e| e.into()),
                    Err(error) => break Err(error),
                    _ => {}
                }
            }
        })
    }

    #[tracing::instrument(name = "try_send", level = "trace", skip_all)]
    async fn try_request(client: Client, url: String, api_key: String, throttler: Arc<Mutex<RequestThrottler>>) -> Result<Option<Response>, HypixelApiError> {
        let mut watcher = None;
        loop {
            let ticket = {
                let mut throttler = throttler.lock();
                let (ticket, wait_rx) = throttler.request_ticket();
                if watcher.is_none() {
                    watcher = Some(wait_rx);
                }
                ticket
            };
            if ticket {
                break Ok(());
            }
            if let Err(error) = watcher.as_mut().unwrap().changed().await {
                break Err(error);
            }
        }?;

        let response = client.get(&url)
            .header("API-Key", api_key)
            .send().await?;

        let status_code = response.status();
        let headers = response.headers();
        let time_before_reset = get_from_headers(headers, "ratelimit-reset", 10)?.max(1);
        let requests_remaining = get_from_headers(headers, "ratelimit-remaining", 110)?.max(1);
        if {
            let mut throttler = throttler.lock();
            throttler.on_received(status_code, time_before_reset, requests_remaining)
        }? {
            Ok(Some(response))
        } else {
            Ok(None)
        }
    }
}

fn get_from_headers<K: AsHeaderName, E: Error + Send + Sync + 'static, T: FromStr<Err=E> + Copy>(headers: &HeaderMap, name: K, default: T) -> Result<T, HypixelApiError> {
    headers.get(name)
        .map(|o| o.to_str())
        .map(|o| o.map_or(Ok(default), |s| s.parse::<T>().map_err(|_| HypixelApiError::IntFromStrError(String::from(s)))))
        .unwrap_or(Ok(default))
}