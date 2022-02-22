//!
//! A basic wrapper around the [Hypixel Public API](https://api.hypixel.net)
//! that's fully asynchronous and can query any data type that implements
//! [`DeserializeOwned`](https://docs.serde.rs/serde/de/trait.DeserializeOwned.html) from serde.
//!
//! Hypixel's Public API imposes a default limit of 120 requests/minute. This crate
//! offers a wrapper around this dynamic limit to avoid hitting it and
//! receiving useless error responses.
//!
//! Any [`Result::Ok`] response by this crate will guarantee to be a `200 OK` from the API
//! and thus by consequence guarantee to be deserializable into a corresponding data struct (see [`reply`]
//! for examples).
//!
//! View the [`RequestHandler`] documentation for more information on sending requests.

#[macro_use]
extern crate tracing;

mod api;

pub use api::error;
pub use api::reply;

pub use api::request::RequestHandler;
pub use api::reply::*;