//! This module provides some examples of
//! data structures that link to repsonses from
//! Hypixel's Public API.

use chrono::{DateTime, Local, TimeZone};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use crate::api::{ColorCodes, MonthlyPackageRank, PackageRank, StaffLevel};
use crate::util::leveling;

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerReply {
    success: bool,
    player: Option<PlayerData>,
}

impl PlayerReply {
    /// Returns the data associated with the requested player.
    ///
    /// If this function returns [`Option::None`], the player isn't linked
    /// to any stats on hypixel. (And thus can be a nick)
    pub fn player(&self) -> Option<&PlayerData> {
        self.player.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerData {
    uuid: Uuid,
    #[serde(rename = "displayname")]
    display_name: Option<String>,
    #[serde(rename = "knownAliases")]
    known_aliases: Option<Vec<String>>,
    #[serde(rename = "playername")]
    player_name: Option<String>,
    #[serde(rename = "username")]
    user_name: Option<String>,
    #[serde(rename = "rank")]
    staff_level: Option<StaffLevel>,
    #[serde(rename = "packageRank")]
    package_rank: Option<PackageRank>,
    #[serde(rename = "newPackageRank")]
    new_package_rank: Option<PackageRank>,
    #[serde(rename = "monthlyPackageRank")]
    is_plus_plus: Option<MonthlyPackageRank>,
    #[serde(rename = "rankPlusColor")]
    rank_plus_color: Option<ColorCodes>,
    #[serde(rename = "monthlyRankColor")]
    superstar_tag_color: Option<ColorCodes>,
    #[serde(rename = "firstLogin")]
    first_login: Option<u64>,
    #[serde(rename = "lastLogin")]
    last_login: Option<u64>,
    #[serde(rename = "lastLogout")]
    last_logout: Option<u64>,
    #[serde(rename = "networkExp", default)]
    network_exp: f64,
    #[serde(rename = "networkLevel", default)]
    network_lvl: f64,
    #[serde(default)]
    karma: u64,
    stats: Option<HashMap<String, Value>>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

impl PlayerData {
    /// Returns the player's UUID.
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Returns the player's name.
    ///
    /// This follows the implementation in [`PlayerReply.java`](https://github.com/HypixelDev/PublicAPI/search?q=PlayerReply)
    /// in the official java implementation of the Hypixel Public API.
    ///
    /// In this Rust implementation, currently it does the following:\
    /// 1. if `"displayname"` is present, return it
    /// 2. else if `"knownAliases"` is present, return the last element of this list (most recent name)
    /// 3. else if `"playername"` is present, return it
    /// 4. else if `"username"` is present, return it
    /// 5. else no username could be returned
    pub fn name(&self) -> Option<&str> {
        if self.display_name.is_some() {
            return self.display_name.as_deref();
        }
        if let Some(aliases) = &self.known_aliases {
            if aliases.len() > 0 {
                return Some(&aliases[aliases.len() - 1]);
            }
        }
        if self.player_name.is_some() {
            return self.player_name.as_deref();
        }
        if self.user_name.is_some() {
            return self.user_name.as_deref();
        }
        None
    }

    /// Returns the total amount of network experience the player has earned.
    ///
    /// If no experience or level field was included, `0` will be used instead.
    pub fn network_xp(&self) -> u64 {
        (self.network_exp + leveling::network::total_xp_to_full_level(self.network_lvl + 1.0)) as u64
    }

    /// Returns the player's precise network level, including their progress to the next level.
    pub fn network_level(&self) -> f64 {
        // use direct values for precision
        let xp = self.network_exp + leveling::network::total_xp_to_full_level(self.network_lvl + 1.0);
        leveling::network::exact_level(xp)
    }

    /// Returns the total amount of karma points earned by the player.
    ///
    /// If this field is not present, 0 is returned.
    pub fn karma(&self) -> u64 {
        self.karma
    }

    /// Returns the date when the player first connected to Hypixel.
    pub fn first_login(&self) -> Option<DateTime<Local>> {
        self.first_login.map(|v| Local.timestamp_millis(v as i64))
    }

    /// Returns the last known time when the player connected to the main Hypixel network.
    pub fn last_login(&self) -> Option<DateTime<Local>> {
        self.last_login.map(|v| Local.timestamp_millis(v as i64))
    }

    /// Returns the last known time when the player disconnected from the main Hypixel network.
    pub fn last_logout(&self) -> Option<DateTime<Local>> {
        self.last_logout.map(|v| Local.timestamp_millis(v as i64))
    }

    /// Returns the color of the player's `"+"`s if they have `MVP+` or `MVP++`.
    ///
    /// If they do not have either rank, or if they have not selected a color, `"RED"` is returned as the default.
    pub fn selected_plus_color(&self) -> ColorCodes {
        self.rank_plus_color.unwrap_or(ColorCodes::Red)
    }

    /// Returns the color of the player's name tag if they have `MVP++`.
    ///
    /// Defaults to [`ColorCodes::Gold`].
    pub fn superstar_tag_color(&self) -> ColorCodes {
        self.superstar_tag_color.unwrap_or(ColorCodes::Gold)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusReply {
    success: bool,
    #[serde(flatten)]
    data: StatusData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusData {
    uuid: Uuid,
    session: SessionData,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct KeyReply {
    success: bool,
    record: KeyData,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    queries_in_past_min: i32,
    key: Uuid,
    owner: Uuid,
    limit: i32,
    total_queries: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionData {
    online: bool,
    /// TODO: chage into enum for easier game sorting
    #[serde(rename = "gameType")]
    game_type: Option<String>,
    mode: Option<String>,
    map: Option<String>,
}
