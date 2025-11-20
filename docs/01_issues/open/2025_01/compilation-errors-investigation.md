# コンパイルエラー調査結果と修正方法

## 作成日
2025年1月

## 概要

プロジェクト全体で発生しているコンパイルエラーの原因を調査し、修正方法をまとめます。

## 調査結果サマリー

実際のコンパイルエラーを確認した結果、以下の56個のエラーと11個の警告が検出されました。

## エラー一覧

### 1. `file not found for module 'mock'`

**エラー箇所:**
- `app/src/lib.rs:4`

**原因:**
- `pub mod mock;`が定義されているが、`app/src/mock.rs`または`app/src/mock/mod.rs`が存在しない

**修正方法:**
- **推奨**: `app/src/lib.rs`から`pub mod mock;`を削除する（未使用のモジュールのため）
- **代替**: もし`mock`モジュールが必要な場合は、`app/src/mock.rs`を作成する

**修正コード:**
```rust
// app/src/lib.rs
pub mod github;
pub mod components;
pub mod pages;
// pub mod mock;  // ← この行を削除
pub mod concepts;
pub mod synchronizations;
```

---

### 2. `unresolved import 'leptos::ev::ChangeEvent'`

**エラー箇所:**
- `app/src/components/select/mod.rs:2`
- `app/src/components/checkbox/mod.rs:2`
- `app/src/components/radio/mod.rs:2`
- `app/src/components/toggle/mod.rs:2`

**原因:**
- Leptos 0.8では`ChangeEvent`が存在しない
- エラーメッセージ: `no 'ChangeEvent' in 'html::event'`
- Leptos 0.8では`Event`型を使用する必要がある

**修正方法:**
- `ChangeEvent`を`Event`に変更する
- `Callback<ChangeEvent>`を`Callback<Event>`に変更する

**修正コード例:**
```rust
// 修正前
use leptos::ev::ChangeEvent;
#[prop(optional, into)] on_change: Option<Callback<ChangeEvent>>,

// 修正後
use leptos::ev::Event;
#[prop(optional, into)] on_change: Option<Callback<Event>>,
```

**影響範囲:**
- `app/src/components/select/mod.rs`
- `app/src/components/checkbox/mod.rs`
- `app/src/components/radio/mod.rs`
- `app/src/components/toggle/mod.rs`

---

### 3. `use of unresolved module or unlinked crate 'wasm_bindgen'` / `js_sys` / `web_sys`

**エラー箇所:**
- `app/src/components/toast/mod.rs:19-22`
- `app/src/components/countdown/mod.rs:18`

**原因:**
- `wasm_bindgen`、`wasm_bindgen_futures`、`js_sys`、`web_sys`の依存関係が`app/Cargo.toml`に追加されていない
- これらのクレートはWASM環境でのみ使用可能で、SSR環境では使用できない
- Leptosには`leptos::web_sys`が存在するが、直接`web_sys`を使用している

**修正方法:**
- **推奨**: `app/Cargo.toml`に以下の依存関係を追加（条件付きで）:
  ```toml
  [dependencies]
  wasm-bindgen = { version = "0.2", optional = true }
  wasm-bindgen-futures = { version = "0.4", optional = true }
  js-sys = { version = "0.3", optional = true }
  web-sys = { version = "0.3", optional = true }
  
  [features]
  hydrate = ["leptos/hydrate", "dep:wasm-bindgen", "dep:wasm-bindgen-futures", "dep:js-sys", "dep:web-sys"]
  ```
- `toast/mod.rs`と`countdown/mod.rs`のWASM関連のコードを条件付きコンパイルで囲む:
  ```rust
  #[cfg(feature = "hydrate")]
  use wasm_bindgen::prelude::*;
  #[cfg(feature = "hydrate")]
  use wasm_bindgen_futures::JsFuture;
  #[cfg(feature = "hydrate")]
  use js_sys::Promise;
  #[cfg(feature = "hydrate")]
  use web_sys::window;
  ```
- または、`leptos::web_sys`を使用する（Leptosが提供するラッパーを使用）

---

