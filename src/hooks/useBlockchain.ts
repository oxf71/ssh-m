import { useQuery, useMutation } from "@tanstack/react-query";
import { deriveAccounts, queryBalances, getDefaultChainConfigs } from "../services/tauri";
import type { MultiChainAccounts, ChainConfig } from "../types/blockchain";

export function useDefaultChainConfigs() {
  return useQuery({
    queryKey: ["chain-configs"],
    queryFn: getDefaultChainConfigs,
  });
}

export function useDeriveAccounts() {
  return useMutation({
    mutationFn: ({
      vault,
      item,
      field,
      chains,
      count,
    }: {
      vault: string;
      item: string;
      field: string;
      chains: string[];
      count: number;
    }) => deriveAccounts(vault, item, field, chains, count),
  });
}

export function useQueryBalances(
  accounts: MultiChainAccounts | null,
  chainsConfig: ChainConfig[] | undefined,
) {
  const addresses = accounts
    ? [
        ...accounts.evm.map((a) => ({ address: a.address, chain_type: "evm" })),
        ...accounts.solana.map((a) => ({ address: a.address, chain_type: "solana" })),
        ...accounts.bitcoin.map((a) => ({ address: a.address, chain_type: "bitcoin" })),
      ]
    : [];

  return useQuery({
    queryKey: ["balances", addresses.map((a) => a.address).join(",")],
    queryFn: () => queryBalances(addresses, chainsConfig || []),
    enabled: addresses.length > 0 && !!chainsConfig,
    refetchInterval: 60000, // Refresh every minute
  });
}
