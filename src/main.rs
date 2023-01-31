#![allow(unused)] // TODO: remove for release

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::TryFutureExt;
use std::str::FromStr;

use crate::prelude::*;
use ethers::prelude::*;

mod alchemy;
mod error;
mod listener;
mod prelude;
mod storage;
mod utils;

#[get("/logs/{event_name}")]
async fn get_all_logs_for_event(
    storage: web::Data<storage::SqliteStorage>,
    event_name: web::Path<String>,
) -> impl Responder {
    let name = event_name.into_inner();
    let logs = storage.get_all_logs_for_event(&name).unwrap();
    HttpResponse::Ok().body(logs)
}

#[post("/logs/latest/{event_name}")]
async fn get_latest_log_for_event(
    storage: web::Data<storage::SqliteStorage>,
    event_name: web::Path<String>,
) -> impl Responder {
    let name = event_name.into_inner();
    let logs = storage.get_latest_log_for_event(&name).unwrap();
    HttpResponse::Ok().body("yo")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let url = f!(
        "wss://eth-mainnet.g.alchemy.com/v2/{}",
        std::env::var("ALCHEMY_API_KEY")?
    );

    let storage_path = "storage.db";
    let provider = alchemy::AlchemyWebSocketProvider::new(&url).await;
    let storage = storage::SqliteStorage::init(Some(storage_path));

    let usd_coin_address = Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap();

    // Transfer event signature
    let topics =
        vec!["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()];

    let stream = provider
        .alchemy_subscribe_logs(usd_coin_address, Some(topics))
        .await?;

    let mut stream_listener =
        listener::StreamListener::new("usdc_transfers", stream, storage.clone());

    let listener_handle = stream_listener.listen();

    println!("Starting web server...");
    let web_server_handle = HttpServer::new(|| {
        let api_storage = storage::SqliteStorage::init(Some(storage_path));

        App::new()
            .app_data(web::Data::new(api_storage))
            .service(get_all_logs_for_event)
            .service(get_latest_log_for_event)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .map_err(|e| e.into());

    futures::try_join!(listener_handle, web_server_handle)?;

    Ok(())
}
