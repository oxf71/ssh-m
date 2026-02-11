import { useState } from "react";
import { RefreshCw, Search, Terminal, FileEdit } from "lucide-react";
import { clsx } from "clsx";
import { useSshHosts, useOpenSshTerminal, useRefreshSshConfig } from "../hooks/useSshHosts";
import { SshHostCard, SshConfigEditor } from "../components/SSH";
import type { SshHost } from "../types/ssh";

export function SshManager() {
  const { data: hosts, isLoading, error } = useSshHosts();
  const connectMutation = useOpenSshTerminal();
  const refreshMutation = useRefreshSshConfig();
  const [search, setSearch] = useState("");
  const [activeGroup, setActiveGroup] = useState<string | "all">("all");
  const [editorOpen, setEditorOpen] = useState(false);

  const groups = ["all", "direct", "proxy", "local", "github"] as const;
  const groupLabels: Record<string, string> = {
    all: "全部",
    direct: "直连",
    proxy: "跳板",
    local: "本地",
    github: "代码托管",
  };

  const filteredHosts = (hosts || []).filter((h: SshHost) => {
    const matchSearch =
      !search ||
      h.name.toLowerCase().includes(search.toLowerCase()) ||
      h.hostname.toLowerCase().includes(search.toLowerCase());
    const matchGroup = activeGroup === "all" || h.group === activeGroup;
    return matchSearch && matchGroup;
  });

  return (
    <div className="p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-xl font-bold flex items-center gap-2">
            <Terminal className="w-5 h-5 text-primary" />
            SSH 管理
          </h1>
          <p className="text-sm text-text-dim mt-1">
            管理 SSH 配置，一键连接远程服务器
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setEditorOpen(true)}
            className={clsx(
              "flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all",
              "bg-surface-light border border-border text-text-dim hover:text-text hover:bg-surface-lighter",
            )}
          >
            <FileEdit className="w-4 h-4" />
            编辑配置
          </button>
          <button
            onClick={() => refreshMutation.mutate()}
            disabled={refreshMutation.isPending}
            className={clsx(
              "flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition-all",
              "bg-primary/10 text-primary hover:bg-primary hover:text-white",
            )}
          >
            <RefreshCw
              className={clsx("w-4 h-4", refreshMutation.isPending && "animate-spin")}
            />
            刷新配置
          </button>
        </div>
      </div>

      {/* Search & Filter */}
      <div className="flex items-center gap-4 mb-6">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-text-dim" />
          <input
            type="text"
            placeholder="搜索主机名或地址..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full pl-9 pr-4 py-2 bg-surface-light border border-border rounded-lg text-sm focus:outline-none focus:border-primary transition-colors"
          />
        </div>
        <div className="flex gap-1 bg-surface-light rounded-lg p-1 border border-border">
          {groups.map((g) => (
            <button
              key={g}
              onClick={() => setActiveGroup(g)}
              className={clsx(
                "px-3 py-1 rounded-md text-xs font-medium transition-colors",
                activeGroup === g
                  ? "bg-primary text-white"
                  : "text-text-dim hover:text-text hover:bg-surface-lighter",
              )}
            >
              {groupLabels[g]}
            </button>
          ))}
        </div>
      </div>

      {/* Content */}
      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="bg-surface-light border border-border rounded-xl p-4 animate-pulse">
              <div className="h-4 bg-surface-lighter rounded w-32 mb-3" />
              <div className="h-3 bg-surface-lighter rounded w-48 mb-2" />
              <div className="h-3 bg-surface-lighter rounded w-24" />
            </div>
          ))}
        </div>
      ) : error ? (
        <div className="text-center py-12">
          <p className="text-danger text-sm">加载 SSH 配置失败</p>
          <p className="text-xs text-text-dim mt-1">{String(error)}</p>
        </div>
      ) : filteredHosts.length === 0 ? (
        <div className="text-center py-12">
          <Terminal className="w-12 h-12 text-text-dim mx-auto mb-3" />
          <p className="text-text-dim text-sm">
            {search ? "没有找到匹配的主机" : "未找到 SSH 配置"}
          </p>
          <p className="text-xs text-text-dim mt-1">
            请确保 ~/.ssh/config 文件存在
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
          {filteredHosts.map((host: SshHost) => (
            <SshHostCard
              key={host.name}
              host={host}
              onConnect={(name) => {
                const terminal = localStorage.getItem("ssh-m:defaultTerminal") || "terminal";
                connectMutation.mutate({ host: name, terminal });
              }}
              isConnecting={connectMutation.isPending}
            />
          ))}
        </div>
      )}

      {/* SSH Config Editor Modal */}
      <SshConfigEditor
        open={editorOpen}
        onClose={() => setEditorOpen(false)}
        onSaved={() => refreshMutation.mutate()}
      />
    </div>
  );
}
