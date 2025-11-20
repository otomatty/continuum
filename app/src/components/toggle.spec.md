# Toggle Component Specification

## Related Files

- Implementation: `app/src/components/toggle.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/toggle/

## Requirements

### 責務

- トグルスイッチの表示と管理
- DaisyUIの`toggle`クラスを使用したスタイリング
- リアクティブなON/OFF状態のバインディングとイベントハンドリング

### Props構造

- `variant` (optional, ToggleVariant): スタイルバリエーション（デフォルト: Primary）
- `checked` (optional, ReadSignal<bool>): ON/OFF状態のSignal
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

- `on_change`: ユーザーがトグルスイッチを切り替えた際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのToggle表示

- Given: variantが指定されていない
- When: Toggleコンポーネントをレンダリング
- Then: `toggle toggle-primary`クラスが適用される

### TC-002: Successバリアントの適用

- Given: variant=ToggleVariant::Success
- When: Toggleコンポーネントをレンダリング
- Then: `toggle toggle-success`クラスが適用される

### TC-003: ON状態の表示

- Given: checked=ReadSignal<bool>でtrue
- When: Toggleコンポーネントをレンダリング
- Then: checked属性がtrueに設定される

### TC-004: OFF状態の表示

- Given: checked=ReadSignal<bool>でfalse
- When: Toggleコンポーネントをレンダリング
- Then: checked属性がfalseに設定される

### TC-005: カスタムクラスの追加

- Given: class="w-12 h-6"
- When: Toggleコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-12 h-6"が追加される

### TC-006: 変更イベントのハンドリング

- Given: on_change=Callbackで状態を更新するハンドラー
- When: ユーザーがトグルスイッチをクリック
- Then: on_changeコールバックが呼び出される

