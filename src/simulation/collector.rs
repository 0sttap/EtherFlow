use anyhow::Result;
use tokio::task;
use std::sync::Arc;
use ethers::{
    providers::Provider, 
    prelude::*,
};
use warp::{reply::Json, Rejection};
use crate::{
    errors::{IncorrectChainIdError, ProviderError},
    simulation::types::SimulationRequestCollect,
    utils::config::Config
};

use futures::future::join_all;

use super::helpers::{handle_sender, handle_withdraw};

pub async fn do_collect(request: SimulationRequestCollect, config: Config) -> Result<Json, Rejection> {
    let client = Provider::<Http>::try_from(config.fork_url.unwrap())
        .map_err(|_err| warp::reject::custom(IncorrectChainIdError()))?;

    let contract_address: Address = config.contract_address.unwrap().parse().unwrap();

    let private_key_env_var = std::env::var(&request.owner_private_key)
        .expect("Environment variable not found");
    let wallet: LocalWallet = private_key_env_var.parse().expect("Invalid private key");
    let wallet = wallet.with_chain_id(1u64);

    let provider = Arc::new(SignerMiddleware::new(client, wallet.clone()));

    let mut tasks = vec![];

    for sender_data in request.senders_with_shares {
        let provider = provider.clone();
        let contract_address = contract_address.clone();
        let token = request.token.clone();
    
        let task = task::spawn(handle_sender(provider, contract_address, token, sender_data));
        tasks.push(task);
    }

    // Wait for all tasks to complete
    let results = join_all(tasks).await;

    // Check if all tasks were successful
    if results.iter().all(|res| res.is_ok()) {
        let result = handle_withdraw(
            provider,
            contract_address,
            request.token,
            request.receiver,
            request.withdraw_amount,
            request.fixed_type
        ).await?;

        Ok(warp::reply::json(&result))
    } else {
        eprintln!("One or more transfers failed.");
        Err(warp::reject::custom(ProviderError()))
    }
}
