# Radio Component Specification

## Related Files

- Implementation: `app/src/components/radio.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/radio/

## Requirements

### 責務

- ラジオボタンの表示と管理
- DaisyUIの`radio`クラスを使用したスタイリング
- リアクティブな選択状態のバインディングとイベントハンドリング
- 同じname属性を持つラジオボタン間での排他的選択

### Props構造

- `name` (required, String): ラジオボタングループの名前（必須）
- `value` (required, String): ラジオボタンの値（必須）
- `variant` (optional, RadioVariant): スタイルバリエーション（デフォルト: Primary）
- `checked` (optional, ReadSignal<bool>): 選択状態のSignal
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

- `on_change`: ユーザーがラジオボタンを選択した際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのRadio表示

- Given: variantが指定されていない、name="option", value="value1"
- When: Radioコンポーネントをレンダリング
- Then: `radio radio-primary`クラスが適用され、name="option", value="value1"が設定される

### TC-002: Successバリアントの適用

- Given: variant=RadioVariant::Success, name="option", value="value1"
- When: Radioコンポーネントをレンダリング
- Then: `radio radio-success`クラスが適用される

### TC-003: 選択状態の表示

- Given: checked=ReadSignal<bool>でtrue, name="option", value="value1"
- When: Radioコンポーネントをレンダリング
- Then: checked属性がtrueに設定される

### TC-004: 未選択状態の表示

- Given: checked=ReadSignal<bool>でfalse, name="option", value="value1"
- When: Radioコンポーネントをレンダリング
- Then: checked属性がfalseに設定される

### TC-005: 同じname属性での排他的選択

- Given: 同じname="option"を持つ複数のRadioコンポーネント
- When: 1つのRadioを選択
- Then: 他のRadioの選択が解除される（ブラウザの標準動作）

### TC-006: カスタムクラスの追加

- Given: class="w-6 h-6", name="option", value="value1"
- When: Radioコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-6 h-6"が追加される

### TC-007: 変更イベントのハンドリング

- Given: on_change=Callbackで状態を更新するハンドラー, name="option", value="value1"
- When: ユーザーがラジオボタンをクリック
- Then: on_changeコールバックが呼び出される

