pub(crate) mod throttler;
#[cfg(feature = "reply")]
pub mod reply;
pub(crate) mod request;
#[macro_use]
pub(crate) mod macros;
pub mod error;
#[cfg(feature = "util")]
pub mod util;
mod tests;

use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MonthlyPackageRank {
    None,
    Superstar,
}
display_enum_with_case!(MonthlyPackageRank, Upper);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
#[serde(rename_all = "UPPERCASE", from="String")]
pub enum StaffLevel {
    Normal,
    Helper,
    Moderator,
    Admin,
    Unknown(String)
}

impl From<String> for StaffLevel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "NORMAL" => StaffLevel::Normal,
            "HELPER" => StaffLevel::Helper,
            "MODERATOR" => StaffLevel::Moderator,
            "ADMIN" => StaffLevel::Admin,
            _ => StaffLevel::Unknown(s)
        }
    }
}

impl Display for StaffLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StaffLevel::Normal => write!(f, "NORMAL"),
            StaffLevel::Helper => write!(f, "HELPER"),
            StaffLevel::Moderator => write!(f, "MODERATOR"),
            StaffLevel::Admin => write!(f, "ADMIN"),
            StaffLevel::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageRank {
    None,
    Vip,
    VipPlus,
    Mvp,
    MvpPlus,
    MvpPlusPlus,
}
display_enum_with_case!(PackageRank, ScreamingSnake);

/// This corresponds to the table on [this wiki](https://minecraft.fandom.com/wiki/Formatting_codes#Color_codes).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColorCodes {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}
display_enum_with_case!(ColorCodes, ScreamingSnake);