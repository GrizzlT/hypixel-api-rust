use std::error::Error;
use std::fmt::Formatter;
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
use crate::error::ErrorReply;

pub struct RequestHandler {
    client: Client,
    api_key: Uuid,
    throttler: Arc<Mutex<RequestThrottler>>,
}

impl std::fmt::Debug for RequestHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestHandler")
            .field("client", &self.client)
            .field("throttler", &self.throttler)
            .finish()
    }
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
    /// If `authenticated` is `true` then the API key will be sent along as a header.
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
    /// let request1 = request_handler.request::<StatusReply>("status?uuid=069a79f4-44e9-4726-a5be-fca90e38aaf5", true);
    ///
    /// // send more requests ...
    ///
    /// let reply: StatusReply = request1.await.unwrap().unwrap();
    /// // use reply ...
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(name = "queue_req", skip(self)))]
    pub fn request<T: DeserializeOwned + Send + 'static>(&self, path: &str, authenticated: bool) -> JoinHandle<Result<T, HypixelApiError>> {
        let url = format!("https://api.hypixel.net/{}", path);
        let api_key = self.api_key.hyphenated().to_string();
        let client = self.client.clone();
        let throttler = Arc::clone(&self.throttler);
        tokio::spawn(async move {
            let client = client;
            let url = url;
            let api_key = api_key;
            let throttler = throttler;
            loop {
                match RequestHandler::try_request(&client, &url, &api_key, &throttler, authenticated).await {
                    Ok(Some(response)) => break response.json::<T>().await.map_err(|e| e.into()),
                    Err(error) => break Err(error),
                    _ => {}
                }
            }
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(name = "try_send", level = "trace", skip_all))]
    async fn try_request(client: &Client, url: &str, api_key: &str, throttler: &Arc<Mutex<RequestThrottler>>, authenticated: bool) -> Result<Option<Response>, HypixelApiError> {
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

        let mut response = client.get(url);
        if authenticated {
            response = response.header("API-Key", api_key);
        }
        let response = response.send().await?;

        let status_code = response.status();
        let headers = response.headers();
        let time_before_reset = get_from_headers(headers, "ratelimit-reset", 10)?.max(1);
        let requests_remaining = get_from_headers(headers, "ratelimit-remaining", 110)?.max(1);
        let result_check = {
            let mut throttler = throttler.lock();
            throttler.on_received(status_code, time_before_reset, requests_remaining)
        };
        match result_check {
            Ok(result) => {
                if result {
                    Ok(Some(response))
                } else {
                    Ok(None)
                }
            }
            Err(HypixelApiError::UnexpectedResponseCode(code, _)) => {
                let cause = response.json::<ErrorReply>().await.ok();
                Err(HypixelApiError::UnexpectedResponseCode(code, cause))
            }
            Err(error) => Err(error)
        }
    }
}

fn get_from_headers<K: AsHeaderName, E: Error + Send + Sync + 'static, T: FromStr<Err=E> + Copy>(headers: &HeaderMap, name: K, default: T) -> Result<T, HypixelApiError> {
    headers.get(name)
        .map(|o| o.to_str())
        .map(|o| o.map_or(Ok(default), |s| s.parse::<T>().map_err(|_| HypixelApiError::IntFromStrError(String::from(s)))))
        .unwrap_or(Ok(default))
}
