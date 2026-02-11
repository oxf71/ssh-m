# SSH-M

轻量级 SSH 连接管理桌面应用，基于 Tauri v2 构建。

解析本地 `~/.ssh/config`，按分组可视化展示所有 SSH 主机，一键打开终端连接。支持内置编辑器直接编辑 SSH 配置文件（含 `Include` 子配置）。

![Tauri](https://img.shields.io/badge/Tauri-v2-blue)
![React](https://img.shields.io/badge/React-19-61dafb)
![Rust](https://img.shields.io/badge/Rust-2021-orange)
![License](https://img.shields.io/badge/License-MIT-green)

## 功能

- **SSH 主机管理** — 自动解析 `~/.ssh/config`，按 直连 / 跳板 / 本地 / 代码托管 分组
- **一键连接** — 点击即在 Terminal.app / iTerm2 / Warp 中打开 SSH 连接
- **配置编辑器** — 内置文本编辑器，支持 `Include` 引用的子配置文件目录浏览
- **格式校验** — 保存时自动校验 SSH 配置语法（未知指令、非法值、端口范围等）
- **搜索 & 过滤** — 按主机名、地址搜索，按分组过滤
- **1Password 标识** — 自动检测使用 1Password SSH Agent 的主机

## 截图

> *TODO: 添加截图*

## 安装

### 下载安装包

前往 [Releases](../../releases) 下载对应平台安装包：

| 平台 | 文件 |
|---|---|
| macOS (Apple Silicon) | `ssh-m_x.x.x_aarch64.dmg` |
| macOS (Intel) | `ssh-m_x.x.x_x64.dmg` |
| Linux | `.deb` / `.AppImage` |
| Windows | `.msi` |

> macOS 首次打开如提示未验证开发者，右键 → 打开，或前往 系统设置 → 隐私与安全 → 仍然打开。

### 从源码构建

**依赖：**
- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 20
- [pnpm](https://pnpm.io/) >= 9
- Linux 额外需要：`libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`

```bash
git clone https://github.com/your-username/ssh-m.git
cd ssh-m
pnpm install
pnpm tauri build
```

构建产物在 `src-tauri/target/release/bundle/` 目录下。

## 开发

```bash
# 启动开发模式（前端热重载 + Rust 自动编译）
pnpm tauri dev

# 仅前端类型检查
npx tsc --noEmit

# 仅 Rust 类型检查
cd src-tauri && cargo check
```

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri v2 |
| 前端 | React 19 + TypeScript + Vite 7 |
| 样式 | Tailwind CSS v4 |
| 路由 | React Router DOM v7 |
| 数据获取 | TanStack React Query v5 |
| 图标 | Lucide React |
| SSH 解析 | ssh2-config (Rust) |

## 项目结构

```
ssh-m/
├── src/                    # 前端 React + TypeScript
│   ├── components/         # UI 组件
│   │   ├── Layout/         # 侧边栏 + 布局
│   │   └── SSH/            # SSH 主机卡片 + 配置编辑器
│   ├── hooks/              # React Query hooks
│   ├── pages/              # 路由页面
│   ├── services/           # Tauri invoke 封装
│   └── types/              # TypeScript 类型定义
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── ssh/            # SSH config 解析
│       ├── onepassword/    # 1Password CLI 集成
│       ├── crypto/         # 密钥派生 (BIP44)
│       ├── blockchain/     # 链上查询
│       └── commands/       # Tauri 命令
└── .github/workflows/      # CI/CD
```

## 设置

在应用内 **设置** 页面可配置：

- **默认终端** — Terminal.app / iTerm2 / Warp
- **SSH 配置文件路径** — 默认 `~/.ssh/config`

## License

MIT
