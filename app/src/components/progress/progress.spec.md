# Progress Component Specification

## Related Files

- Implementation: `app/src/components/progress/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/progress/

## Requirements

### 責務

- プログレスバーの表示と管理
- DaisyUIの`progress`クラスを使用したスタイリング
- 進捗状況の視覚的な表示

### Props構造

- `value` (u32): 現在の値（0-maxの範囲）
- `max` (optional, u32): 最大値（デフォルト: 100）
- `variant` (optional, ProgressVariant): スタイルバリエーション（デフォルト: Primary）
- `class` (optional, String): 追加のCSSクラス

### Variant

以下のバリアントをサポート：

- `Primary`: プライマリカラー（デフォルト）
- `Success`: 成功カラー
- `Warning`: 警告カラー
- `Error`: エラーカラー

### スタイル

- `Progress`: `progress`クラスとバリアントクラス（`progress-primary`, `progress-success`, `progress-warning`, `progress-error`）を適用
- CSS変数`--value`を使用して進捗率を設定

## Test Cases

### TC-001: Progressの基本表示

- Given: value=50, maxが指定されていない
- When: Progressコンポーネントをレンダリング
- Then: `progress progress-primary`クラスが適用された`<progress>`要素が表示され、value=50, max=100が設定される

### TC-002: Primaryバリアントの適用

- Given: variant=ProgressVariant::Primary
- When: Progressコンポーネントをレンダリング
- Then: `progress progress-primary`クラスが適用される

### TC-003: Successバリアントの適用

- Given: variant=ProgressVariant::Success
- When: Progressコンポーネントをレンダリング
- Then: `progress progress-success`クラスが適用される

### TC-004: Warningバリアントの適用

- Given: variant=ProgressVariant::Warning
- When: Progressコンポーネントをレンダリング
- Then: `progress progress-warning`クラスが適用される

### TC-005: Errorバリアントの適用

- Given: variant=ProgressVariant::Error
- When: Progressコンポーネントをレンダリング
- Then: `progress progress-error`クラスが適用される

### TC-006: カスタムmax値の設定

- Given: value=30, max=50
- When: Progressコンポーネントをレンダリング
- Then: value=30, max=50が設定され、進捗率は60%となる

### TC-007: 値のクランプ

- Given: value=150, max=100
- When: Progressコンポーネントをレンダリング
- Then: valueは100にクランプされ、進捗率は100%となる

### TC-008: カスタムクラスの追加

- Given: class="w-full"
- When: Progressコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full"が追加される

### TC-009: 進捗率の計算

- Given: value=75, max=100
- When: Progressコンポーネントをレンダリング
- Then: CSS変数`--value`が75%に設定される

