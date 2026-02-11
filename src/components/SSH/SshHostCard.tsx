import { clsx } from "clsx";
import {
  Monitor,
  ArrowRight,
  Shield,
  Network,
  Globe,
} from "lucide-react";
import type { SshHost } from "../../types/ssh";

interface SshHostCardProps {
  host: SshHost;
  onConnect: (name: string) => void;
  isConnecting: boolean;
}

const groupIcons: Record<string, React.FC<{ className?: string }>> = {
  direct: Monitor,
  proxy: Network,
  local: Monitor,
  github: Globe,
};

const groupLabels: Record<string, string> = {
  direct: "直连",
  proxy: "跳板",
  local: "本地",
  github: "代码托管",
};

export function SshHostCard({ host, onConnect, isConnecting }: SshHostCardProps) {
  const Icon = groupIcons[host.group] || Monitor;

  return (
    <div className="bg-surface-light border border-border rounded-xl p-4 hover:border-primary/40 transition-colors group">
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div
            className={clsx(
              "w-10 h-10 rounded-lg flex items-center justify-center",
              host.group === "proxy" ? "bg-warning/15 text-warning" :
              host.group === "github" ? "bg-success/15 text-success" :
              "bg-primary/15 text-primary",
            )}
          >
            <Icon className="w-5 h-5" />
          </div>
          <div>
            <h3 className="font-medium text-sm">{host.name}</h3>
            <p className="text-xs text-text-dim mt-0.5">
              {host.user ? `${host.user}@` : ""}
              {host.hostname}
              {host.port && host.port !== 22 ? `:${host.port}` : ""}
            </p>
          </div>
        </div>

        <button
          onClick={() => onConnect(host.name)}
          disabled={isConnecting}
          className={clsx(
            "flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium transition-all",
            "bg-primary/10 text-primary hover:bg-primary hover:text-white",
            "disabled:opacity-50 disabled:cursor-not-allowed",
          )}
        >
          <ArrowRight className="w-3.5 h-3.5" />
          连接
        </button>
      </div>

      {/* Tags */}
      <div className="flex items-center gap-2 mt-3">
        <span className="text-[10px] px-2 py-0.5 rounded-full bg-surface-lighter text-text-dim">
          {groupLabels[host.group]}
        </span>
        {host.is_1password_agent && (
          <span className="text-[10px] px-2 py-0.5 rounded-full bg-primary/10 text-primary flex items-center gap-1">
            <Shield className="w-2.5 h-2.5" />
            1Password
          </span>
        )}
        {host.proxy_jump && (
          <span className="text-[10px] px-2 py-0.5 rounded-full bg-warning/10 text-warning">
            via {host.proxy_jump}
          </span>
        )}
        {host.identity_file && (
          <span className="text-[10px] px-2 py-0.5 rounded-full bg-surface-lighter text-text-dim truncate max-w-40">
            {host.identity_file.split("/").pop()}
          </span>
        )}
      </div>
    </div>
  );
}
