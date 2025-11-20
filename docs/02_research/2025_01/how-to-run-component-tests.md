# UIコンポーネントのテスト実行方法

## 概要

UIコンポーネントのテストを単体で実行する方法を説明します。

## テストの実行方法

### 1. すべてのテストを実行

```bash
# プロジェクト全体のテストを実行
cargo test

# ライブラリクレート（app）のテストのみ実行
cargo test --package app --lib
```

### 2. 特定のコンポーネントのテストのみ実行

```bash
# Accordionコンポーネントのテストのみ実行
cargo test --package app --lib components::accordion::tests

# より具体的に、テストモジュールを指定
cargo test --package app --lib components::accordion::tests::test_accordion_variant_default
```

### 3. 特定のテスト関数のみ実行

```bash
# テスト関数名でフィルタリング
cargo test --package app --lib test_accordion_variant_default

# パターンマッチングで複数のテストを実行
cargo test --package app --lib accordion
```

### 4. テストの詳細出力を表示

```bash
# テストの詳細な出力を表示（--nocapture）
cargo test --package app --lib components::accordion::tests -- --nocapture

# テストの実行時間も表示
cargo test --package app --lib components::accordion::tests -- --show-output
```

### 5. テストを実行せずにコンパイルのみ確認

```bash
# テストがコンパイルできるか確認（実行はしない）
cargo test --package app --lib components::accordion::tests --no-run
```

## テスト実行の例

### Accordionコンポーネントのテストを実行

```bash
# Accordionコンポーネントのすべてのテストを実行
cargo test --package app --lib components::accordion::tests

# 実行結果の例:
# running 9 tests
# test components::accordion::tests::test_accordion_variant_default ... ok
# test components::accordion::tests::test_accordion_variant_arrow ... ok
# test components::accordion::tests::test_accordion_variant_plus ... ok
# test components::accordion::tests::test_get_variant_class_arrow ... ok
# test components::accordion::tests::test_get_variant_class_plus ... ok
# test components::accordion::tests::test_combine_classes_empty_custom ... ok
# test components::accordion::tests::test_combine_classes_with_custom ... ok
# test components::accordion::tests::test_get_collapse_class_closed ... ok
# test components::accordion::tests::test_get_collapse_class_open ... ok
# test components::accordion::tests::test_get_collapse_class_open_with_custom ... ok
# test components::accordion::tests::test_get_content_class_default ... ok
# test components::accordion::tests::test_get_content_class_with_custom ... ok
#
# test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 特定のテストケースのみ実行

```bash
# variant関連のテストのみ実行
cargo test --package app --lib components::accordion::tests variant

# 実行結果の例:
# running 5 tests
# test components::accordion::tests::test_accordion_variant_default ... ok
# test components::accordion::tests::test_accordion_variant_arrow ... ok
# test components::accordion::tests::test_accordion_variant_plus ... ok
# test components::accordion::tests::test_get_variant_class_arrow ... ok
# test components::accordion::tests::test_get_variant_class_plus ... ok
#
# test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 注意事項

### プロジェクト全体のコンパイルが必要

Rustのテストは、プロジェクト全体がコンパイルできる必要があります。特定のコンポーネントのテストだけを実行することはできますが、プロジェクト全体がコンパイルできる状態である必要があります。

### テストの独立性

各テストは独立して実行できますが、プロジェクト全体のコンパイルは必要です。これは、Rustのモジュールシステムの制約によるものです。

### テストの実行順序

テストは並列に実行されるため、テスト間で状態を共有しないように注意してください。各テストは独立して実行できるように設計されています。

## トラブルシューティング

### コンパイルエラーが発生する場合

```bash
# まず、プロジェクト全体がコンパイルできるか確認
cargo build --package app --lib

# コンパイルエラーを修正してから、テストを実行
cargo test --package app --lib components::accordion::tests
```

### テストが見つからない場合

```bash
# テストが正しく定義されているか確認
cargo test --package app --lib --list | grep accordion

# テストモジュールが正しくインポートされているか確認
# mod.rsに以下があることを確認:
# #[cfg(test)]
# mod tests;
```

## まとめ

UIコンポーネントのテストは、以下の方法で実行できます：

1. **すべてのテスト**: `cargo test`
2. **特定のコンポーネント**: `cargo test --package app --lib components::accordion::tests`
3. **特定のテスト関数**: `cargo test --package app --lib test_accordion_variant_default`
4. **パターンマッチング**: `cargo test --package app --lib accordion`

テストは独立して実行できますが、プロジェクト全体がコンパイルできる必要があります。

