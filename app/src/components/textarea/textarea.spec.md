# Textarea Component Specification

## Related Files

- Implementation: `app/src/components/textarea/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/textarea/

## Requirements

### 責務

- 複数行テキスト入力フィールドの表示と管理
- DaisyUIの`textarea`クラスを使用したスタイリング
- リアクティブな値のバインディングとイベントハンドリング

### Props構造

- `variant` (optional, TextareaVariant): スタイルバリエーション（デフォルト: Bordered）
- `placeholder` (optional, String): プレースホルダーテキスト
- `rows` (optional, u32): 表示行数
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

### TC-001: デフォルトのTextarea表示

- Given: variantが指定されていない
- When: Textareaコンポーネントをレンダリング
- Then: `textarea textarea-bordered`クラスが適用される

### TC-002: Primaryバリアントの適用

- Given: variant=TextareaVariant::Primary
- When: Textareaコンポーネントをレンダリング
- Then: `textarea textarea-primary`クラスが適用される

### TC-003: プレースホルダーの表示

- Given: placeholder="複数行のテキストを入力"
- When: Textareaコンポーネントをレンダリング
- Then: placeholder属性に"複数行のテキストを入力"が設定される

### TC-004: 行数の指定

- Given: rows=5
- When: Textareaコンポーネントをレンダリング
- Then: rows属性に"5"が設定される

### TC-005: 値のバインディング

- Given: value=ReadSignal<String>で初期値"test\ncontent"
- When: Textareaコンポーネントをレンダリング
- Then: value属性に"test\ncontent"が設定される

### TC-006: カスタムクラスの追加

- Given: class="w-full max-w-xs"
- When: Textareaコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full max-w-xs"が追加される

### TC-007: 入力イベントのハンドリング

- Given: on_input=Callbackで値を更新するハンドラー
- When: ユーザーがテキストエリアにテキストを入力
- Then: on_inputコールバックが呼び出される

