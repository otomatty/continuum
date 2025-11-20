# Badge Component Specification

## Related Files

- Implementation: `app/src/components/badge/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/badge/

## Requirements

### 責務

- バッジの表示と管理
- DaisyUIの`badge`クラスを使用したスタイリング
- 様々なバリアントによる視覚的な分類表示

### Props構造

- `variant` (required, BadgeVariant): スタイルバリエーション（必須）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): バッジの表示テキスト

### Variant

以下のバリアントをサポート：

- `Primary`: プライマリカラー
- `Secondary`: セカンダリカラー
- `Accent`: アクセントカラー
- `Success`: 成功状態
- `Warning`: 警告状態
- `Error`: エラー状態
- `Info`: 情報表示
- `Ghost`: ゴーストスタイル

### 注意事項

- `variant`プロパティは必須です（optionalではありません）

## Test Cases

### TC-001: Primaryバリアントの表示

- Given: variant=BadgeVariant::Primary
- When: Badgeコンポーネントをレンダリング
- Then: `badge badge-primary`クラスが適用される

### TC-002: Successバリアントの表示

- Given: variant=BadgeVariant::Success
- When: Badgeコンポーネントをレンダリング
- Then: `badge badge-success`クラスが適用される

### TC-003: Errorバリアントの表示

- Given: variant=BadgeVariant::Error
- When: Badgeコンポーネントをレンダリング
- Then: `badge badge-error`クラスが適用される

### TC-004: 子要素の表示

- Given: children="New"
- When: Badgeコンポーネントをレンダリング
- Then: バッジ内に"New"が表示される

### TC-005: カスタムクラスの追加

- Given: class="badge-lg"
- When: Badgeコンポーネントをレンダリング
- Then: ベースクラスに加えて"badge-lg"が追加される

### TC-006: すべてのバリアントの確認

- Given: 各バリアント（Primary, Secondary, Accent, Success, Warning, Error, Info, Ghost）を設定
- When: Badgeコンポーネントをレンダリング
- Then: それぞれのバリアントに対応するクラスが適用される

