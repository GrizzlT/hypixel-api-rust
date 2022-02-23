use std::ops::Deref;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct StatusReply {
    success: bool,
    #[serde(flatten)]
    data: StatusData,
}

impl StatusReply {
    /// Returns whether the response was successful.
    ///
    /// This should always return true. (not guaranteed though)
    pub fn success(&self) -> bool {
        self.success
    }
}

impl Deref for StatusReply {
    type Target = StatusData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusData {
    uuid: Uuid,
    session: SessionData,
}

impl StatusData {
    /// Returns the player's UUID.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Returns `true` if the player is online.
    ///
    /// Players have the option to hide this value in their settings:
    /// `false` means either offline or online but hidden.
    pub fn online(&self) -> bool {
        self.session.online
    }

    /// Returns the type of game the player is currently playing, if present.
    ///
    /// TODO: This will be changed into an enum
    pub fn game_type(&self) -> Option<&str> {
        self.session.game_type.as_deref()
    }

    /// Returns the mode of the game the player is playing, if present.
    pub fn mode(&self) -> Option<&str> {
        self.session.mode.as_deref()
    }

    /// Returns the map the player is playing on, if present.
    pub fn map(&self) -> Option<&str> {
        self.session.map.as_deref()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct SessionData {
    online: bool,
    /// TODO: chage into enum for easier game sorting
    #[serde(rename = "gameType")]
    game_type: Option<String>,
    mode: Option<String>,
    map: Option<String>,
}