# Stats Component Specification

## Related Files

- Implementation: `app/src/components/stats/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/stats/

## Requirements

### 責務

- 統計情報の表示
- DaisyUIの`stats`クラスを使用したスタイリング
- StatItem, StatTitle, StatValue, StatDescriptionコンポーネントとの連携
- 数値統計の表示

### Props構造

#### Statsコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): 統計の子要素（StatItem）

#### StatItemコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): 統計アイテムの子要素（StatTitle, StatValue, StatDescription）

#### StatTitleコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): タイトルのテキスト

#### StatValueコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): 値のテキスト

#### StatDescriptionコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): 説明文のテキスト

### スタイル

- `Stats`: `stats shadow`クラスを適用
- `StatItem`: `stat`クラスを適用
- `StatTitle`: `stat-title`クラスを適用
- `StatValue`: `stat-value`クラスを適用
- `StatDescription`: `stat-desc`クラスを適用

## Test Cases

### TC-001: Statsの基本表示

- Given: Statsコンポーネントをレンダリング
- When: ページを表示
- Then: `stats shadow`クラスが適用された`<div>`要素が表示される

### TC-002: StatItemの表示

- Given: StatItemコンポーネントをレンダリング
- When: ページを表示
- Then: `stat`クラスが適用された`<div>`要素が表示される

### TC-003: StatTitleの表示

- Given: StatTitleコンポーネントをレンダリング
- When: ページを表示
- Then: `stat-title`クラスが適用された`<div>`要素が表示される

### TC-004: StatValueの表示

- Given: StatValueコンポーネントをレンダリング
- When: ページを表示
- Then: `stat-value`クラスが適用された`<div>`要素が表示される

### TC-005: StatDescriptionの表示

- Given: StatDescriptionコンポーネントをレンダリング
- When: ページを表示
- Then: `stat-desc`クラスが適用された`<div>`要素が表示される

### TC-006: Statsのカスタムクラス

- Given: Statsにclass="stats-horizontal"を設定
- When: Statsコンポーネントをレンダリング
- Then: ベースクラスに加えて"stats-horizontal"が追加される

### TC-007: StatItemのカスタムクラス

- Given: StatItemにclass="stat-primary"を設定
- When: StatItemコンポーネントをレンダリング
- Then: ベースクラスに加えて"stat-primary"が追加される

### TC-008: Stats構造の組み合わせ

- Given: Stats内に複数のStatItem（各StatItemにStatTitle, StatValue, StatDescriptionを含む）を配置
- When: Statsコンポーネントをレンダリング
- Then: すべてのStatItemが正しく表示される

### TC-009: 統計情報の表示

- Given: StatItem内にStatTitle="総ユーザー数", StatValue="1,234", StatDescription="前月比+10%"を配置
- When: StatItemコンポーネントをレンダリング
- Then: 統計情報が正しく表示される

