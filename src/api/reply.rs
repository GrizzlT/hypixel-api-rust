//! This module provides some examples of
//! data structures that link to repsonses from
//! Hypixel's Public API.

use chrono::serde::ts_milliseconds_option::deserialize as from_milli_ts;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PlayerReply {
    success: bool,
    player: PlayerData,
}

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    uuid: Uuid,
    #[serde(rename = "displayname")]
    display_name: Option<String>,
    #[serde(rename = "rank")]
    staff_level: Option<StaffLevel>,
    #[serde(rename = "packageRank")]
    package_rank: Option<PackageRank>,
    #[serde(rename = "newPackageRank")]
    new_package_rank: Option<PackageRank>,
    #[serde(rename = "monthlyPackageRank")]
    is_plus_plus: Option<MonthlyPackageRank>,
    #[serde(rename = "firstLogin", deserialize_with = "from_milli_ts")]
    first_login: Option<DateTime<Utc>>,
    #[serde(rename = "lastLogin", deserialize_with = "from_milli_ts")]
    last_login: Option<DateTime<Utc>>,
    #[serde(rename = "lastLogout", deserialize_with = "from_milli_ts")]
    last_logout: Option<DateTime<Utc>>,
    stats: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
pub struct StatusReply {
    success: bool,
    #[serde(flatten)]
    data: StatusData,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    uuid: Uuid,
    session: SessionData,
}

#[derive(Debug, Deserialize)]
pub struct KeyReply {
    success: bool,
    record: KeyData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    queries_in_past_min: i32,
    key: Uuid,
    owner: Uuid,
    limit: i32,
    total_queries: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum MonthlyPackageRank {
    Superstar,
    None,
}

#[derive(Debug, Deserialize)]
pub struct SessionData {
    online: bool,
    /// TODO: chage into enum for easier game sorting
    #[serde(rename = "gameType")]
    game_type: Option<String>,
    mode: Option<String>,
    map: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum StaffLevel {
    Admin,
    Moderator,
    Helper,
    Normal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageRank {
    None,
    Vip,
    VipPlus,
    Mvp,
    MvpPlus,
    MvpPlusPlus,
}
