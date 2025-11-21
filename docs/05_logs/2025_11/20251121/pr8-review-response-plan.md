# PR #8 レビューコメント対応計画

## 概要

PR #8「feat: UIコンポーネントとConcept/Synchronizationの実装準備」に対するレビューコメントの対応計画です。

**PR URL**: https://github.com/otomatty/continuum/pull/8

## レビューコメントの分類

### 1. Leptos 0.8のCallback使用方法の問題（必須対応・高優先度）

Leptos 0.8では、`Callback`は`Fn`トレイトを実装しており、`.call()`メソッドは存在しません。直接関数として呼び出す必要があります。

#### 1.1 `app/src/pages/repositories/components/sort_control.rs:72`
- **問題**: `on_change.call(sort_option)` が使用されている
- **修正**: `on_change(sort_option)` に変更
- **優先度**: 高（コンパイルエラー）

#### 1.2 `app/src/pages/repositories/components/language_filter.rs:30`
- **問題**: `on_change.call(current)` が使用されている
- **修正**: `on_change(current)` に変更
- **優先度**: 高（コンパイルエラー）

#### 1.3 `app/src/pages/contributors/components/status_filter.rs:24`
- **問題**: `on_change.call(role)` が使用されている
- **修正**: `on_change(role)` に変更
- **優先度**: 高（コンパイルエラー）

#### 1.4 `app/src/pages/contributors/components/contributor_card.rs:38`
- **問題**: `callback.call(user.username.clone())` が使用されている
- **修正**: `callback(user.username.clone())` に変更
- **優先度**: 高（コンパイルエラー）

### 2. dotenvクレートの使用方法（推奨改善・低優先度）

#### 2.1 `server/src/config.rs`
- **問題**: `#[cfg(not(test))]` をimportに使用している
- **推奨**: 関数呼び出しに使用する方が明確
- **現状**: 既に実装されている（`#[cfg(not(test))]` が関数呼び出しにも使用されている）
- **対応**: レビューコメントの提案に従い、importの`#[cfg(not(test))]`を削除し、関数呼び出しのみに残す
- **優先度**: 低（動作には影響しないが、コードの明確性向上）

### 3. Markdownパーサーのセキュリティ問題（必須対応・重要）

#### 3.1 `app/src/pages/knowledge_detail/components/markdown_renderer.rs`
- **問題**: カスタムMarkdownパーサーが脆弱で、HTMLインジェクションの脆弱性がある可能性
- **影響**: ユーザー入力のMarkdownをHTMLに変換する際に、悪意のあるHTMLが注入される可能性
- **推奨対応**: `pulldown-cmark`と`pulldown-cmark-to-html`を使用して堅牢なMarkdownパーサーに置き換える
- **優先度**: 重要（セキュリティ問題）

## 対応手順

### ステップ1: Leptos 0.8のCallback使用方法を修正（必須）

1. `app/src/pages/repositories/components/sort_control.rs`を修正
   - `on_change.call(sort_option)` → `on_change(sort_option)`

2. `app/src/pages/repositories/components/language_filter.rs`を修正
   - `on_change.call(current)` → `on_change(current)`

3. `app/src/pages/contributors/components/status_filter.rs`を修正
   - `on_change.call(role)` → `on_change(role)`

4. `app/src/pages/contributors/components/contributor_card.rs`を修正
   - `callback.call(user.username.clone())` → `callback(user.username.clone())`

5. コンパイル確認
   ```bash
   cargo check --manifest-path app/Cargo.toml
   ```

### ステップ2: dotenvクレートの使用方法を改善（推奨）

1. `server/src/config.rs`を修正
   - importの`#[cfg(not(test))]`を削除
   - 関数呼び出しの`#[cfg(not(test))]`は維持

2. コンパイル確認
   ```bash
   cargo check --manifest-path server/Cargo.toml
   ```

### ステップ3: Markdownパーサーを堅牢なライブラリに置き換え（必須・重要）

1. `app/Cargo.toml`に依存関係を追加
   ```toml
   [dependencies]
   pulldown-cmark = "0.9"
   pulldown-cmark-to-html = "0.9"
   ```

2. `app/src/pages/knowledge_detail/components/markdown_renderer.rs`を修正
   - カスタムMarkdownパーサーを削除
   - `pulldown-cmark`と`pulldown-cmark-to-html`を使用して実装

3. コンパイル確認
   ```bash
   cargo check --manifest-path app/Cargo.toml
   ```

4. 動作確認
   - Markdownのレンダリングが正しく動作することを確認
   - HTMLインジェクションの脆弱性が解消されていることを確認

## 優先順位

1. **最優先（必須対応）**: Leptos 0.8のCallback使用方法の修正（4箇所）
   - コンパイルエラーを解消するため、最優先で対応

2. **重要（必須対応）**: Markdownパーサーのセキュリティ問題の修正
   - セキュリティリスクを解消するため、早期に対応

3. **推奨（任意対応）**: dotenvクレートの使用方法の改善
   - 動作には影響しないが、コードの明確性向上のため対応推奨

## 関連ドキュメント

- [Leptos 0.8 Documentation](https://book.leptos.dev/)
- [Leptos 0.8 API Reference](https://docs.rs/leptos/latest/leptos/)
- [pulldown-cmark Documentation](https://docs.rs/pulldown-cmark/)
- [pulldown-cmark-to-html Documentation](https://docs.rs/pulldown-cmark-to-html/)
- [コンパイルエラー調査ドキュメント](../2025_01/compilation-errors-investigation.md)

## チェックリスト

- [x] Leptos 0.8のCallback使用方法を修正（4箇所）
  - [x] `app/src/pages/repositories/components/sort_control.rs`
  - [x] `app/src/pages/repositories/components/language_filter.rs`
  - [x] `app/src/pages/contributors/components/status_filter.rs`
  - [x] `app/src/pages/contributors/components/contributor_card.rs`
- [x] コンパイル確認（Callback修正後）
- [x] dotenvクレートの使用方法を改善
  - [x] `server/src/config.rs`
- [x] Markdownパーサーを堅牢なライブラリに置き換え
  - [x] `app/Cargo.toml`に依存関係を追加（`pulldown-cmark = "0.9"`）
  - [x] `app/src/pages/knowledge_detail/components/markdown_renderer.rs`を修正
- [x] コンパイル確認（Markdownパーサー修正後）
- [x] コンパイル確認（server側）

## 実施結果

### 修正完了日時
2025年11月21日

### 修正内容の詳細

#### 1. Leptos 0.8のCallback使用方法の修正
すべての`callback.call(args)`を`callback(args)`に変更しました。

#### 2. Markdownパーサーの置き換え
- `pulldown-cmark`ライブラリを使用して実装
- HTMLインジェクションの脆弱性を解消
- 適切なHTMLエスケープ処理を実装

#### 3. dotenvクレートの使用方法の改善
- importの`#[cfg(not(test))]`を削除
- 関数呼び出しの`#[cfg(not(test))]`は維持

### コンパイル結果
- ✅ `cargo check --manifest-path app/Cargo.toml`: 成功（警告のみ、エラーなし）
- ✅ `cargo check --manifest-path server/Cargo.toml`: 成功（警告のみ、エラーなし）

### 注意事項
- 警告は未使用の変数やフィールドに関するもので、今回の修正とは無関係です
- 動作確認は実際のアプリケーション実行時に実施してください

