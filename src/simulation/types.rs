use ethers::{
    abi::{Address, Uint},
    types::H256
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationRequestCollect {
    pub token: Option<Address>, // only for ERC20
    pub owner_private_key: String,
    pub senders_with_shares: Vec<SenderData>,
    pub receiver: Address,
    pub withdraw_amount: PermissiveUint,
    pub fixed_type: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SenderData {
    pub private_key: String,
    pub share: PermissiveUint
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationRequestDisperse {
    pub token: Option<Address>, // only for ERC20
    pub sender_private_key: String,
    pub amount_to_distribute: PermissiveUint,
    pub receivers: Vec<Address>,
    pub shares: Vec<PermissiveUint>,
    pub fixed_type: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub transaction_hash: H256
}

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq)]
#[serde(transparent)]
pub struct PermissiveUint(pub Uint);

impl From<PermissiveUint> for Uint {
    fn from(value: PermissiveUint) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for PermissiveUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Accept value in hex or decimal formats
        let value = String::deserialize(deserializer)?;
        let parsed = if value.starts_with("0x") {
            Uint::from_str(&value).map_err(serde::de::Error::custom)?
        } else {
            Uint::from_dec_str(&value).map_err(serde::de::Error::custom)?
        };
        Ok(Self(parsed))
    }
}
