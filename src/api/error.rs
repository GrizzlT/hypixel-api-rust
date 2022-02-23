//! Error type(s) used throughout this library.

use std::time::Duration;

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HypixelApiError {
    #[error("Could not parse {0} into integer")]
    IntFromStrError(String),
    #[error("Unexpected response code received: {0}, {1:?}")]
    UnexpectedResponseCode(StatusCode, Option<ErrorReply>),
    #[error("Error from reqwest!")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("Could not send time to dedicated thread")]
    TokioSendTime {
        #[from]
        source: tokio::sync::mpsc::error::TrySendError<Option<Duration>>,
    },
    #[error("Error while receiving watcher update")]
    TokioReceive {
        #[from]
        source: tokio::sync::watch::error::RecvError,
    },
    #[error("Error while deserializing from json")]
    SerdeJsonError {
        #[from]
        source: serde_json::Error,
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorReply {
    success: bool,
    cause: String,
}

impl ErrorReply {
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn cause(&self) -> &str {
        &self.cause
    }
}