use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::{Context, Result};
use parking_lot::Mutex;
use reqwest::{Client, Response};
use reqwest::header::{AsHeaderName, HeaderMap};
use serde::de::DeserializeOwned;
use tokio::task::JoinHandle;
use uuid::Uuid;
use crate::api::throttler::RequestThrottler;

pub struct RequestHandler {
    client: Client,
    api_key: Uuid,
    throttler: Arc<Mutex<RequestThrottler>>,
}

impl RequestHandler {
    pub fn new(api_key: Uuid) -> Self {
        RequestHandler {
            client: Client::new(),
            api_key,
            throttler: RequestThrottler::new(),
        }
    }

    /// Call this function from an async context!
    pub fn request<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> JoinHandle<Result<T>> {
        let url = format!("https://api.hypixel.net/{}", path);
        let api_key = self.api_key.to_hyphenated().to_string();
        let client = self.client.clone();
        let throttler = Arc::clone(&self.throttler);
        tokio::spawn(async move {
            loop {
                match RequestHandler::try_request(client.clone(), url.clone(), api_key.clone(), Arc::clone(&throttler)).await {
                    Ok(Some(response)) => break response.json::<T>().await.with_context(|| format!("Error while deserializing response!")),
                    Err(error) => break Err(error),
                    _ => {}
                }
            }
        })
    }

    async fn try_request(client: Client, url: String, api_key: String, throttler: Arc<Mutex<RequestThrottler>>) -> Result<Option<Response>> {
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
            .send().await.with_context(|| format!("Could not send request to {}", url))?;

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

fn get_from_headers<K: AsHeaderName, E: Error + Send + Sync + 'static, T: FromStr<Err=E> + Copy>(headers: &HeaderMap, name: K, default: T) -> Result<T> {
    headers.get(name)
        .map(|o| o.to_str())
        .map(|o| o.map_or(Ok(default), |s| s.parse::<T>().with_context(|| format!("Invalid response from api!"))))
        .unwrap_or(Ok(default))
}