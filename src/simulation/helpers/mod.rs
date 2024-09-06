use anyhow::Result;

use std::sync::Arc;
use ethers::{
    providers::Provider, 
    prelude::*
};

use warp::Rejection;

use crate::{
    errors::{CollectERC20Error, CollectETHError, DistributeERC20Error, DistributeETHError, GetTransactionHashError, SendETHError, TransferERC20Error},
    simulation::types::{Response, SimulationRequestDisperse},
};

use super::types::{PermissiveUint, SenderData};

pub async fn handle_distribute(
    provider: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    is_eth: bool,
    token: Address,
    request: SimulationRequestDisperse,
    shares_permissive_uint: Vec<U256>,
) -> Result<Response, Rejection> {
    abigen!(FLOW, "contract/out/Flow.sol/Flow.json");
    let contract = FLOW::new(contract_address, provider.clone());

    if is_eth {
        let distribute_eth = contract.distribute_eth(
            request.receivers,
            shares_permissive_uint,
            request.fixed_type
        ).value(request.amount_to_distribute);

        let pending_tx = distribute_eth.send().await.map_err(|_| warp::reject::custom(DistributeETHError()))?;

        let receipt = pending_tx.await
            .map_err(|_| warp::reject::custom(GetTransactionHashError()))?
            .ok_or_else(|| warp::reject::custom(GetTransactionHashError()))?;

        Ok(Response {
            transaction_hash: receipt.transaction_hash,
        })
    } else {
        let distribute_erc20 = contract.distribute_erc20(
            token,
            U256::from(request.amount_to_distribute),
            request.receivers,
            shares_permissive_uint,
            request.fixed_type
        );

        let pending_tx = distribute_erc20.send().await.map_err(|_| warp::reject::custom(DistributeERC20Error()))?;
        
        let receipt = pending_tx.await
            .map_err(|_| warp::reject::custom(GetTransactionHashError()))?
            .ok_or_else(|| warp::reject::custom(GetTransactionHashError()))?;

        Ok(Response {
            transaction_hash: receipt.transaction_hash,
        })
    }
}

pub async fn handle_sender(
    provider: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    token: Option<Address>,
    sender_data: SenderData,
) -> Result<(), Rejection> {
    let share = sender_data.share;

    let private_key_env_var = std::env::var(sender_data.private_key)
        .expect("Environment variable not found");
    let sender: LocalWallet = private_key_env_var.parse().expect("Invalid private key");
    let sender = sender.with_chain_id(1u64);

    if let Some(token) = token {
        abigen!(IERC20, "contract/out/IERC20.sol/IERC20.json");
        let erc20_contract = IERC20::new(token, provider.clone());

        erc20_contract
            .transfer(contract_address, U256::from(share))
            .from(sender.address())
            .send()
            .await
            .map_err(|_err| warp::reject::custom(TransferERC20Error()))?;
    } else {
        provider
            .send_transaction(
                TransactionRequest::new()
                    .to(contract_address)
                    .value(share)
                    .from(sender.address()),
                None,
            )
            .await
            .map_err(|_err| warp::reject::custom(SendETHError()))?;
    }

    Ok(())
}

pub async fn handle_withdraw(
    provider: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: Address,
    token: Option<Address>,
    receiver: Address,
    withdraw_amount: PermissiveUint,
    fixed_type: bool,
) -> Result<Response, Rejection> {
    abigen!(FLOW, "contract/out/Flow.sol/Flow.json");
    let contract = FLOW::new(contract_address, provider.clone());

    if let Some(token) = token {
        let withdraw_erc20 = contract.withdraw_erc20(
            token,
            receiver,
            U256::from(withdraw_amount),
            fixed_type
        );

        let pending_tx = withdraw_erc20.send().await.map_err(|_| warp::reject::custom(CollectERC20Error()))?;
        
        let receipt = pending_tx.await
            .map_err(|_| warp::reject::custom(GetTransactionHashError()))?
            .ok_or_else(|| warp::reject::custom(GetTransactionHashError()))?;

        Ok(Response {
            transaction_hash: receipt.transaction_hash,
        })
    } else {
        let withdraw_eth = contract.withdraw_eth(
            receiver,
            U256::from(withdraw_amount),
            fixed_type
        );

        let pending_tx = withdraw_eth.send().await.map_err(|_| warp::reject::custom(CollectETHError()))?;
        
        let receipt = pending_tx.await
            .map_err(|_| warp::reject::custom(GetTransactionHashError()))?
            .ok_or_else(|| warp::reject::custom(GetTransactionHashError()))?;

        Ok(Response {
            transaction_hash: receipt.transaction_hash,
        })
    }
}
