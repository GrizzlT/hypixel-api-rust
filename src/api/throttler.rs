use std::sync::Arc;
use std::time::Duration;
use parking_lot::Mutex;
use reqwest::StatusCode;
use anyhow::{Error, Result};
use ignore_result::Ignore;
use tokio::runtime;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Instant};

pub struct RequestThrottler {
    requests_left: u32,
    received_first: bool,
    overflow_flagged: bool,
    notify_rx: watch::Receiver<()>,
    time_tx: mpsc::Sender<Option<Duration>>,
}

impl RequestThrottler {
    /// Call this function from an async context
    pub fn new() -> Arc<Mutex<Self>> {
        let (notify_tx, notify_rx) = watch::channel(());
        let (time_tx, time_rx) = mpsc::channel(5);
        let handler = Arc::new(Mutex::new(RequestThrottler {
            requests_left: 1,
            received_first: false,
            overflow_flagged: false,
            notify_rx,
            time_tx,
        }));
        let handler_cloned = Arc::clone(&handler);
        std::thread::spawn(move || {
            runtime::Builder::new_current_thread()
                .enable_time()
                .build().unwrap()
                .block_on(RequestThrottler::start_waiting(handler_cloned, notify_tx, time_rx))
        });
        handler
    }

    pub fn request_ticket(&mut self) -> (bool, watch::Receiver<()>) {
        let allow_pass = if self.requests_left > 0 {
            self.requests_left -= 1;
            true
        } else {
            false
        };
        (allow_pass, self.notify_rx.clone())
    }

    pub fn on_received(&mut self, status_code: StatusCode, time_before_reset: u64, requests_remaining: u32) -> Result<bool> {
        match status_code {
            StatusCode::TOO_MANY_REQUESTS => {
                println!("Too many requests!");
                println!("Vars: {}, {}", time_before_reset, requests_remaining);
                if !self.overflow_flagged {
                    self.overflow_flagged = true;
                    self.requests_left = 0;
                    self.time_tx.try_send(Some(Duration::from_secs(time_before_reset + 2))).ignore();
                }
                Ok(false)
            }
            StatusCode::OK => {
                if !self.received_first {
                    self.received_first = true;
                    self.requests_left = requests_remaining;
                    self.time_tx.try_send(Some(Duration::from_secs(time_before_reset + 2))).ignore();
                    self.time_tx.try_send(None).ignore();
                }
                Ok(true)
            }
            code => return Err(Error::msg(format!("Unexpected response code: {}", code))),
        }
    }

    async fn start_waiting(throttler: Arc<Mutex<RequestThrottler>>, wait_tx: watch::Sender<()>, mut time_rx: mpsc::Receiver<Option<Duration>>) {
        let sleeper = sleep(Duration::from_millis(10));
        tokio::pin!(sleeper);
        let mut duration_set = false;
        loop {
            tokio::select! {
                () = &mut sleeper, if duration_set => {
                    duration_set = false;
                    {
                        let mut throttler = throttler.lock();
                        throttler.received_first = false;
                        throttler.overflow_flagged = false;
                        throttler.requests_left = 1;
                    }
                    if let Err(error) = wait_tx.send(()) {
                        println!("Error while sending! {}", error);
                    }
                }
                duration = time_rx.recv() => {
                    match duration {
                        Some(duration) => {
                            match duration {
                                Some(duration) => {
                                    sleeper.as_mut().reset(Instant::now() + duration);
                                    duration_set = true;
                                }
                                None => {
                                    if let Err(error) = wait_tx.send(()) {
                                        println!("Error while sending! {}", error);
                                    }
                                }
                            }
                        }
                        None => break,
                    }
                }
            }
        }
    }
}