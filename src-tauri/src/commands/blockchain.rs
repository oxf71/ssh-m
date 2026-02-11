use crate::blockchain::types::{default_chain_configs, BalanceReport, ChainConfig};
use crate::blockchain::{bitcoin as btc, evm, solana as sol};
use crate::crypto::evm as crypto_evm;
use crate::crypto::solana as crypto_sol;
use crate::crypto::bitcoin as crypto_btc;
use crate::crypto::types::{mnemonic_to_seed, MultiChainAccounts};
use crate::onepassword::cli as op_cli;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddressQuery {
    pub address: String,
    pub chain_type: String,
}

#[tauri::command]
pub async fn derive_accounts(
    vault: String,
    item: String,
    field: String,
    chains: Vec<String>,
    count: u32,
) -> Result<MultiChainAccounts, String> {
    // Read mnemonic from 1Password
    let mnemonic = op_cli::read_item_field(&vault, &item, &field)?;

    // Parse mnemonic and generate seed
    let seed = mnemonic_to_seed(&mnemonic)?;

    let mut evm_accounts = Vec::new();
    let mut solana_accounts = Vec::new();
    let mut bitcoin_accounts = Vec::new();

    for chain in &chains {
        match chain.as_str() {
            "evm" => {
                evm_accounts = crypto_evm::derive_evm_accounts(&seed, count)?;
            }
            "solana" => {
                solana_accounts = crypto_sol::derive_solana_accounts(&seed, count)?;
            }
            "bitcoin" => {
                bitcoin_accounts = crypto_btc::derive_bitcoin_accounts(&seed, count)?;
            }
            _ => {
                return Err(format!("Unsupported chain type: {}", chain));
            }
        }
    }

    // Seed is automatically zeroized when dropped here

    Ok(MultiChainAccounts {
        evm: evm_accounts,
        solana: solana_accounts,
        bitcoin: bitcoin_accounts,
        source_vault: vault,
        source_item: item,
    })
}

#[tauri::command]
pub async fn query_balances(
    addresses: Vec<AddressQuery>,
    chains_config: Vec<ChainConfig>,
) -> Result<BalanceReport, String> {
    let mut accounts = Vec::new();
    let defaults = default_chain_configs();

    for addr_query in &addresses {
        let config = chains_config
            .iter()
            .find(|c| c.chain_type == addr_query.chain_type)
            .or_else(|| {
                defaults
                    .iter()
                    .find(|c| c.chain_type == addr_query.chain_type)
            });

        if let Some(config) = config {
            let result = match addr_query.chain_type.as_str() {
                "evm" => evm::query_evm_balance(&addr_query.address, &config).await,
                "solana" => sol::query_solana_balance(&addr_query.address, &config).await,
                "bitcoin" => btc::query_bitcoin_balance(&addr_query.address, &config).await,
                _ => continue,
            };

            match result {
                Ok(balance) => accounts.push(balance),
                Err(e) => {
                    eprintln!(
                        "Failed to query balance for {} on {}: {}",
                        addr_query.address, addr_query.chain_type, e
                    );
                }
            }
        }
    }

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(BalanceReport {
        accounts,
        total_usd_value: None,
        timestamp,
    })
}

#[tauri::command]
pub fn get_default_chain_configs() -> Vec<ChainConfig> {
    default_chain_configs()
}
