import { useState } from "react";
import { Coins, Key, RefreshCw, ChevronRight, AlertCircle } from "lucide-react";
import { clsx } from "clsx";
import { useOpStatus, useVaults, useVaultItems } from "../hooks/useOnePassword";
import { useDeriveAccounts, useQueryBalances, useDefaultChainConfigs } from "../hooks/useBlockchain";
import { AccountCard } from "../components/Blockchain";
import type { Vault, VaultItem, MultiChainAccounts } from "../types";

export function BlockchainAccounts() {
  const { data: opStatus, isLoading: opLoading } = useOpStatus();
  const [opConnected, setOpConnected] = useState(false);
  const { data: vaults, isError: vaultsError, refetch: refetchVaults } = useVaults(opConnected);
  const [selectedVault, setSelectedVault] = useState<string | null>(null);
  const { data: vaultItems } = useVaultItems(selectedVault);
  const [selectedItem, setSelectedItem] = useState<VaultItem | null>(null);
  const [accounts, setAccounts] = useState<MultiChainAccounts | null>(null);

  const deriveMutation = useDeriveAccounts();
  const { data: chainConfigs } = useDefaultChainConfigs();
  const { data: balanceReport, isLoading: balancesLoading } = useQueryBalances(
    accounts,
    chainConfigs,
  );

  const handleDerive = async () => {
    if (!selectedVault || !selectedItem) return;
    const result = await deriveMutation.mutateAsync({
      vault: selectedVault,
      item: selectedItem.id,
      field: "mnemonic",
      chains: ["evm", "solana", "bitcoin"],
      count: 3,
    });
    setAccounts(result);
  };

  // Combine all accounts for display
  const allAccounts = accounts
    ? [...accounts.evm, ...accounts.solana, ...accounts.bitcoin]
    : [];

  return (
    <div className="p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-xl font-bold flex items-center gap-2">
            <Coins className="w-5 h-5 text-primary" />
            区块链账户
          </h1>
          <p className="text-sm text-text-dim mt-1">
            通过 1Password 助记词派生多链账户，查询余额
          </p>
        </div>
      </div>

      {/* 1Password Status */}
      {opLoading ? (
        <div className="bg-surface-light border border-border rounded-xl p-4 mb-6 animate-pulse">
          <div className="h-4 bg-surface-lighter rounded w-48" />
        </div>
      ) : !opStatus?.cli_installed ? (
        <div className="bg-danger/10 border border-danger/30 rounded-xl p-4 mb-6 flex items-center gap-3">
          <AlertCircle className="w-5 h-5 text-danger shrink-0" />
          <div>
            <p className="text-sm font-medium text-danger">1Password CLI 未安装</p>
            <p className="text-xs text-text-dim mt-0.5">
              请安装 1Password CLI：brew install 1password-cli
            </p>
          </div>
        </div>
      ) : !opConnected ? (
        <div className="bg-surface-light border border-border rounded-xl p-4 mb-6 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Key className="w-5 h-5 text-primary shrink-0" />
            <div>
              <p className="text-sm font-medium">1Password CLI 已安装</p>
              <p className="text-xs text-text-dim mt-0.5">
                版本: {opStatus.cli_version}
              </p>
            </div>
          </div>
          <button
            onClick={() => { setOpConnected(true); refetchVaults(); }}
            className="px-4 py-1.5 bg-primary hover:bg-primary/80 text-white text-sm rounded-lg transition"
          >
            连接 1Password
          </button>
        </div>
      ) : vaultsError ? (
        <div className="bg-warning/10 border border-warning/30 rounded-xl p-4 mb-6 flex items-center gap-3">
          <AlertCircle className="w-5 h-5 text-warning shrink-0" />
          <div>
            <p className="text-sm font-medium text-warning">1Password 未登录</p>
            <p className="text-xs text-text-dim mt-0.5">
              请先在 1Password 桌面应用中启用 CLI 集成，或运行 op signin
            </p>
          </div>
        </div>
      ) : (
        <div className="bg-success/10 border border-success/30 rounded-xl p-4 mb-6 flex items-center gap-3">
          <Key className="w-5 h-5 text-success shrink-0" />
          <div>
            <p className="text-sm font-medium text-success">1Password CLI 已连接</p>
            <p className="text-xs text-text-dim mt-0.5">
              版本: {opStatus.cli_version}
            </p>
          </div>
        </div>
      )}

      {/* Vault & Item Selection */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
        {/* Step 1: Select Vault */}
        <div className="bg-surface-light border border-border rounded-xl p-4">
          <h3 className="text-sm font-medium mb-3 flex items-center gap-2">
            <span className="w-5 h-5 rounded-full bg-primary text-white text-xs flex items-center justify-center">
              1
            </span>
            选择 Vault
          </h3>
          <div className="space-y-1 max-h-48 overflow-y-auto">
            {vaults?.map((vault: Vault) => (
              <button
                key={vault.id}
                onClick={() => {
                  setSelectedVault(vault.id);
                  setSelectedItem(null);
                  setAccounts(null);
                }}
                className={clsx(
                  "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors",
                  selectedVault === vault.id
                    ? "bg-primary/15 text-primary"
                    : "hover:bg-surface-lighter text-text-dim",
                )}
              >
                {vault.name}
              </button>
            )) || (
              <p className="text-xs text-text-dim">加载中...</p>
            )}
          </div>
        </div>

        {/* Step 2: Select Item */}
        <div className="bg-surface-light border border-border rounded-xl p-4">
          <h3 className="text-sm font-medium mb-3 flex items-center gap-2">
            <span className="w-5 h-5 rounded-full bg-primary text-white text-xs flex items-center justify-center">
              2
            </span>
            选择助记词
          </h3>
          <div className="space-y-1 max-h-48 overflow-y-auto">
            {!selectedVault ? (
              <p className="text-xs text-text-dim">请先选择 Vault</p>
            ) : vaultItems?.length === 0 ? (
              <p className="text-xs text-text-dim">该 Vault 中没有项目</p>
            ) : (
              vaultItems?.map((item: VaultItem) => (
                <button
                  key={item.id}
                  onClick={() => {
                    setSelectedItem(item);
                    setAccounts(null);
                  }}
                  className={clsx(
                    "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors flex items-center justify-between",
                    selectedItem?.id === item.id
                      ? "bg-primary/15 text-primary"
                      : "hover:bg-surface-lighter text-text-dim",
                  )}
                >
                  <span>{item.title}</span>
                  <ChevronRight className="w-3.5 h-3.5" />
                </button>
              ))
            )}
          </div>
        </div>

        {/* Step 3: Derive */}
        <div className="bg-surface-light border border-border rounded-xl p-4 flex flex-col">
          <h3 className="text-sm font-medium mb-3 flex items-center gap-2">
            <span className="w-5 h-5 rounded-full bg-primary text-white text-xs flex items-center justify-center">
              3
            </span>
            派生账户
          </h3>
          <p className="text-xs text-text-dim mb-4 flex-1">
            {selectedItem
              ? `将从 "${selectedItem.title}" 派生 EVM、Solana、Bitcoin 账户`
              : "请先选择助记词项目"}
          </p>
          <button
            onClick={handleDerive}
            disabled={!selectedItem || deriveMutation.isPending}
            className={clsx(
              "w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium transition-all",
              "bg-primary text-white hover:bg-primary-dark",
              "disabled:opacity-50 disabled:cursor-not-allowed",
            )}
          >
            {deriveMutation.isPending ? (
              <>
                <RefreshCw className="w-4 h-4 animate-spin" />
                派生中...
              </>
            ) : (
              <>
                <Key className="w-4 h-4" />
                派生账户
              </>
            )}
          </button>
          {deriveMutation.isError && (
            <p className="text-xs text-danger mt-2">
              {String(deriveMutation.error)}
            </p>
          )}
        </div>
      </div>

      {/* Accounts */}
      {allAccounts.length > 0 && (
        <>
          <h2 className="text-lg font-semibold mb-4">
            派生账户 ({allAccounts.length})
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
            {allAccounts.map((account) => {
              const balance = balanceReport?.accounts.find(
                (b) => b.address === account.address,
              );
              return (
                <AccountCard
                  key={`${account.chain}-${account.address}`}
                  account={account}
                  balance={balance}
                  isLoading={balancesLoading}
                />
              );
            })}
          </div>
        </>
      )}
    </div>
  );
}
