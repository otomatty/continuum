# Input Component Specification

## Related Files

- Implementation: `app/src/components/input.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/input/

## Requirements

### 責務

- テキスト入力フィールドの表示と管理
- DaisyUIの`input`クラスを使用したスタイリング
- リアクティブな値のバインディングとイベントハンドリング

### Props構造

- `variant` (optional, InputVariant): スタイルバリエーション（デフォルト: Bordered）
- `placeholder` (optional, String): プレースホルダーテキスト
- `value` (optional, ReadSignal<String>): 入力値のSignal
- `on_input` (optional, Callback<InputEvent>): 入力イベントのコールバック
- `class` (optional, String): 追加のCSSクラス

### Variant

以下のバリアントをサポート：

- `Primary`: プライマリカラー
- `Secondary`: セカンダリカラー
- `Accent`: アクセントカラー
- `Error`: エラー状態
- `Success`: 成功状態
- `Warning`: 警告状態
- `Info`: 情報表示
- `Ghost`: ゴーストスタイル
- `Bordered`: ボーダー付き（デフォルト）

### イベントハンドラー

- `on_input`: ユーザーが入力した際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのInput表示

- Given: variantが指定されていない
- When: Inputコンポーネントをレンダリング
- Then: `input input-bordered`クラスが適用される

### TC-002: Primaryバリアントの適用

- Given: variant=InputVariant::Primary
- When: Inputコンポーネントをレンダリング
- Then: `input input-primary`クラスが適用される

### TC-003: プレースホルダーの表示

- Given: placeholder="テキストを入力"
- When: Inputコンポーネントをレンダリング
- Then: placeholder属性に"テキストを入力"が設定される

### TC-004: 値のバインディング

- Given: value=ReadSignal<String>で初期値"test"
- When: Inputコンポーネントをレンダリング
- Then: value属性に"test"が設定される

### TC-005: カスタムクラスの追加

- Given: class="w-full max-w-xs"
- When: Inputコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full max-w-xs"が追加される

### TC-006: 入力イベントのハンドリング

- Given: on_input=Callbackで値を更新するハンドラー
- When: ユーザーが入力フィールドにテキストを入力
- Then: on_inputコールバックが呼び出される

