# Checkbox Component Specification

## Related Files

- Implementation: `app/src/components/checkbox/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/checkbox/

## Requirements

### 責務

- チェックボックスの表示と管理
- DaisyUIの`checkbox`クラスを使用したスタイリング
- リアクティブなチェック状態のバインディングとイベントハンドリング

### Props構造

- `variant` (optional, CheckboxVariant): スタイルバリエーション（デフォルト: Primary）
- `checked` (optional, ReadSignal<bool>): チェック状態のSignal
- `on_change` (optional, Callback<ChangeEvent>): 変更イベントのコールバック
- `class` (optional, String): 追加のCSSクラス

### Variant

以下のバリアントをサポート：

- `Primary`: プライマリカラー（デフォルト）
- `Secondary`: セカンダリカラー
- `Accent`: アクセントカラー
- `Success`: 成功状態
- `Warning`: 警告状態
- `Error`: エラー状態
- `Info`: 情報表示

### イベントハンドラー

- `on_change`: ユーザーがチェック状態を変更した際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのCheckbox表示

- Given: variantが指定されていない
- When: Checkboxコンポーネントをレンダリング
- Then: `checkbox checkbox-primary`クラスが適用される

### TC-002: Successバリアントの適用

- Given: variant=CheckboxVariant::Success
- When: Checkboxコンポーネントをレンダリング
- Then: `checkbox checkbox-success`クラスが適用される

### TC-003: チェック状態の表示

- Given: checked=ReadSignal<bool>でtrue
- When: Checkboxコンポーネントをレンダリング
- Then: checked属性がtrueに設定される

### TC-004: 未チェック状態の表示

- Given: checked=ReadSignal<bool>でfalse
- When: Checkboxコンポーネントをレンダリング
- Then: checked属性がfalseに設定される

### TC-005: カスタムクラスの追加

- Given: class="w-6 h-6"
- When: Checkboxコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-6 h-6"が追加される

### TC-006: 変更イベントのハンドリング

- Given: on_change=Callbackで状態を更新するハンドラー
- When: ユーザーがチェックボックスをクリック
- Then: on_changeコールバックが呼び出される

