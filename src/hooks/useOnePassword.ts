import { useQuery } from "@tanstack/react-query";
import { checkOpStatus, listVaults, listVaultItems } from "../services/tauri";

export function useOpStatus() {
  return useQuery({
    queryKey: ["op-status"],
    queryFn: checkOpStatus,
    retry: false,
  });
}

export function useVaults(enabled = false) {
  return useQuery({
    queryKey: ["vaults"],
    queryFn: listVaults,
    enabled,
    retry: false,
  });
}

export function useVaultItems(vaultId: string | null) {
  return useQuery({
    queryKey: ["vault-items", vaultId],
    queryFn: () => listVaultItems(vaultId!),
    enabled: !!vaultId,
    retry: false,
  });
}
