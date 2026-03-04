# Security Upgrade TODO - MiniBot.rs

本文件規劃專案的安全性升級任務，所有 AI Agent 都應依此執行。

## 專案安全問題分析

### 🔴 高風險 (High Risk)

#### 1. Shell Tool 命令注入漏洞
**位置**: `src/tools/shell.rs`

**問題**:
- 當 `allowed_commands` 為空時，預設允許所有命令 (第 23-25 行)
- allowlist 檢查僅使用 `starts_with()`，可被繞過 (第 28 行)
- 無命令執行超時或資源限制

**修復方向**:
- [ ] 預設拒絕所有命令，而非允許
- [ ] 實作更嚴格的命令驗證 (完整匹配或正則表達式)
- [ ] 添加命令執行超時 (建議 30 秒)
- [ ] 整合 Config 中的 `security.allowed_commands`

---

#### 2. File Tool 路徑穿越漏洞
**位置**: `src/tools/file.rs`

**問題**:
- 當 `allowed_directory` 為 None 時，允許所有路徑 (第 24-34 行)
- `canonicalize()` 可被 symlink 繞過
- 無檔案大小限制

**修復方向**:
- [ ] 預設設定工作目錄限制
- [x] 添加檔案大小上限 (建議 10MB)
- [ ] 整合 Config 中的 `security.workspace_only` 和 `security.allowed_roots`
- [ ] 實作目錄越界檢查 (檢查 `..` 和 symlink)

---

#### 3. Gateway 無認證保護
**位置**: `src/gateway/mod.rs`

**問題**:
- `/webhook` 端點無任何認證機制
- 無速率限制
- 無 CORS 配置

**修復方向**:
- [ ] 添加 API Key 認證 (X-API-Key header)
- [ ] 實作速率限制 (tower-http rate limit)
- [ ] 添加 CORS 設定 (如果需要跨域訪問)
- [ ] 添加 IP 白名單功能

---

### 🟠 中風險 (Medium Risk)

#### 4. API Key 明文儲存
**位置**: `src/config/mod.rs`, `src/providers/minimax.rs`

**問題**:
- API Key 以明文存儲在 config.toml
- 無環境變數支援

**修復方向**:
- [x] 優先從環境變數讀取 API Key
- [ ] 支援加密的 config 檔案
- [x] 添加 API Key 存在性檢查

---

#### 5. Agent 工具迭代無限制
**位置**: `src/agent/mod.rs`

**問題**:
- Config 定義了 `max_tool_iterations: 100`，但未實際使用
- 無法防止無限工具呼叫迴圈

**修復方向**:
- [x] 在 `execute_tool` 前檢查迭代次數
- [ ] 添加最大執行時間限制

---

#### 6. Memory 儲存無加密
**位置**: `src/memory/sqlite.rs`

**問題**:
- SQLite 資料庫無加密
- Session ID 未經 sanitization

**修復方向**:
- [ ] 添加 SQL injection 防護 (使用參數化查詢)
- [ ] 考慮添加 SQLite 加密 (sqlcipher) 或記錄加密

---

### 🟡 低風險 (Low Risk)

#### 7. 依賴版本檢查
**位置**: `Cargo.toml`

**問題**:
- 需要確認所有依賴版本的安全性

**修復方向**:
- [ ] 執行 `cargo audit` 檢查已知漏洞
- [ ] 更新到安全的版本

---

#### 8. 日誌敏感資訊
**問題**:
- 可能會記錄 API Key 或敏感資料

**修復方向**:
- [ ] 實作日誌 sanitization
- [ ] 添加日誌級別過濾

---

## 執行順序

### Phase 1: 緊急修復 (Critical)
1. Shell Tool 命令限制
2. File Tool 路徑限制
3. Gateway 認證

### Phase 2: 重要修復 (Important)
4. API Key 安全儲存
5. Agent 迭代限制

### Phase 3: 加強修復 (Enhancement)
6. Memory 加密
7. 依賴安全審計
8. 日誌安全

---

## 驗證方式

完成每個任務後，執行以下測試:

```bash
# 依賴安全檢查
cargo audit

# 單元測試
cargo test

# 整合測試
cargo test --test integration
```

---

## 參考資源

- [Rust 安全指南](https://anssi-fr.github.io/rust-secure-coding/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [tower-http 安全 Middleware](https://docs.rs/tower-http/latest/tower_http/)

---

*本文件最後更新於 2026-03-04*
