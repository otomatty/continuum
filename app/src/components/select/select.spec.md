# Select Component Specification

## Related Files

- Implementation: `app/src/components/select/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/select/

## Requirements

### 責務

- セレクトボックスの表示と管理
- DaisyUIの`select`クラスを使用したスタイリング
- リアクティブな値のバインディングとイベントハンドリング
- SelectOptionコンポーネントとの連携

### Props構造

#### Selectコンポーネント

- `variant` (optional, SelectVariant): スタイルバリエーション（デフォルト: Bordered）
- `value` (optional, ReadSignal<String>): 選択値のSignal
- `on_change` (optional, Callback<ChangeEvent>): 変更イベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): SelectOptionコンポーネントの子要素

#### SelectOptionコンポーネント

- `disabled` (optional, bool): 無効化フラグ
- `selected` (optional, bool): 選択状態フラグ
- `value` (optional, String): オプションの値
- `children` (Children): オプションの表示テキスト

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

- `on_change`: ユーザーが選択を変更した際に発火するイベントハンドラー

## Test Cases

### TC-001: デフォルトのSelect表示

- Given: variantが指定されていない
- When: Selectコンポーネントをレンダリング
- Then: `select select-bordered`クラスが適用される

### TC-002: Primaryバリアントの適用

- Given: variant=SelectVariant::Primary
- When: Selectコンポーネントをレンダリング
- Then: `select select-primary`クラスが適用される

### TC-003: SelectOptionの追加

- Given: Selectコンポーネント内にSelectOptionを配置
- When: Selectコンポーネントをレンダリング
- Then: SelectOptionが子要素として表示される

### TC-004: SelectOptionの無効化

- Given: SelectOptionにdisabled=trueを設定
- When: SelectOptionコンポーネントをレンダリング
- Then: disabled属性が設定される

### TC-005: SelectOptionの選択状態

- Given: SelectOptionにselected=trueを設定
- When: SelectOptionコンポーネントをレンダリング
- Then: selected属性が設定される

### TC-006: 値のバインディング

- Given: value=ReadSignal<String>で初期値"option1"
- When: Selectコンポーネントをレンダリング
- Then: value属性に"option1"が設定される

### TC-007: カスタムクラスの追加

- Given: class="w-full max-w-xs"
- When: Selectコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full max-w-xs"が追加される

### TC-008: 変更イベントのハンドリング

- Given: on_change=Callbackで値を更新するハンドラー
- When: ユーザーがセレクトボックスの選択を変更
- Then: on_changeコールバックが呼び出される

