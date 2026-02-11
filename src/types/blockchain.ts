// Blockchain Types

export type ChainType = "evm" | "solana" | "bitcoin";

export interface Account {
  address: string;
  chain: ChainType;
  chain_name: string;
  derivation_path: string;
  index: number;
}

export interface MultiChainAccounts {
  evm: Account[];
  solana: Account[];
  bitcoin: Account[];
  source_vault: string;
  source_item: string;
}

export interface TokenBalance {
  symbol: string;
  name: string;
  balance: string;
  balance_formatted: string;
  decimals: number;
  contract_address: string | null;
  usd_value: string | null;
  is_native: boolean;
}

export interface AccountBalance {
  address: string;
  chain: ChainType;
  chain_name: string;
  chain_id: number | null;
  native_balance: TokenBalance;
  token_balances: TokenBalance[];
}

export interface ChainConfig {
  chain_id: number | null;
  chain_type: ChainType;
  name: string;
  rpc_url: string;
  symbol: string;
  explorer_url: string;
  tokens: TokenConfig[];
}

export interface TokenConfig {
  symbol: string;
  name: string;
  contract_address: string;
  decimals: number;
}

export interface BalanceReport {
  accounts: AccountBalance[];
  total_usd_value: string | null;
  timestamp: number;
}
