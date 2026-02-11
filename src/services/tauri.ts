import { invoke } from "@tauri-apps/api/core";
import type { SshHost } from "../types/ssh";
import type { Vault, VaultItem, OpStatus } from "../types/onepassword";
import type { MultiChainAccounts, BalanceReport, ChainConfig } from "../types/blockchain";

// ============ SSH Commands ============

export async function listSshHosts(): Promise<SshHost[]> {
  return invoke("list_ssh_hosts");
}

export async function openSshTerminal(host: string, terminal?: string): Promise<void> {
  return invoke("open_ssh_terminal", { host, terminal });
}

export async function refreshSshConfig(): Promise<SshHost[]> {
  return invoke("refresh_ssh_config");
}

export interface SshConfigFile {
  name: string;
  path: string;
  is_main: boolean;
  host_count: number;
}

export async function listSshConfigFiles(): Promise<SshConfigFile[]> {
  return invoke("list_ssh_config_files");
}

export async function readSshConfig(path?: string): Promise<string> {
  return invoke("read_ssh_config", { path });
}

export async function validateSshConfig(content: string): Promise<string[]> {
  return invoke("validate_ssh_config", { content });
}

export async function saveSshConfig(content: string, path?: string): Promise<string[]> {
  return invoke("save_ssh_config", { content, path });
}

// ============ 1Password Commands ============

export async function checkOpStatus(): Promise<OpStatus> {
  return invoke("check_op_status");
}

export async function listVaults(): Promise<Vault[]> {
  return invoke("list_vaults");
}

export async function listVaultItems(vaultId: string): Promise<VaultItem[]> {
  return invoke("list_vault_items", { vaultId });
}

// ============ Blockchain Commands ============

export async function deriveAccounts(
  vault: string,
  item: string,
  field: string,
  chains: string[],
  count: number,
): Promise<MultiChainAccounts> {
  return invoke("derive_accounts", { vault, item, field, chains, count });
}

export async function queryBalances(
  addresses: { address: string; chain_type: string }[],
  chainsConfig: ChainConfig[],
): Promise<BalanceReport> {
  return invoke("query_balances", { addresses, chainsConfig });
}

export async function getDefaultChainConfigs(): Promise<ChainConfig[]> {
  return invoke("get_default_chain_configs");
}
