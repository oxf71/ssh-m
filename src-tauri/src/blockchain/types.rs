use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    pub chain_id: Option<u64>,
    pub chain_type: String,
    pub name: String,
    pub rpc_url: String,
    pub symbol: String,
    pub explorer_url: String,
    pub tokens: Vec<TokenConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub symbol: String,
    pub name: String,
    pub contract_address: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenBalance {
    pub symbol: String,
    pub name: String,
    pub balance: String,
    pub balance_formatted: String,
    pub decimals: u8,
    pub contract_address: Option<String>,
    pub usd_value: Option<String>,
    pub is_native: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountBalance {
    pub address: String,
    pub chain: String,
    pub chain_name: String,
    pub chain_id: Option<u64>,
    pub native_balance: TokenBalance,
    pub token_balances: Vec<TokenBalance>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceReport {
    pub accounts: Vec<AccountBalance>,
    pub total_usd_value: Option<String>,
    pub timestamp: u64,
}

/// Get default chain configurations
pub fn default_chain_configs() -> Vec<ChainConfig> {
    vec![
        ChainConfig {
            chain_id: Some(1),
            chain_type: "evm".to_string(),
            name: "Ethereum".to_string(),
            rpc_url: "https://eth.llamarpc.com".to_string(),
            symbol: "ETH".to_string(),
            explorer_url: "https://etherscan.io".to_string(),
            tokens: vec![
                TokenConfig {
                    symbol: "USDT".to_string(),
                    name: "Tether USD".to_string(),
                    contract_address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
                    decimals: 6,
                },
                TokenConfig {
                    symbol: "USDC".to_string(),
                    name: "USD Coin".to_string(),
                    contract_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                    decimals: 6,
                },
            ],
        },
        ChainConfig {
            chain_id: Some(137),
            chain_type: "evm".to_string(),
            name: "Polygon".to_string(),
            rpc_url: "https://polygon.llamarpc.com".to_string(),
            symbol: "MATIC".to_string(),
            explorer_url: "https://polygonscan.com".to_string(),
            tokens: vec![],
        },
        ChainConfig {
            chain_id: Some(56),
            chain_type: "evm".to_string(),
            name: "BSC".to_string(),
            rpc_url: "https://bsc-dataseed.binance.org".to_string(),
            symbol: "BNB".to_string(),
            explorer_url: "https://bscscan.com".to_string(),
            tokens: vec![],
        },
        ChainConfig {
            chain_id: Some(42161),
            chain_type: "evm".to_string(),
            name: "Arbitrum".to_string(),
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            symbol: "ETH".to_string(),
            explorer_url: "https://arbiscan.io".to_string(),
            tokens: vec![],
        },
        ChainConfig {
            chain_id: None,
            chain_type: "solana".to_string(),
            name: "Solana".to_string(),
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            symbol: "SOL".to_string(),
            explorer_url: "https://solscan.io".to_string(),
            tokens: vec![],
        },
    ]
}
