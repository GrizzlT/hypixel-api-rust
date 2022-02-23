//!
//! A basic wrapper around the [Hypixel Public API](https://api.hypixel.net)
//! that's fully asynchronous and can query any data type that implements
//! [`DeserializeOwned`](https://docs.serde.rs/serde/de/trait.DeserializeOwned.html) from serde.
//!
//! Hypixel's Public API imposes a default limit of 120 requests/minute. This crate
//! offers a wrapper around this dynamic limit to avoid hitting it and
//! receiving useless error responses.
//!
//! # Getting started
//!
//! As explained in the [Hypixel Public API](https://api.hypixel.net/), an `ApiKey` is required to
//! query most of the endpoints in the API.
//!
//! The heart of this crate is the [`RequestHandler`].\
//! Internally, it keeps track of how many requests the program has sent in the past minute,
//! keeps an overflow of requests pending when necessary and always stays synchronized
//! with the internal clock of the `Hypixel REST API`.
//!
//! A response from the API can be deserialized into any data structure that implements
//! [`DeserializeOwned`](https://docs.serde.rs/serde/de/trait.DeserializeOwned.html) by [serde](https://serde.rs).
//! #### Basic example
//! ```rust,no_run
//! use hypixel_api::RequestHandler;
//! use hypixel_api::StatusReply;
//! # use uuid::Uuid;
//! # use std::str::FromStr;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let api_key = Uuid::from_str("your-api-key").unwrap(); // get your ApiKey
//! let request_handler = RequestHandler::new(api_key); // initialize a new RequestHandler
//!
//! let response = request_handler.request::<StatusReply>("status?uuid=069a79f4-44e9-4726-a5be-fca90e38aaf5", true); // query the status of Notch
//! // send more requests ...
//!
//! let data: StatusReply = response.await.unwrap().unwrap();
//! // use data ...
//! # }
//! ```
//!
//! Any [`Result::Ok`] response by this crate will guarantee to be a `200 OK` response from
//! the API and thus by consequence guarantee to be deserializable into a corresponding data
#![cfg_attr(feature = "reply", doc = "struct (see [`reply`] for examples).")]
#![cfg_attr(not(feature = "reply"), doc = "struct.")]
//!
//! Currently, many example response data structures are unimplemented. This does not impact
//! this crate's ability to still query that data. Simply define your own  data structure and
//! use it with [`RequestHandler`] like usual. You could even replace all pre-made data structures
//! and go fully custom.
//!
//! See the documentation of [`RequestHandler`] for more information on sending requests.
//!
//! #### Example with custom data structure
//! ```rust,no_run
//! use hypixel_api::RequestHandler;
//! # use uuid::Uuid;
//! # use std::str::FromStr;
//! # use serde::Deserialize;
//!
//! // Simplistic Key data
//! #[derive(Deserialize)]
//! pub struct MyCustomKeyReply {
//!     success: bool,
//!     record: KeyData,
//! }
//!
//! #[derive(Deserialize)]
//! pub struct KeyData {
//!     owner: Uuid,
//!     limit: i32,
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let api_key = Uuid::from_str("your-api-key").unwrap();
//! let request_handler = RequestHandler::new(api_key); // initialize a RequestHandler
//!
//! let response = request_handler.request::<MyCustomKeyReply>("key", true); // query https://api.hypixel.net/key
//! // do something ...
//!
//! let data: MyCustomKeyReply = response.await.unwrap().unwrap();
//! // use data ...
//! # }
//! ```
//!
//! # Features
//! - `util` - enables the utility functions to process data returned by the `Hypixel Public API`
#![cfg_attr(feature = "util", doc = ", see [`util`]")]
//! - `reply` - (*depends on `util`*) - enables ready-to-use data structures as responses from the `Hypixel Public API`
#![cfg_attr(feature = "reply", doc = ", see [`reply`]")]

#[cfg_attr(feature = "tracing", macro_use)]
#[cfg(feature = "tracing")]
extern crate tracing;

mod api;

pub use api::error;
#[cfg(feature = "reply")]
pub use api::reply;
#[cfg(feature = "util")]
pub use api::util;

pub use api::request::RequestHandler;
#[cfg(feature = "reply")]
pub use api::reply::*;
pub use api::{ColorCodes, MonthlyPackageRank, PackageRank, StaffLevel};