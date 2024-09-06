// src/errors.rs

use eyre::Report;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, error::Error};

use warp::{body::BodyDeserializeError, hyper::StatusCode, reject::Reject, Rejection, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
pub struct IncorrectChainIdError();

impl Reject for IncorrectChainIdError {}

#[derive(Debug)]
pub struct OverrideError;

impl Reject for OverrideError {}

#[derive(Debug)]
pub struct EvmError(pub Report);

impl Reject for EvmError {}

#[derive(Debug)]
pub struct ProviderError();

impl Reject for ProviderError {}

#[derive(Debug)]
pub struct DistributeERC20Error();

impl Reject for DistributeERC20Error {}

#[derive(Debug)]
pub struct DistributeETHError();

impl Reject for DistributeETHError {}

#[derive(Debug)]
pub struct GetTransactionHashError();

impl Reject for GetTransactionHashError {}

#[derive(Debug)]
pub struct GetBalanceError();

impl Reject for GetBalanceError {}

#[derive(Debug)]
pub struct TransferERC20Error();

impl Reject for TransferERC20Error {}

#[derive(Debug)]
pub struct SendETHError();

impl Reject for SendETHError {}

#[derive(Debug)]
pub struct CollectERC20Error();

impl Reject for CollectERC20Error {}

#[derive(Debug)]
pub struct CollectETHError();

impl Reject for CollectETHError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;
    println!("Handling rejection: {:?}", err);
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(_e) = err.find::<IncorrectChainIdError>() {
        code = StatusCode::BAD_REQUEST;
        message = "INCORRECT_CHAIN_ID".to_string();
    } else if let Some(_e) = err.find::<ProviderError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "PROVIDER_ERROR".to_string();
    } else if let Some(_e) = err.find::<DistributeERC20Error>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "DISTRIBUTE_ERC20_ERROR".to_string();
    } else if let Some(_e) = err.find::<DistributeETHError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "DISTRIBUTE_ETH_ERROR".to_string();
    } else if let Some(_e) = err.find::<GetTransactionHashError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "GET_TRANSACTION_HASH_ERROR".to_string();
    } else if let Some(_e) = err.find::<GetBalanceError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "GET_BALANCE_ERROR".to_string();
    } else if let Some(_e) = err.find::<TransferERC20Error>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "TRANSFER_ERC20_ERROR".to_string();
    } else if let Some(_e) = err.find::<SendETHError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "SEND_ETH_ERROR".to_string();
    } else if let Some(_e) = err.find::<CollectERC20Error>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "COLLECT_ERC20_ERROR".to_string();
    } else if let Some(_e) = err.find::<CollectETHError>() {
        dbg!(_e);
        code = StatusCode::BAD_REQUEST;
        message = "COLLECT_ETH_ERROR".to_string();
    } else if let Some(_e) = err.find::<OverrideError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "OVERRIDE_ERROR".to_string();
    } else if let Some(_e) = err.find::<EvmError>() {
        if _e.0.to_string().contains("CallGasCostMoreThanGasLimit") {
            code = StatusCode::BAD_REQUEST;
            message = "OUT_OF_GAS".to_string();
        } else {
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "EVM_ERROR".to_string();
        }
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        dbg!(e);
        message = match e.source() {
            Some(cause) => format!("BAD REQUEST: {cause}"),
            None => "BAD_REQUEST".to_string(),
        };
        code = StatusCode::BAD_REQUEST;
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_string();
    } else if err.find::<warp::reject::MissingHeader>().is_some() {
        code = StatusCode::UNAUTHORIZED;
        message = "UNAUTHORIZED".to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {err:?}");
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
