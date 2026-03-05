# MiniBot.rs

一個用 **100% Rust** 實作的高效能 AI Agent 執行框架。

## 特性

- **極低資源消耗**：< 5MB RAM 執行時期記憶體
- **快速啟動**：< 10ms 冷啟動時間
- **高度模組化**：透過 Trait 驅動的架構，所有元件皆可替換
- **跨平台**：支援 ARM、x86、RISC-V 架構
- **安全預設**：嚴格的沙箱隔離、明確的白名單

## MVP 功能

### CLI 命令列介面

```bash
mini_bot.rs agent      # 啟動互動式對話
mini_bot.rs gateway   # 啟動 Webhook 閘道伺服器
mini_bot.rs config    # 配置管理
```

### 核心工具集

- **Shell Tool** - 執行系統命令
- **File Tool** - 檔案讀寫操作
- ~~Web Fetch Tool~~ - 擷取網頁內容（籌備中）

### 支援的 AI 模型供應商

- MiniMax (預設)
- 支援擴充其他供應商

## 專案結構

```
mini_bot.rs/
├── Cargo.toml              # Rust 專案配置
├── src/
│   ├── main.rs             # 程式進入點
│   ├── lib.rs              # 模組匯出
│   ├── config/             # 配置管理
│   ├── agent/              # Agent 核心
│   ├── providers/          # AI 模型供應商
│   ├── tools/              # 工具集合
│   └── memory/             # 記憶體系統
└── README.md
```

## 建置需求

| 軟體 | 版本需求 |
|------|----------|
| Rust | 1.87+    |
| Cargo | 內建於 Rust |
| SQLite | 3 |

## 安裝

```bash
# 建置專案
cargo build --release
```

## 使用方式

```bash
# 互動式 Agent 對話
cargo run -- agent

# 單次訊息模式
cargo run -- agent --message "Hello"

# 啟動 Gateway
cargo run -- gateway --port 3000

# 顯示版本
cargo run -- version
```

## 設定檔

建立 `config.toml` 檔案並放置於：

- **Windows**: `%APPDATA%\com.minibot.mini_bot_rs\config.toml`
- **macOS/Linux**: `~/.config/com.minibot.mini_bot_rs/config.toml`

或使用 `--config-dir` 參數指定配置目錄。

### 範例 config.toml

```toml
version = "1.0"
default_provider = "minimax"
default_model = "minimax-coding-plan/MiniMax-M2.5"
api_key = "YOUR_MINIMAX_API_KEY"

[gateway]
host = "127.0.0.1"
port = 3000

[agent]
max_tool_iterations = 100
max_history_messages = 50
temperature = 0.7

[security]
workspace_only = true
allowed_roots = []
allowed_commands = []
```

## 測試

```bash
# 執行所有單元測試
cargo test

# 執行特定模組測試
cargo test --lib
cargo test --test integration

# 執行涵蓋率測試（需要先安裝 cargo-llvm-cov）
cargo install cargo-llvm-cov
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --text
```

### 涵蓋率報告

涵蓋率報告會顯示各模組的測試覆蓋程度：
- `src/config/` - 配置管理
- `src/agent/` - Agent 核心（含歷史管理）
- `src/providers/` - AI 供應商
- `src/tools/` - 工具集合（Shell、File）

## 授權

MIT OR Apache-2.0

<!-- [😸SAM] -->