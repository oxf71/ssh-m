import { NavLink } from "react-router-dom";
import { Terminal, /* Coins, */ Settings, Shield } from "lucide-react";
import { clsx } from "clsx";

const navItems = [
  { to: "/", icon: Terminal, label: "SSH 管理" },
  // { to: "/blockchain", icon: Coins, label: "区块链账户" },
  { to: "/settings", icon: Settings, label: "设置" },
];

export function Sidebar() {
  return (
    <aside className="w-56 bg-surface-light border-r border-border flex flex-col h-full shrink-0">
      {/* Logo */}
      <div className="flex items-center gap-2 px-4 py-4 border-b border-border">
        <Shield className="w-6 h-6 text-primary" />
        <span className="font-bold text-lg">SSH-M</span>
      </div>

      {/* Navigation */}
      <nav className="flex-1 py-3">
        {navItems.map(({ to, icon: Icon, label }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              clsx(
                "flex items-center gap-3 px-4 py-2.5 mx-2 rounded-lg text-sm transition-colors",
                isActive
                  ? "bg-primary/15 text-primary font-medium"
                  : "text-text-dim hover:bg-surface-lighter hover:text-text",
              )
            }
          >
            <Icon className="w-4.5 h-4.5" />
            {label}
          </NavLink>
        ))}
      </nav>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-border text-xs text-text-dim">
        SSH-M v0.1.0
      </div>
    </aside>
  );
}
