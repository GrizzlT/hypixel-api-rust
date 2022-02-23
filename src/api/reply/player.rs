use chrono::{DateTime, Local, TimeZone};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use crate::api::{ColorCodes, MonthlyPackageRank, PackageRank, StaffLevel};
use crate::error::HypixelApiError;
use crate::util::leveling;

/// A data structure that maps to [`this endpoint`](https://api.hypixel.net/#tag/Player-Data).
///
/// Response fields are captured in [`PlayerData`].
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerReply {
    success: bool,
    player: Option<PlayerData>,
}

impl PlayerReply {
    /// Returns whether the response was successful.
    ///
    /// This should always return true. (not guaranteed though)
    pub fn success(&self) -> bool {
        self.success
    }

    /// Returns the data associated with the requested player.
    ///
    /// If this function returns [`Option::None`], the player isn't linked
    /// to any data on hypixel. (And thus can be a nick)
    pub fn player(&self) -> Option<&PlayerData> {
        self.player.as_ref()
    }
}

/// The response data corresponding to [`this endpoint`](https://api.hypixel.net/#tag/Player-Data).
///
/// ##### This struct implements some convenience functions to parse hypixel api data:
///
/// ### Player name
/// The player name is spread out over multiple fields, use [`PlayerData::name`]
/// to get a correct one (like the api does).
///
/// ### Network xp and lvl
/// `Hypixel Network XP` and `Level` are stored in two separate fields in a specific format.\
/// Use [`PlayerData::network_xp`] and [`PlayerData::network_level`] for these values.
///
/// ### Ranks
/// Ranks are spread out over 5 fields!\
/// Always use [`PlayerData::staff_level`] and [`PlayerData::package_rank`] to get
/// the correct precedence. See [`this FAQ`](https://github.com/HypixelDev/PublicAPI/wiki/Common-Questions#how-do-i-get-a-players-rank-prefix)
/// for more information.
///
/// ### Game stats
/// All game stats are captured generically. To get a specific one,
/// use [`PlayerData::stat_value`] or define a corresponding struct
/// and use [`PlayerData::stat_json`].
///
/// ### Other properties
/// You can get any property that the functions in this struct don't cover
/// by using [`PlayerData::property_value`] or defining a corresponding struct
/// and use [`PlayerData::property_json`].
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
    #[serde(rename = "buildTeam", default)]
    build_team: bool,
    #[serde(rename = "buildTeamAdmin", default)]
    build_team_admin: bool,
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

    /// Returns the special rank of players if present.
    ///
    /// Defaults to [`StaffLevel::Normal`].
    pub fn staff_level(&self) -> &StaffLevel {
        self.staff_level.as_ref().unwrap_or(&StaffLevel::Normal)
    }

    /// Returns the highest in precedence rank that the player has.
    /// See [`this FAQ`](https://github.com/HypixelDev/PublicAPI/wiki/Common-Questions#how-do-i-get-a-players-rank-prefix).
    ///
    /// This function only considers values in [`PackageRank`].
    pub fn package_rank(&self) -> PackageRank {
        if self.is_plus_plus.filter(|v| *v != MonthlyPackageRank::None).is_some() {
            PackageRank::MvpPlusPlus
        } else if let Some(rank) = self.new_package_rank.filter(|v| *v != PackageRank::None) {
            rank
        } else if let Some(rank) = self.package_rank.filter(|v| *v != PackageRank::None) {
            rank
        } else {
            PackageRank::None
        }
    }

    /// Returns true if the player has either a special [`StaffLevel`] rank
    /// or a [`PackageRank`] rank.
    ///
    /// Defaults to false.
    pub fn has_rank(&self) -> bool {
        *self.staff_level() != StaffLevel::Normal || self.package_rank() != PackageRank::None
    }

    /// Returns true if the player is part of the
    /// [Hypixel Build Team](https://twitter.com/hypixelbuilders)
    ///
    /// Defaults to false.
    pub fn on_build_team(&self) -> bool {
        self.build_team || self.build_team_admin
    }

    /// Returns the json entry corresponding to `name`, if present.
    ///
    /// See [`PlayerData::stat_json`] for a possibly more convenient function.
    pub fn stat_value(&self, name: &str) -> Option<&Value> {
        self.stats.as_ref().map(|m| m.get(name)).flatten()
    }

    /// Returns the json entry corresponding to `name`, if present,
    /// and automatically deserialized into `T`.
    ///
    /// # Note
    /// This function **clones** the data in order to deserialize it. In the future this
    /// could be updated to automatically deserialize stable games.
    pub fn stat_json<T: DeserializeOwned>(&self, name: &str) -> Option<Result<T, HypixelApiError>> {
        self.stats.as_ref().map(|m| m.get(name))
            .flatten()
            .map(|v| serde_json::from_value(v.clone()).map_err(|e| e.into()))
    }

    /// Returns any other property this struct does not capture
    /// explicitly already, if present.
    ///
    /// See [`PlayerData::property_json`] for a possibly more convenient function.
    pub fn property_value(&self, name: &str) -> Option<&Value> {
        self.other.get(name)
    }

    /// Returns any other property this struct does not capture
    /// explicitly already, if present, and automatically deserializes
    /// it into `T`.
    /// # Note
    /// This function **clones** the data in order to deserialize it. For maximum efficiency,
    /// always consider contributing stable fields to the repository, thank you!
    pub fn property_json<T: DeserializeOwned>(&self, name: &str) -> Option<Result<T, HypixelApiError>> {
        self.other.get(name)
            .map(|v| serde_json::from_value(v.clone()).map_err(|e| e.into()))
    }
}