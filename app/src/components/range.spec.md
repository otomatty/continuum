# Range Component Specification

## Related Files

- Implementation: `app/src/components/range.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/range/

## Requirements

### 責務

- レンジスライダーの表示と管理
- DaisyUIの`range`クラスを使用したスタイリング
- リアクティブな値のバインディングとイベントハンドリング
- 最小値、最大値、ステップ値の設定

### Props構造

- `min` (optional, f64): 最小値
- `max` (optional, f64): 最大値
- `step` (optional, f64): ステップ値
- `variant` (optional, RangeVariant): スタイルバリエーション（デフォルト: Primary）
- `value` (optional, ReadSignal<f64>): 現在値のSignal
- `on_input` (optional, Callback<InputEvent>): 入力イベントのコールバック
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

- `on_input`: ユーザーがスライダーを操作した際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのRange表示

- Given: variantが指定されていない
- When: Rangeコンポーネントをレンダリング
- Then: `range range-primary`クラスが適用される

### TC-002: Successバリアントの適用

- Given: variant=RangeVariant::Success
- When: Rangeコンポーネントをレンダリング
- Then: `range range-success`クラスが適用される

### TC-003: 最小値・最大値の設定

- Given: min=0.0, max=100.0
- When: Rangeコンポーネントをレンダリング
- Then: min属性に"0", max属性に"100"が設定される

### TC-004: ステップ値の設定

- Given: step=5.0
- When: Rangeコンポーネントをレンダリング
- Then: step属性に"5"が設定される

### TC-005: 値のバインディング

- Given: value=ReadSignal<f64>で初期値50.0
- When: Rangeコンポーネントをレンダリング
- Then: value属性に"50"が設定される

### TC-006: カスタムクラスの追加

- Given: class="w-full"
- When: Rangeコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full"が追加される

### TC-007: 入力イベントのハンドリング

- Given: on_input=Callbackで値を更新するハンドラー
- When: ユーザーがスライダーをドラッグ
- Then: on_inputコールバックが呼び出される

