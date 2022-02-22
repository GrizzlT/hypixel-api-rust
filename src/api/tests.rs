#![cfg(test)]

use crate::api::reply::{PlayerData, StatusData};

#[test]
fn test_player() {
    let sample = r#"
        {
            "uuid": "3fa85f6457174562b3fc2c963f66afa6",
            "displayname": "string",
            "rank": "ADMIN",
            "packageRank": "MVP_PLUS",
            "newPackageRank": "MVP_PLUS",
            "monthlyPackageRank": "SUPERSTAR",
            "firstLogin": 0,
            "lastLogin": 0,
            "lastLogout": 0,
            "stats": { }
        }
    "#;

    let data: PlayerData = serde_json::from_str(sample).unwrap();
    print!("Sample data:\n {:?}", data);
}

#[test]
fn test_status() {
    let sample = r#"
        {
            "uuid": "ad8fefaa8351454bb739a4eaa872173f",
            "session": {
                "online": true,
                "gameType": "string",
                "mode": "string",
                "map": "string"
            }
        }
    "#;

    let data: StatusData = serde_json::from_str(sample).unwrap();
    print!("Sample data:\n {:?}", data);
}