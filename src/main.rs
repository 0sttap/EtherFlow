// src/main.rs
use std::env;

use warp::Filter;
use ether_flow::{errors::handle_rejection, simulate_routes, utils::{config::config, logger}};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config(args.get(1).cloned());

    logger::init().expect("Logger init error");

    let api_base = warp::path("api");

    let api_key = config.clone().api_key;
    let api_port = config.port;

    let api_base = if let Some(api_key) = api_key {
        log::info!("Running with API key protection");
        let api_key_filter = warp::header::exact("X-API-KEY", Box::leak(api_key.into_boxed_str()));
        api_base.and(api_key_filter).boxed()
    } else {
        api_base.boxed()
    };

    let routes = api_base
        .and(simulate_routes(config))
        .recover(handle_rejection)
        .with(warp::log("ts::api"));
    
    log::info!(target: "ts::api", "Starting server on port: {api_port}");
    warp::serve(routes).run(([0, 0, 0, 0], api_port)).await;
}