### 4. `no method named 'call' found for struct 'leptos::prelude::Callback'`

**エラー箇所:**
- `app/src/components/accordion/mod.rs:72`
- `app/src/components/countdown/mod.rs:37`
- `app/src/components/rating/mod.rs:37`
- `app/src/components/toast/mod.rs:61, 79`
- `app/src/components/popover/mod.rs:43`
- `app/src/components/drawer/mod.rs:87, 130, 133`
- `app/src/components/dropdown/mod.rs:79, 82`
- `app/src/components/modal/mod.rs:42`
- `app/src/components/pagination/mod.rs:35`
- `app/src/components/tabs/mod.rs:108`

**原因:**
- Leptos 0.8では`Callback`は`Fn`トレイトを実装しており、`call()`メソッドは存在しない
- `Callback`は直接関数として呼び出すことができる

**修正方法:**
- `callback.call(args)`を`callback(args)`に変更する

**修正コード例:**
```rust
// 修正前
if let Some(callback) = on_close {
    callback.call(());
}

// 修正後
if let Some(callback) = on_close {
    callback(());
}
```

**影響範囲（13箇所）:**
- `app/src/components/accordion/mod.rs`
- `app/src/components/countdown/mod.rs`
- `app/src/components/rating/mod.rs`
- `app/src/components/toast/mod.rs`
- `app/src/components/popover/mod.rs`
- `app/src/components/drawer/mod.rs`
- `app/src/components/dropdown/mod.rs`
- `app/src/components/modal/mod.rs`
- `app/src/components/pagination/mod.rs`
- `app/src/components/tabs/mod.rs`

---

### 5. `expected a 'FnMut(MouseEvent)' closure, found 'std::option::Option<leptos::prelude::Callback<MouseEvent>>'`

**エラー箇所:**
- `app/src/components/accordion/mod.rs:120`
- `app/src/components/button/mod.rs:39`
- その他複数のコンポーネントで`on:click=on_click`を使用している箇所

**原因:**
- Leptos 0.8では、`on:click`に`Option<Callback<MouseEvent>>`を直接渡すことができない
- `on:click`は`FnMut(MouseEvent)`を期待しているが、`Option<Callback>`は直接渡せない

**修正方法:**
- `Option<Callback>`をクロージャーでラップする
- または、`on_click`が`None`の場合は何もしないクロージャーを渡す

**修正コード例:**
```rust
// 修正前
<button on:click=on_click>

// 修正後（方法1: クロージャーでラップ）
<button on:click=move |ev| {
    if let Some(cb) = on_click {
        cb(ev);
    }
}>

// 修正後（方法2: ヘルパー関数を使用）
let handle_click = move |ev: MouseEvent| {
    if let Some(cb) = on_click {
        cb(ev);
    }
};
<button on:click=handle_click>
```

**注意**: `button/mod.rs`では`on:click=on_click`が動作しているように見えるが、実際にはエラーが発生している可能性がある。確認が必要。

---

### 6. `missing lifetime specifier`

**エラー箇所:**
- `app/src/concepts/user/actions.rs:94`
- `app/src/concepts/repository/actions.rs:133`
- `app/src/concepts/activity/actions.rs:95`

**原因:**
- 参照を返す関数で、どの引数から借用しているかを明示するライフタイム指定が必要
- Rustコンパイラが自動推論できないため、明示的なライフタイム指定が必要

**修正方法:**
- エラーメッセージに従ってライフタイムを追加する

**修正コード例:**
```rust
// 修正前
pub fn find_user_by_username(state: &UserState, username: &str) -> Option<&User> {
    state.users.iter().find(|u| u.username == username)
}

// 修正後
pub fn find_user_by_username<'a>(state: &'a UserState, username: &'a str) -> Option<&'a User> {
    state.users.iter().find(|u| u.username == username)
}
```

**影響範囲:**
- `app/src/concepts/user/actions.rs:94`
- `app/src/concepts/repository/actions.rs:133`
- `app/src/concepts/activity/actions.rs:95`

---

### 7. `cannot find function 'spawn_local' in this scope`

**エラー箇所:**
- `app/src/components/toast/mod.rs:67`

