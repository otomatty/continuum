# Divider Component Specification

## Related Files

- Implementation: `app/src/components/divider/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/divider/

## Requirements

### 責務

- 区切り線の表示と管理
- DaisyUIの`divider`クラスを使用したスタイリング
- セクション間の区切り表示

### Props構造

- `text` (optional, String): 区切り線中央に表示するテキスト
- `class` (optional, String): 追加のCSSクラス

### スタイル

- `Divider`: `divider`クラスを適用
- テキストが指定された場合、区切り線の中央に表示される

## Test Cases

### TC-001: Dividerの基本表示

- Given: textとclassが指定されていない
- When: Dividerコンポーネントをレンダリング
- Then: `divider`クラスが適用された`<div>`要素が表示される

### TC-002: テキスト付きDividerの表示

- Given: text="OR"
- When: Dividerコンポーネントをレンダリング
- Then: `divider`クラスが適用され、中央に"OR"が表示される

### TC-003: カスタムクラスの追加

- Given: class="my-8"
- When: Dividerコンポーネントをレンダリング
- Then: ベースクラスに加えて"my-8"が追加される

### TC-004: テキストなしの表示

- Given: textが指定されていない
- When: Dividerコンポーネントをレンダリング
- Then: 空の`<div>`要素が表示される

### TC-005: マージンのカスタマイズ

- Given: class="my-12"
- When: Dividerコンポーネントをレンダリング
- Then: ベースクラスに加えて"my-12"が追加され、上下マージンが設定される

### TC-006: 水平方向の配置

- Given: text="Section", class="divider-horizontal"
- When: Dividerコンポーネントをレンダリング
- Then: 水平方向の区切り線として表示される

