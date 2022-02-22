use std::str::FromStr;
use std::time::Duration;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use uuid::Uuid;
use crate::api::reply::{KeyReply};
use crate::api::request::RequestHandler;

pub mod api;

#[tokio::main]
async fn main() {
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
}
