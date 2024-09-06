// src/lib.rs

use warp::{Filter, Rejection, Reply};
use serde::de::DeserializeOwned;

pub mod errors;
pub mod utils;
pub mod simulation;

use crate::{
    utils::config::Config,
    simulation::{
        types::{SimulationRequestCollect, SimulationRequestDisperse},
        distributor::do_disperse,
        collector::do_collect
    }
};

pub fn simulate_routes(
    config: Config
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    disperse(config.clone())
        .or(collect(config.clone()))
}

/// POST /disperse
pub fn disperse(config: Config) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("disperse")
        .and(warp::post())
        .and(json_body::<SimulationRequestDisperse>())
        .and(with_config(config))
        .and_then(do_disperse)
}

/// POST /collect
pub fn collect(config: Config) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("collect")
        .and(warp::post())
        .and(json_body::<SimulationRequestCollect>())
        .and(with_config(config))
        .and_then(do_collect)
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
{
    warp::body::content_length_limit(8192 * 16).and(warp::body::json())
}
