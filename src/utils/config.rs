// src/config.rs
use dotenv::{from_filename, dotenv};

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub fork_url: Option<String>,
    pub api_key: Option<String>,
    pub contract_address: Option<String>,
}

pub fn config(env: Option<String>) -> Config {
    if env.is_none() {
        dotenv().ok();
    } else {
        from_filename(env.unwrap()).ok();
    }

    load_config()
}

fn load_config() -> Config {
    dotenv().expect("DotEnv init error");

    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT must be valid u16.");

    let fork_url = match std::env::var("RPC_URL") {
        Ok(f) => {
            if f.is_empty() {
                panic!("RPC_URL must not be empty.");
            } else {
                f
            }
        },
        Err(_) => panic!("RPC_URL must exist.")
    };

    let contract_address = match std::env::var("CONTRACT_ADDRESS") {
        Ok(f) => {
            if f.is_empty() {
                panic!("CONTRACT_ADDRESS must not be empty.");
            } else {
                f
            }
        },
        Err(_) => panic!("CONTRACT_ADDRESS must exist.")
    };

    let api_key = std::env::var("API_KEY").ok().filter(|k| !k.is_empty());

    Config {
        port,
        fork_url: Some(fork_url),
        api_key,
        contract_address: Some(contract_address),
    }
}
