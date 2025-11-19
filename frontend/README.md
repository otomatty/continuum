# Frontend ディレクトリ

このディレクトリは、WASMクライアント専用のコードを配置する場所です。

## 現在の構造

```
frontend/
├── Cargo.toml          # WASMクライアント用の設定
└── src/
    └── lib.rs          # WASMエントリーポイント（ハイドレーション）
```

## 役割

### 1. WASMエントリーポイント（必須）

`frontend/src/lib.rs`は、ブラウザで実行されるWASMバイナリのエントリーポイントです。

**現在の実装:**
```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    // ロギングの初期化
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // サーバーから送られてきたHTMLにインタラクティブ性を追加
    leptos::mount::hydrate_body(App);
}
```

この`hydrate()`関数は、サーバーサイドレンダリング（SSR）で生成されたHTMLに、クライアントサイドのインタラクティブ性を追加します。

### 2. クライアント専用のコード（オプション）

以下のような**ブラウザ専用**のコードがある場合、`frontend`ディレクトリに配置します：

#### ✅ `frontend`に配置すべきコード

1. **ブラウザAPI専用のコード**
   - `web_sys`を使用するコード
   - `wasm_bindgen`でブラウザAPIを直接呼び出すコード
   - LocalStorage、SessionStorage、IndexedDBなどのブラウザストレージ

   **例: `frontend/src/browser_storage.rs`**
   ```rust
   use wasm_bindgen::prelude::*;
   use web_sys::window;

   pub fn set_local_storage(key: &str, value: &str) -> Result<(), JsValue> {
       let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
       let storage = window.local_storage()?
           .ok_or_else(|| JsValue::from_str("No localStorage"))?;
       storage.set_item(key, value)?;
       Ok(())
   }
   ```

2. **クライアントサイド専用の初期化コード**
   - ブラウザ拡張機能との連携
   - Service Workerの登録
   - クライアント専用のエラーハンドリング

   **例: `frontend/src/init.rs`**
   ```rust
   pub fn init_client_only_features() {
       // Service Workerの登録
       // ブラウザ拡張機能との連携
       // クライアント専用の設定
   }
   ```

3. **WASM専用のユーティリティ**
   - パフォーマンス測定（`Performance API`）
   - ブラウザの機能検出
   - クライアントサイドのキャッシュ管理

#### ❌ `frontend`に配置**しない**コード

以下のコードは`app`クレートに配置し、`cfg-if`や`features`で条件分岐します：

1. **SSRとクライアントの両方で使用されるコード**
   - コンポーネント（`app/src/components/`）
   - ページ（`app/src/pages/`）
   - ビジネスロジック（`app/src/github/`）

2. **条件分岐で対応できるコード**
   - `cfg-if`を使用してSSR/クライアントを切り替えるコード

   **例: `app/src/utils/storage.rs`**
   ```rust
   use cfg_if::cfg_if;

   cfg_if! {
       if #[cfg(target_arch = "wasm32")] {
           // ブラウザでの実装
           pub fn get_storage(key: &str) -> Option<String> {
               // web_sysを使用
           }
       } else {
           // サーバーでの実装
           pub fn get_storage(key: &str) -> Option<String> {
               // サーバーサイドストレージを使用
           }
       }
   }
   ```

## ディレクトリ構造の例

将来的にクライアント専用のコードが必要になった場合：

```
frontend/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs              # WASMエントリーポイント（必須）
    ├── browser_storage.rs  # LocalStorage/SessionStorage（オプション）
    ├── init.rs             # クライアント専用の初期化（オプション）
    └── utils.rs            # WASM専用のユーティリティ（オプション）
```

## 現在のプロジェクトでの推奨事項

### ✅ 現在の構造で十分な場合

現在のプロジェクトでは、**ほとんどのコードは`app`クレートに配置するのが正しい**です：

- ✅ コンポーネント → `app/src/components/`
- ✅ ページ → `app/src/pages/`
- ✅ GitHub APIクライアント → `app/src/github/`
- ✅ ルーティング → `app/src/lib.rs`

### 🔮 将来的に`frontend`に追加する可能性があるコード

以下の機能を実装する場合、`frontend`にコードを追加することを検討してください：

1. **オフライン対応**
   - Service Workerの登録
   - IndexedDBでのデータキャッシュ

2. **ブラウザ拡張機能との連携**
   - Chrome Extension API
   - Firefox Extension API

3. **クライアント専用のパフォーマンス測定**
   - Web Vitalsの測定
   - カスタムメトリクスの収集

4. **クライアントサイドのキャッシュ戦略**
   - ブラウザのCache API
   - クライアント専用のキャッシュ管理

## 実装例

### 例1: LocalStorageのラッパーを追加する場合

**`frontend/Cargo.toml`に依存関係を追加:**
```toml
[dependencies]
web-sys = { version = "0.3", features = ["Window", "Storage", "StorageEvent"] }
wasm-bindgen = { workspace = true }
```

**`frontend/src/browser_storage.rs`を作成:**
```rust
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn set_local_storage(key: &str, value: &str) -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No window"))?;
    let storage = window.local_storage()?
        .ok_or_else(|| JsValue::from_str("No localStorage"))?;
    storage.set_item(key, value)?;
    Ok(())
}

pub fn get_local_storage(key: &str) -> Option<String> {
    let window = window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}
```

**`frontend/src/lib.rs`でエクスポート:**
```rust
mod browser_storage;
pub use browser_storage::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
```

**`app`クレートから使用:**
```rust
// app/src/pages/dashboard/mod.rs
#[cfg(target_arch = "wasm32")]
use frontend::{get_local_storage, set_local_storage};

#[component]
pub fn DashboardPage() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(value) = get_local_storage("theme") {
            // テーマを適用
        }
    }
    // ...
}
```

## まとめ

- **現在**: `frontend/src/lib.rs`は最小限の実装で十分
- **原則**: ほとんどのコードは`app`クレートに配置
- **例外**: ブラウザAPI専用のコードのみ`frontend`に配置
- **判断基準**: SSRでも動作する必要があるか？ → はい → `app`、いいえ → `frontend`

## 関連ドキュメント

- [Leptos公式ドキュメント: SSR vs CSR](https://leptos.dev/leptos_book/ssr_vs_csr.html)
- [プロジェクト構造の説明](../app/src/pages/README.md)

