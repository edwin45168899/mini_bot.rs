# TDD 開發執行指南 (AI Agent 專用)

本指南定義 AI Agent 在執行 TDD（測試驅動開發）時應遵循的流程、互動模式與回報機制。

---

## 1. TDD 核心循環

AI Agent 必須遵循 **Red-Green-Refactor** 循環：

```text
┌─────────────────────────────────────────────────────────┐
│  1. RED    │ 寫一個失敗的測試 (預期失敗)                  │
│────────────┼────────────────────────────────────────────│
│  2. GREEN  │ 寫最少的生產程式碼讓測試通過                 │
│────────────┼────────────────────────────────────────────│
│  3. REFACTOR │ 重構程式碼 (保持測試通過)                 │
└─────────────────────────────────────────────────────────┘
         ↑___________________________________↓
              重複循環直到功能完成
```

---

## 2. Task 拆分原則

每個任務應拆分為 **可測試的子任務**：

| 拆分標準 | 說明                              |
| -------- | --------------------------------- |
| 單一職責 | 每個 struct / function 只做一件事 |
| 可測試性 | 輸出可斷言、副作用可隔離          |
| 邊界條件 | 包含正常、邊界、錯誤情況          |

### Task 範例模板

```markdown
## Task: [功能名稱]

### 子任務

- [ ] T1: 寫測試 - [測試項目描述]
- [ ] T2: 實作 - [功能描述]
- [ ] T3: 重構 - [優化點]
- [ ] T4: 整合測試 - [端到端場景]
```

---

## 3. 互動回報機制

### 3.1 回報時機

每個 **子任務 (Sub-task)** 開始與完成時必須回報：

```text
📋 [開始] T1: 寫測試 - 驗證使用者登入功能
   └─ 預期：建立 test_login_success, test_login_failure, test_login_invalid_password

✅ [完成] T1: 寫測試 - 通過 3/3 測試
   └─ 紅燈：符合預期
```

### 3.2 回報模板

```markdown
## 🚀 Task 進度回報

### 任務：[功能名稱]

**狀態**：進行中 / 完成

### 子任務進度

| 子任務       | 狀態      | 說明           |
| ------------ | --------- | -------------- |
| T1: 寫測試   | ✅ 完成   | 3 tests passed |
| T2: 實作     | 🔄 進行中 | 實作中...      |
| T3: 重構     | ⏳ 待處理 | -              |
| T4: 整合測試 | ⏳ 待處理 | -              |

### 變更檔案

- `src/auth.rs` - 新增
- `tests/integration_test.rs` - 修改

### 遇到問題

無 / [問題描述 + 詢問選項]
```

### 3.3 詢問時機

以下情況 **必須** 詢問開發者：

| 情況       | 詢問範例                                                                       |
| ---------- | ------------------------------------------------------------------------------ |
| 需求不明確 | 「請問登入失敗時要回傳錯誤訊息還是只回傳狀態碼？」                             |
| 技術決策   | 「要用 trait 對外部依賴進行 mock，還是直接在測試版使用真實依賴的替代實作？」   |
| 測試策略   | 「這個整合測試是否需要啟動 local server 還是直接透過 `tower::Service` 呼叫？」 |
| 優先順序   | 「有兩個功能都要做要先做哪個？」                                               |
| 發現風險   | 「重構可能影響現有功能，是否要額外新增回歸測試？」                             |

---

## 4. 測試寫作規範

### 4.1 測試檔案結構

在 Rust 專案中，**單元測試**通常與原始碼放置於同一個檔案（透過 `#[cfg(test)] mod tests` 配置），**整合測試**則放置於專案根目錄的 `tests/` 資料夾內。

