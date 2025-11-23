# CIチェックガイド

このドキュメントでは、ローカルでCIと同じチェックを実行する方法を説明します。

## 現在のCIで実行されているチェック

GitHub ActionsのCI（`.github/workflows/ci.yml`）では、以下のチェックが実行されます：

1. **フォーマットチェック**: `cargo fmt --check --all`
   - コードがRustの標準フォーマットに従っているか確認

2. **Clippyチェック**: `cargo clippy --all-targets -- -D warnings`
   - コードの品質チェック（警告をエラーとして扱う）

3. **テスト実行**: `cargo test --all`
   - すべてのテストがパスするか確認

4. **ビルドチェック**: `cargo check --all`
   - コードがコンパイル可能か確認

## ローカルでCIチェックを実行する方法

### 方法1: CIチェックスクリプトを使用（推奨）

すべてのCIチェックを一括で実行：

```bash
# スクリプトを直接実行
./scripts/ci-check.sh

# または bun を使用
bun run ci:check
```

このスクリプトは以下のチェックを順番に実行します：
1. フォーマットチェック
2. Clippyチェック
3. テスト実行
4. ビルドチェック

すべてのチェックがパスすると、コミット可能な状態であることが確認できます。

### 方法2: 高速チェック（フォーマット + Clippyのみ）

テストとビルドチェックをスキップして、フォーマットとClippyのみを実行：

```bash
bun run ci:check:fast
```

これはコミット前のクイックチェックに便利です。

### 方法3: 個別のコマンドを実行

各チェックを個別に実行することもできます：

```bash
# フォーマットチェック
cargo fmt --check --all

# Clippyチェック
cargo clippy --all-targets -- -D warnings

# テスト実行
cargo test --all

# ビルドチェック
cargo check --all
```

## Pre-commitフックの設定

Git hooksをインストールすると、コミット前に自動的にチェックが実行されます：

```bash
# Git hooksをインストール
bun run install:hooks
```

インストール後、`git commit`を実行すると、以下のチェックが自動的に実行されます：

1. **ブランチ保護チェック**: main/developブランチへの直接コミットを防止
2. **フォーマットチェック**: コードがフォーマットされているか確認
3. **Clippyチェック**: コードの品質を確認

### Pre-commitフックをスキップする場合

緊急時など、pre-commitフックをスキップしたい場合は：

```bash
SKIP_CI_CHECK=1 git commit -m "your message"
```

または、`--no-verify`フラグを使用：

```bash
git commit --no-verify -m "your message"
```

## 推奨ワークフロー

### コミット前の推奨手順

1. **コードを変更**
2. **高速チェックを実行**（オプション）
   ```bash
   bun run check:fast
   ```
3. **コミット**
   ```bash
   git add .
   git commit -m "your message"
   ```
   - Pre-commitフックが自動的にチェックを実行します

### PR作成前の推奨手順

1. **すべてのCIチェックを実行**
   ```bash
   bun run check
   ```
2. **すべてのチェックがパスすることを確認**
3. **PRを作成**

### チェックスクリプトのオプション

`check-all.sh`スクリプトには以下のオプションがあります：

```bash
# ヘルプを表示
./scripts/check-all.sh --help

# フォーマットチェックのみ
./scripts/check-all.sh --format

# Clippyチェックのみ
./scripts/check-all.sh --clippy

# テストのみ
./scripts/check-all.sh --test

# ビルドチェックのみ
./scripts/check-all.sh --build

# テストをスキップ
./scripts/check-all.sh --skip-test

# PRブランチルールチェックを含める
./scripts/check-all.sh --pr-check
```

## トラブルシューティング

### フォーマットエラーが発生した場合

```bash
# 自動フォーマットを適用
cargo fmt --all
```

### Clippyエラーが発生した場合

```bash
# 詳細なエラーを確認
cargo clippy --all-targets -- -D warnings

# 自動修正可能な問題を修正（注意：すべての問題が修正されるわけではありません）
cargo clippy --fix --all-targets
```

### テストが失敗した場合

```bash
# 特定のパッケージのテストのみ実行
cargo test -p app
cargo test -p server
cargo test -p frontend

# 詳細な出力でテストを実行
cargo test --all -- --nocapture
```

## 参考

- [Rust公式ドキュメント - rustfmt](https://github.com/rust-lang/rustfmt)
- [Rust公式ドキュメント - Clippy](https://github.com/rust-lang/rust-clippy)
- [Git Hooks公式ドキュメント](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)

