use anyhow::Result;

use std::sync::Arc;
use ethers::{
    providers::Provider, 
    prelude::*
};

use warp::{reply::Json, Rejection};

use crate::{
    errors::{GetBalanceError, IncorrectChainIdError, TransferERC20Error},
    simulation::{helpers::handle_distribute, types::SimulationRequestDisperse},
    utils::config::Config
};

pub async fn do_disperse(request: SimulationRequestDisperse, config: Config) -> Result<Json, Rejection> {
    let client = Provider::<Http>::try_from(config.fork_url.unwrap())
        .map_err(|_| warp::reject::custom(IncorrectChainIdError()))?;

    let private_key_env_var = std::env::var(&request.sender_private_key)
        .expect("Environment variable not found");
    let wallet: LocalWallet = private_key_env_var.parse().expect("Invalid private key");
    let wallet = wallet.with_chain_id(1u64);
    let provider = Arc::new(SignerMiddleware::new(client, wallet));

    let contract_address: Address = config.contract_address.unwrap().parse().unwrap();

    let shares_permissive_uint: Vec<U256> = request.shares.iter().map(|x| U256::from(*x)).collect();

    if let Some(token) = request.token {
        abigen!(IERC20, "contract/out/IERC20.sol/IERC20.json");
        let erc20_contract = IERC20::new(token, provider.clone());

        let balance = erc20_contract.balance_of(contract_address).call().await
            .map_err(|_| warp::reject::custom(GetBalanceError()))?;

        if balance < U256::from(request.amount_to_distribute) {
            erc20_contract.transfer(contract_address, U256::from(request.amount_to_distribute)).send().await
                .map_err(|_| warp::reject::custom(TransferERC20Error()))?;
        }

        let result = handle_distribute(
            provider,
            contract_address,
            false,
            token,
            request,
            shares_permissive_uint
        ).await?;

        return Ok(warp::reply::json(&result));
    } else {
        let result = handle_distribute(
            provider,
            contract_address,
            true,
            H160::zero(),
            request,
            shares_permissive_uint
        ).await?;

        return Ok(warp::reply::json(&result));
    }
}
