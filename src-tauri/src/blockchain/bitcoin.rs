use reqwest;
use serde::Deserialize;

use super::types::{AccountBalance, ChainConfig, TokenBalance};

#[derive(Deserialize)]
struct BlockstreamAddress {
    #[serde(default)]
    chain_stats: BlockstreamStats,
    #[serde(default)]
    mempool_stats: BlockstreamStats,
}

#[derive(Deserialize, Default)]
struct BlockstreamStats {
    #[serde(default)]
    funded_txo_sum: u64,
    #[serde(default)]
    spent_txo_sum: u64,
}

/// Query BTC balance via Blockstream API
pub async fn query_bitcoin_balance(
    address: &str,
    _config: &ChainConfig,
) -> Result<AccountBalance, String> {
    let url = format!("https://blockstream.info/api/address/{}", address);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch BTC balance: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Blockstream API error: {} for address {}",
            resp.status(),
            address
        ));
    }

    let data: BlockstreamAddress = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse BTC balance response: {}", e))?;

    let confirmed_balance =
        data.chain_stats.funded_txo_sum - data.chain_stats.spent_txo_sum;
    let unconfirmed_balance =
        data.mempool_stats.funded_txo_sum - data.mempool_stats.spent_txo_sum;
    let total_satoshis = confirmed_balance + unconfirmed_balance;

    let btc_formatted = format!("{:.8}", total_satoshis as f64 / 100_000_000.0);

    Ok(AccountBalance {
        address: address.to_string(),
        chain: "bitcoin".to_string(),
        chain_name: "Bitcoin".to_string(),
        chain_id: None,
        native_balance: TokenBalance {
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            balance: total_satoshis.to_string(),
            balance_formatted: btc_formatted,
            decimals: 8,
            contract_address: None,
            usd_value: None,
            is_native: true,
        },
        token_balances: vec![],
    })
}