```rust
// src/user_service.rs

pub struct UserService { /* ... */ }

impl UserService {
    pub async fn login(&self, input: &str) -> anyhow::Result<String> {
        // ...
        unimplemented!()
    }
}

// ==========================================
// 單元測試區塊
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    // 建立測試主體或共用設定的輔助函數
    fn setup() -> UserService {
        UserService { /* ... */ }
    }

    #[tokio::test]
    async fn test_login_success() {
        // Arrange
        let service = setup();

        // Act
        // (假設原本是 unimplemented，此時應發生 panic)
        let result = service.login("valid_input").await.unwrap();

        // Assert
        assert_eq!(result, "expected_token");
    }

    #[tokio::test]
    async fn test_login_invalid_input_error() {
        // Arrange
        let service = setup();

        // Act
        let result = service.login("invalid").await;

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "invalid input");
    }
}
```

### 4.2 命名規範

| 類型         | 命名模式                         | 範例                        |
| ------------ | -------------------------------- | --------------------------- |
| 單元測試模組 | `#[cfg(test)] mod tests { ... }` | 通常置於原始檔案最下方      |
| 整合測試檔案 | `tests/<category>_test.rs`       | `tests/integration_test.rs` |
| 測試方法     | `test_<action>_<scenario>`       | `test_login_success`        |

### 4.3 測試隔離原則