**原因:**
- `spawn_local`がインポートされていない
- Leptos 0.8では`leptos::spawn_local`を使用する必要がある

**修正方法:**
- `leptos::spawn_local`をインポートする

**修正コード例:**
```rust
// 修正前
spawn_local(async move {
    // ...
});

// 修正後
use leptos::spawn_local;

spawn_local(async move {
    // ...
});
```

**注意**: `spawn_local`はWASM環境でのみ使用可能なため、条件付きコンパイルが必要な場合がある。

---

### 8. `window()`の戻り値の型不一致

**エラー箇所:**
- `app/src/components/toast/mod.rs:70`

**原因:**
- `web_sys::window()`は`Option<Window>`を返すが、コードでは`Window`を期待している
- `if let Some(window) = window()`の構文が正しくない

**修正方法:**
- `window()`の戻り値を正しく処理する

**修正コード例:**
```rust
// 修正前
let promise = Promise::new(&mut |resolve, _| {
    if let Some(window) = window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve,
            dur_ms as i32,
        );
    }
});

// 修正後
let promise = Promise::new(&mut |resolve, _| {
    if let Some(window) = web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve,
            dur_ms as i32,
        );
    }
});
```

### 9. `if`と`else`の型不一致（toast/mod.rs）

**エラー箇所:**
- `app/src/components/toast/mod.rs:87-95`

**原因:**
- `view! {}.into_view()`が`View<()>`を返すが、`if`節は`View<View<...>>`を返している
- 型が一致しない

**修正方法:**
- `else`節も同じ型を返すように修正する

**修正コード例:**
```rust
// 修正前
{if on_close.is_some() {
    view! {
        <button class="btn btn-sm btn-circle" on:click=handle_close>
            "✕"
        </button>
    }.into_view()
} else {
    view! {}.into_view()
}}

// 修正後
{move || {
    if on_close.is_some() {
        view! {
            <button class="btn btn-sm btn-circle" on:click=handle_close>
                "✕"
            </button>
        }.into_view()
    } else {
        ().into_view()
    }
}}
```

## 修正優先順位

1. **高優先度（プロジェクト全体に影響）**
   - ✅ `mock`モジュールのエラー（1箇所）
   - ✅ `Callback::call()`のエラー（13箇所）

2. **中優先度（複数コンポーネントに影響）**
   - ✅ `ChangeEvent`のエラー（4箇所）
   - ✅ `on:click`のエラー（複数箇所）
   - ✅ ライフタイム指定のエラー（3箇所）

3. **低優先度（特定コンポーネントのみ）**
   - ✅ `wasm_bindgen`関連のエラー（`toast`、`countdown`コンポーネント）
   - ✅ `spawn_local`のエラー（`toast`コンポーネントのみ）
   - ✅ `window()`の型不一致（`toast`コンポーネント）
   - ✅ `if/else`の型不一致（`toast`コンポーネント）

## 修正手順

### ステップ1: 基本的なエラーを修正
1. `app/src/lib.rs`から`pub mod mock;`を削除
2. `Callback::call()`を直接呼び出しに変更（13箇所）
3. `ChangeEvent`を`Event`に変更（4箇所）
4. ライフタイム指定を追加（3箇所）

### ステップ2: WASM関連のエラーを修正
1. `app/Cargo.toml`にWASM依存関係を追加
2. `toast/mod.rs`と`countdown/mod.rs`を修正
3. `spawn_local`をインポート

### ステップ3: UI関連のエラーを修正
1. `on:click`の`Option<Callback>`問題を修正
2. `toast/mod.rs`の型不一致を修正

### ステップ4: コンパイル確認
```bash
cargo check --manifest-path app/Cargo.toml
```

## 次のステップ

1. ✅ エラーの原因を特定（完了）
2. ⏳ エラーを優先順位に従って修正する
3. ⏳ 修正後、テストを実行して動作を確認する

## 関連ドキュメント

- Leptos 0.8 Documentation: https://book.leptos.dev/
- Leptos 0.8 API Reference: https://docs.rs/leptos/latest/leptos/

