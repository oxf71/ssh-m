import { useState } from "react";
import { Settings as SettingsIcon, Save, RotateCcw } from "lucide-react";
import { clsx } from "clsx";

// --- 区块链 RPC 配置（暂时注释）---
// interface ChainRpcSetting { name: string; chain_type: string; rpc_url: string; default_url: string; }
// const defaultRpcSettings: ChainRpcSetting[] = [ ... ];

export function SettingsPage() {
  const [defaultTerminal, setDefaultTerminal] = useState(
    () => localStorage.getItem("ssh-m:defaultTerminal") || "terminal"
  );
  const [sshConfigPath, setSshConfigPath] = useState(
    () => localStorage.getItem("ssh-m:sshConfigPath") || "~/.ssh/config"
  );
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    localStorage.setItem("ssh-m:defaultTerminal", defaultTerminal);
    localStorage.setItem("ssh-m:sshConfigPath", sshConfigPath);
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className="p-6 max-w-3xl">
      <div className="mb-6">
        <h1 className="text-xl font-bold flex items-center gap-2">
          <SettingsIcon className="w-5 h-5 text-primary" />
          设置
        </h1>
        <p className="text-sm text-text-dim mt-1">配置 SSH 管理偏好</p>
      </div>

      {/* SSH Settings */}
      <section className="bg-surface-light border border-border rounded-xl p-5 mb-6">
        <h2 className="text-sm font-semibold mb-4">SSH 配置</h2>
        <div className="space-y-4">
          <div className="flex items-center gap-3">
            <label className="w-32 text-sm text-text-dim shrink-0">
              配置文件路径
            </label>
            <input
              type="text"
              value={sshConfigPath}
              onChange={(e) => setSshConfigPath(e.target.value)}
              className="flex-1 px-3 py-2 bg-surface border border-border rounded-lg text-sm font-mono focus:outline-none focus:border-primary transition-colors"
            />
            <button
              onClick={() => setSshConfigPath("~/.ssh/config")}
              className="p-2 rounded-lg hover:bg-surface-lighter text-text-dim hover:text-text transition"
              title="重置为默认"
            >
              <RotateCcw className="w-3.5 h-3.5" />
            </button>
          </div>
          <div className="flex items-center gap-3">
            <label className="w-32 text-sm text-text-dim shrink-0">
              默认终端
            </label>
            <select
              value={defaultTerminal}
              onChange={(e) => setDefaultTerminal(e.target.value)}
              className="flex-1 px-3 py-2 bg-surface border border-border rounded-lg text-sm focus:outline-none focus:border-primary transition-colors"
            >
              <option value="terminal">Terminal.app</option>
              <option value="iterm">iTerm2</option>
              <option value="warp">Warp</option>
            </select>
          </div>
        </div>
      </section>

      {/* Save */}
      <button
        onClick={handleSave}
        className={clsx(
          "flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium transition-all",
          "bg-primary text-white hover:bg-primary-dark",
        )}
      >
        <Save className="w-4 h-4" />
        {saved ? "已保存 ✓" : "保存设置"}
      </button>
    </div>
  );
}
