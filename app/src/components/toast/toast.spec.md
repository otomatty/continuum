# Toast Component Specification

## Related Files

- Implementation: `app/src/components/toast/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/toast/

## Requirements

### 責務

- トースト通知の表示と管理
- DaisyUIの`toast`と`alert`クラスを使用したスタイリング
- 一時的な通知メッセージの表示
- 自動的に閉じる機能（オプション）

### Props構造

#### Toastコンポーネント

- `message` (String): トーストのメッセージ（必須）
- `variant` (optional, ToastVariant): スタイルバリエーション（デフォルト: Info）
- `duration` (optional, u32): 表示時間（秒、指定されない場合は自動で閉じない）
- `on_close` (optional, Callback<()>): 閉じるイベントのコールバック
- `class` (optional, String): 追加のCSSクラス

### Variant

以下のバリアントをサポート：

- `Info`: 情報メッセージ（デフォルト）
- `Success`: 成功メッセージ
- `Warning`: 警告メッセージ
- `Error`: エラーメッセージ

### スタイル

- `Toast`: `toast alert`クラスとバリアントクラス（`alert-info`, `alert-success`, `alert-warning`, `alert-error`）を適用
- 閉じるボタン: `btn btn-sm btn-circle`クラスを適用（on_closeが指定されている場合）

### 機能

- メッセージの表示
- `duration`が指定されている場合、指定時間後に自動的に`on_close`コールバックが呼び出される
- `on_close`が指定されている場合、閉じるボタンが表示される

## Test Cases

### TC-001: Toastの基本表示

- Given: message="通知メッセージ", variantが指定されていない
- When: Toastコンポーネントをレンダリング
- Then: `toast alert alert-info`クラスが適用され、メッセージが表示される

### TC-002: Infoバリアントの適用

- Given: variant=ToastVariant::Info
- When: Toastコンポーネントをレンダリング
- Then: `toast alert alert-info`クラスが適用される

### TC-003: Successバリアントの適用

- Given: variant=ToastVariant::Success
- When: Toastコンポーネントをレンダリング
- Then: `toast alert alert-success`クラスが適用される

### TC-004: Warningバリアントの適用

- Given: variant=ToastVariant::Warning
- When: Toastコンポーネントをレンダリング
- Then: `toast alert alert-warning`クラスが適用される

### TC-005: Errorバリアントの適用

- Given: variant=ToastVariant::Error
- When: Toastコンポーネントをレンダリング
- Then: `toast alert alert-error`クラスが適用される

### TC-006: メッセージの表示

- Given: message="これはトーストメッセージです"
- When: Toastコンポーネントをレンダリング
- Then: "これはトーストメッセージです"が表示される

### TC-007: 閉じるボタンの表示

- Given: Toastにon_closeコールバックを設定
- When: Toastコンポーネントをレンダリング
- Then: 閉じるボタン（✕）が表示される

### TC-008: 閉じるボタンのクリックイベント

- Given: Toastにon_closeコールバックを設定
- When: ユーザーが閉じるボタンをクリック
- Then: on_closeコールバックが呼び出される

### TC-009: 自動的に閉じる機能

- Given: Toastにduration=5とon_closeコールバックを設定
- When: Toastコンポーネントをレンダリング
- Then: 5秒後にon_closeコールバックが自動的に呼び出される

### TC-010: Toastのカスタムクラス

- Given: Toastにclass="toast-top"を設定
- When: Toastコンポーネントをレンダリング
- Then: ベースクラスに加えて"toast-top"が追加される

### TC-011: 閉じるボタンなしのToast

- Given: Toastにon_closeを設定しない
- When: Toastコンポーネントをレンダリング
- Then: 閉じるボタンが表示されない

