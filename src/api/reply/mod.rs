//! This module provides ready-to-use examples of
//! data structures that link to responses from
//! Hypixel's Public API.

mod player;
mod status;
mod key;

pub use player::{PlayerReply, PlayerData};
pub use status::{StatusReply, StatusData};
pub use key::{KeyReply, KeyData};
