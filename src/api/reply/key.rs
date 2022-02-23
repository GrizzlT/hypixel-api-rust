use std::ops::Deref;
use serde::Deserialize;
use uuid::Uuid;

/// A data structure that maps to [`this endpoint`](https://api.hypixel.net/#tag/API/paths/~1key/get).
///
/// Response fields are captured in [`KeyData`].
#[derive(Debug, Copy, Clone, Deserialize)]
pub struct KeyReply {
    success: bool,
    record: KeyData,
}

impl KeyReply {
    /// Returns whether the response was successful.
    ///
    /// This should always return true. (not guaranteed though)
    pub fn success(&self) -> bool {
        self.success
    }
}

impl Deref for KeyReply {
    type Target = KeyData;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

/// The response data corresponding to [`this endpoint`](https://api.hypixel.net/#tag/API/paths/~1key/get).
///
/// All fields are captured, except the repetition
/// of the actual `ApiKey` used to send the request.
/// (This being due to security reasons)
#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    queries_in_past_min: i32,
    owner: Uuid,
    limit: i32,
    total_queries: i32,
}

impl KeyData {
    /// Returns the amount of request in the past minute.
    ///
    /// This is given by the Hypixel API and as of right now
    /// seems to be an inaccurate representation.
    pub fn past_min(&self) -> i32 {
        self.queries_in_past_min
    }

    /// Returns the UUID of the player that owns the
    /// key that this response was requested with.
    pub fn owner(&self) -> Uuid {
        self.owner
    }

    /// Returns the limit this particular key
    /// has. Default value is 120 by the API.
    pub fn limit(&self) -> i32 {
        self.limit
    }

    /// Returns the total amount of queries that have been
    /// executed by this player.
    pub fn total_queries(&self) -> i32 {
        self.total_queries
    }
}