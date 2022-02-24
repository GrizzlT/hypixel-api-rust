# hypixel-api-rust
[![Latest Version](https://img.shields.io/crates/v/hypixel_api)](https://crates.io/crates/hypixel_api)

This is a Rust implementation of the [Hypixel API](https://github.com/HypixelDev/PublicAPI).

Hypixel's Public API imposes a default limit of 120 requests/minute. This crate 
offers a wrapper around this dynamic limit that's fully asynchronous and can query any data type that implements
[`DeserializeOwned`](https://docs.serde.rs/serde/de/trait.DeserializeOwned.html) from [serde](https://serde.rs).

## Getting started
As explained in the [Hypixel Public API](https://api.hypixel.net/), an `ApiKey` is required to
query most of the endpoints in the API.

The heart of this crate is the `RequestHandler`.\
Internally, it keeps track of how many requests the program has sent in the past minute,
keeps an overflow of requests pending when necessary and always stays synchronized
with the internal clock of the `Hypixel REST API`.

A response from the API can be deserialized into any data structure that implements
[`DeserializeOwned`](https://docs.serde.rs/serde/de/trait.DeserializeOwned.html) by [serde](https://serde.rs).
#### Basic example
```rust
use hypixel_api::RequestHandler;
use hypixel_api::StatusReply;

let api_key = Uuid::from_str("your-api-key").unwrap(); // get your ApiKey
let request_handler = RequestHandler::new(api_key); // initialize a new RequestHandler

let response = request_handler.request::<StatusReply>("status?uuid=069a79f4-44e9-4726-a5be-fca90e38aaf5", true); // query the status of Notch
// send more requests ...

let data: StatusReply = response.await.unwrap().unwrap();
// use data ...
```
Any `Result::Ok` response by this crate will guarantee to be a `200 OK` response from
the API and thus by consequence guarantee to be deserializable into a corresponding data struct.

Currently, many example response data structures are unimplemented. This does not impact
this crate's ability to still query that data. Simply define your own  data structure and
use it with `RequestHandler` like usual. You could even replace all pre-made data structures
and go fully custom. (Feel free to contribute to this repository!!)

See the documentation for more examples.

## Features
- `util` - enables the utility functions to process data returned by the `Hypixel Public API`
- `reply` - (*depends on `util`*) - enables ready-to-use data structures as responses from the `Hypixel Public API`

---
# License

Licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Please feel free to contribute, every attempt counts!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
