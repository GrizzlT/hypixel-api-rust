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
//! and thus by consequence guarantee to be deserializable into a corresponding data
#![cfg_attr(feature = "reply", doc = "struct (see [`reply`] for examples).")]
#![cfg_attr(not(feature = "reply"), doc = "struct.")]
//!
//! View the [`RequestHandler`] documentation for more information on sending requests.

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