import { clsx } from "clsx";
import { Copy, ExternalLink } from "lucide-react";
import type { Account, AccountBalance, TokenBalance } from "../../types/blockchain";

interface AccountCardProps {
  account: Account;
  balance?: AccountBalance;
  isLoading: boolean;
}

function TokenRow({ token }: { token: TokenBalance }) {
  return (
    <div className="flex items-center justify-between py-1.5 text-xs">
      <div className="flex items-center gap-2">
        <span className="font-medium">{token.symbol}</span>
        {!token.is_native && (
          <span className="text-text-dim">{token.name}</span>
        )}
      </div>
      <span className="font-mono">{token.balance_formatted}</span>
    </div>
  );
}

export function AccountCard({ account, balance, isLoading }: AccountCardProps) {
  const chainColors: Record<string, string> = {
    evm: "text-primary bg-primary/10",
    solana: "text-success bg-success/10",
    bitcoin: "text-warning bg-warning/10",
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(account.address);
  };

  return (
    <div className="bg-surface-light border border-border rounded-xl p-4">
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span
            className={clsx(
              "text-[10px] font-medium px-2 py-0.5 rounded-full uppercase",
              chainColors[account.chain],
            )}
          >
            {account.chain_name}
          </span>
          <span className="text-[10px] text-text-dim font-mono">
            {account.derivation_path}
          </span>
        </div>
      </div>

      {/* Address */}
      <div className="flex items-center gap-2 mb-3">
        <code className="text-xs font-mono text-text-dim flex-1 truncate">
          {account.address}
        </code>
        <button
          onClick={handleCopy}
          className="p-1 rounded hover:bg-surface-lighter text-text-dim hover:text-text transition"
          title="复制地址"
        >
          <Copy className="w-3.5 h-3.5" />
        </button>
        <button
          className="p-1 rounded hover:bg-surface-lighter text-text-dim hover:text-text transition"
          title="在浏览器查看"
        >
          <ExternalLink className="w-3.5 h-3.5" />
        </button>
      </div>

      {/* Balances */}
      {isLoading ? (
        <div className="text-xs text-text-dim animate-pulse">加载余额中...</div>
      ) : balance ? (
        <div className="border-t border-border pt-2">
          <TokenRow token={balance.native_balance} />
          {balance.token_balances.map((token, i) => (
            <TokenRow key={i} token={token} />
          ))}
        </div>
      ) : (
        <div className="text-xs text-text-dim">点击查询余额</div>
      )}
    </div>
  );
}
