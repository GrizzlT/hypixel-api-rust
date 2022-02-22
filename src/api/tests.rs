#![cfg(test)]

use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use uuid::Uuid;
use crate::api::reply::{PlayerData, StatusData};
use crate::{KeyReply, PlayerReply, RequestHandler};

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

#[test]
#[ignore]
fn test_bulk() {
    tokio::runtime::Runtime::new().unwrap()
        .block_on(async move {
            let request_handler = RequestHandler::new(Uuid::from_str(env!("HYPIXEL_KEY")).unwrap());

            // status?uuid=ec174daf-b5a5-4ea1-adc6-35a7f9fc4a60
            let mut future_pool = FuturesUnordered::new();
            for i in 0..300 {
                let future = request_handler.request::<KeyReply>("key");
                future_pool.push(async move {
                    (i, future.await)
                });
                tokio::time::sleep(Duration::from_millis(3)).await;
            }

            while let Some((i, reply)) = future_pool.next().await {
                println!("Player data #{}: {:?}", i, reply);
            }
        });
}

#[test]
fn test_bed_bed() {
    tokio::runtime::Runtime::new().unwrap()
        .block_on(async move {
            let request_handler = RequestHandler::new(Uuid::from_str(env!("HYPIXEL_KEY")).unwrap());

            let reply = request_handler.request::<PlayerReply>("player?uuid=232b2c37-4d68-4086-a7ce-67d0cadcc7f9").await.unwrap();
            match reply {
                Ok(reply) => println!("Response: {:?}", reply),
                Err(error) => println!("Encoutered error: {}, source: {:?}", error, error.source()),
            }
        })
}