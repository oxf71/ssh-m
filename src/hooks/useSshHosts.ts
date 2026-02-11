import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { listSshHosts, openSshTerminal, refreshSshConfig } from "../services/tauri";

export function useSshHosts() {
  return useQuery({
    queryKey: ["ssh-hosts"],
    queryFn: listSshHosts,
  });
}

export function useOpenSshTerminal() {
  return useMutation({
    mutationFn: ({ host, terminal }: { host: string; terminal?: string }) =>
      openSshTerminal(host, terminal),
  });
}

export function useRefreshSshConfig() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: refreshSshConfig,
    onSuccess: (data) => {
      queryClient.setQueryData(["ssh-hosts"], data);
    },
  });
}
