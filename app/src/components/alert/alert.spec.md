# Alert Component Specification

## Related Files

- Implementation: `app/src/components/alert/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/alert/

## Requirements

### 責務

- アラート/通知メッセージの表示と管理
- DaisyUIの`alert`クラスを使用したスタイリング
- AlertTitleとAlertDescriptionコンポーネントとの連携

### Props構造

#### Alertコンポーネント

- `variant` (optional, AlertVariant): スタイルバリエーション（デフォルト: Info）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): アラートの子要素（AlertTitle, AlertDescriptionなど）

#### AlertTitleコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): タイトルのテキスト

#### AlertDescriptionコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): 説明文のテキスト

### Variant

以下のバリアントをサポート：

- `Info`: 情報メッセージ（デフォルト）
- `Success`: 成功メッセージ
- `Warning`: 警告メッセージ
- `Error`: エラーメッセージ

### スタイル

- `Alert`: `alert`クラスとバリアントクラス（`alert-info`, `alert-success`, `alert-warning`, `alert-error`）を適用
- `AlertTitle`: `font-bold`クラスを適用（デフォルト）
- `AlertDescription`: `text-sm`クラスを適用（デフォルト）

## Test Cases

### TC-001: Alertの基本表示

- Given: variantが指定されていない
- When: Alertコンポーネントをレンダリング
- Then: `alert alert-info`クラスが適用された`<div role="alert">`要素が表示される

### TC-002: Infoバリアントの適用

- Given: variant=AlertVariant::Info
- When: Alertコンポーネントをレンダリング
- Then: `alert alert-info`クラスが適用される

### TC-003: Successバリアントの適用

- Given: variant=AlertVariant::Success
- When: Alertコンポーネントをレンダリング
- Then: `alert alert-success`クラスが適用される

### TC-004: Warningバリアントの適用

- Given: variant=AlertVariant::Warning
- When: Alertコンポーネントをレンダリング
- Then: `alert alert-warning`クラスが適用される

### TC-005: Errorバリアントの適用

- Given: variant=AlertVariant::Error
- When: Alertコンポーネントをレンダリング
- Then: `alert alert-error`クラスが適用される

### TC-006: AlertTitleの表示

- Given: AlertTitleコンポーネントをレンダリング
- When: ページを表示
- Then: `font-bold`クラスが適用された`<h3>`要素が表示される

### TC-007: AlertDescriptionの表示

- Given: AlertDescriptionコンポーネントをレンダリング
- When: ページを表示
- Then: `text-sm`クラスが適用された`<div>`要素が表示される

### TC-008: Alertのカスタムクラス

- Given: Alertにclass="mb-4"を設定
- When: Alertコンポーネントをレンダリング
- Then: ベースクラスに加えて"mb-4"が追加される

### TC-009: Alert構造の組み合わせ

- Given: Alert内にAlertTitleとAlertDescriptionを配置
- When: Alertコンポーネントをレンダリング
- Then: AlertTitleとAlertDescriptionが正しく表示される