- **每個測試獨立**：不依賴其他測試的執行順序。Rust 測試預設為平行執行 (`cargo test`），千萬不要依賴全域可變狀態。
- **Given-When-Then**：明確 Arrange / Act / Assert 程式碼區間。
- **不使用共享狀態**：盡量避免使用靜態全域變數產生跨測試污染；如需共用或暫存資源，請在每個測試方法內建立獨立實例 (例：`tempfile::tempdir()` 來產生獨立暫存目錄，或獨立建立 `:memory:` SQLite 連線)。

---

## 5. 驗證清單

### 5.1 Task 開始前

- [ ] 確認需求與驗收標準
- [ ] 拆分可測試的子任務
- [ ] 判定適當的測試類型（單元測試 / 整合測試）

### 5.2 Red 階段 (寫測試)

- [ ] 寫預期會失敗的測試
- [ ] 執行 `cargo test` 確保測試**確實失敗**，並確認失敗原因是「邏輯未實作 (例如 `unimplemented!()`)」或「邏輯錯誤」，**而非「編譯錯誤或語法錯誤」**。(建議先寫空函式讓其編譯通過，再透過斷言或 `unimplemented!()` 確保測試失敗)

### 5.3 Green 階段 (實作)

- [ ] 寫最少量程式碼通過測試
- [ ] 執行 `cargo test` 確認所有相關測試通過

### 5.4 Refactor 階段

- [ ] 重構程式碼，消除重複邏輯
- [ ] 檢查代碼是否符合 idiomatic Rust 寫法
- [ ] 確保修改後相關測試仍然通過 (`cargo test`)

### 5.5 Task 完成前

- [ ] 執行完整測試套件 (`cargo test`)
- [ ] 執行靜態分析檢查 (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] 執行代碼格式化檢查 (`cargo fmt --all -- --check`)
- [ ] 回報進度給開發者

---

## 6. 指令速查

| 動作                              | 指令                             |
| --------------------------------- | -------------------------------- |
| 執行所有測試                      | `cargo test`                     |
| 執行特定測試模組/函式             | `cargo test <test_name>`         |
| 執行並顯示標準輸出 (忽略捕捉)     | `cargo test -- --nocapture`      |
| 包含執行 `#[ignore]` 測試         | `cargo test -- --ignored`        |
| 檢查代碼語法與型別                | `cargo check`                    |
| 自動格式化代碼                    | `cargo fmt`                      |
| 執行靜態分析 (Clippy)             | `cargo clippy --all-targets`     |
| 限制單執行緒跑測試 (避免資源競爭) | `cargo test -- --test-threads=1` |

---

## 7. 推薦工具與配置

### 7.1 測試模組與輔助套件 (見 `Cargo.toml`)

- **`tokio::test` 或 `tokio-test`** (包含在 `dev-dependencies`)：用於非同步運算的測試場景。
- **`tempfile`** (包含在 `dev-dependencies`)：用於需要隨機寫入檔案系統的測試，避免污染使用者實體環境。

### 7.2 模擬物件 Mocking (依賴注入)

在 Rust 中，我們較偏向使用 **Trait 動態分發 (`dyn Trait`)** 或 **靜態泛型注入** 進行 mocking，而不是在 Runtime 去覆寫函式。專案內目前未使用 `mockall`，我們傾向手動實作測試專用的型別：

**範例**：

```rust
#[async_trait::async_trait]
pub trait ApiClient: Send + Sync {
    async fn fetch_data(&self) -> anyhow::Result<String>;
}

// 供正式環境使用的實作
pub struct RealClient { /* ... */ }
#[async_trait::async_trait]
impl ApiClient for RealClient {
    async fn fetch_data(&self) -> anyhow::Result<String> { /* ... */ }
}

// 供測試環境使用的 Mock
#[cfg(test)]
pub struct MockClient {
    pub return_value: String,
}

#[cfg(test)]
#[async_trait::async_trait]
impl ApiClient for MockClient {
    async fn fetch_data(&self) -> anyhow::Result<String> {
        Ok(self.return_value.clone())
    }
}
```

### 7.3 API 整合測試

若需要測試 `axum` 路由層，可以直接使用 `tower::Service` API 呼叫而無需實際綁定對外連接埠 (port)，以減少系統不可靠和連接埠被佔用的問題。

---

## 8. 範例：完整 Task 流程

### Task: 新增使用者登入功能

```markdown
## 🚀 Task 進度回報

### 任務：使用者登入功能

**狀態**：進行中

---

### T1: 寫測試 (RED)

**狀態**：✅ 完成

$ cargo test test_login_success
running 1 test
test user_service::tests::test_login_success ... FAILED

failures:
---- user_service::tests::test_login_success stdout ----
thread 'user_service::tests::test_login_success' panicked at src/user_service.rs:15:9:
not yet implemented

紅燈：符合預期 (功能以 `unimplemented!()` 佔位符導致 Panic)

---

### T2: 實作登入功能 (GREEN)

**狀態**：🔄 進行中

正在實作 `UserService::login()` 方法與相關認證邏輯...
```

```markdown
## 🚀 Task 進度回報

### 任務：使用者登入功能

**狀態**：✅ 完成

---

### T1: 寫測試 - ✅ 完成

- `test_login_success` - 驗證正確帳密
- `test_login_invalid_password` - 驗證密碼錯誤

### T2: 實作 - ✅ 完成

- `UserService::login()` 已實作，正確處理密碼驗證邏輯
- 執行 `cargo test user_service::` 相關單元測試通過

### T3: 重構 - ✅ 完成

- 提取共用的密碼雜湊檢查為封裝良好的輔助函式
- 將錯誤從 `anyhow::Error` 定義為更明確的自訂 Enum (`thiserror` 實作)

### T4: 整合測試 - ✅ 完成

- 新增 `tests/integration_test.rs` 中的 E2E 認證流程測試 - 通過

---

### 變更檔案

- `src/user_service.rs` - 修改
- `tests/integration_test.rs` - 修改

### 驗證結果

$ cargo test
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

---

## 9. 開發者互動範例

### 範例 1: 需求確認

```text
❓ 詢問：關於登入失敗的處理方式，請確認：
1. 是否要回傳 `anyhow::Result` 作為泛用錯誤，還是定義一個專用的 `AuthError` enum (使用 `thiserror`) 來區分錯誤類型以便於 API 回傳？
2. 失敗訊息要統一顯示「帳號或密碼錯誤」還是分開顯示？
```

### 範例 2: 技術決策

```text
❓ 詢問：在這部分關聯到 SQLite 時：
測試配置當中是否要使用 `tempfile::tempdir()` 建立實體 SQLite 檔案，還是直接使用 SQLite `::memory:` 以提高速度即可？
```

### 範例 3: 優先順序

```text
⚠️ 發現：將 `UserService` 重構為依賴 `dyn ApiClient` Trait 可能會牽動現有整合測試的大量修改。
是否要：
1. 先做全面重構（一次改好，風險較高）
2. 第一階段只針對新功能 `login` 進行介面抽象，之後再逐步調整其他舊功能？
```
