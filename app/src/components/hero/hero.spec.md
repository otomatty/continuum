# Hero Component Specification

## Related Files

- Implementation: `app/src/components/hero/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/hero/

## Requirements

### 責務

- ヒーローセクションの表示
- DaisyUIの`hero`クラスを使用したスタイリング
- HeroContentとHeroOverlayコンポーネントとの連携
- ランディングページのメインセクション

### Props構造

#### Heroコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ヒーローの子要素（HeroContent, HeroOverlay）

#### HeroContentコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): コンテンツの要素

#### HeroOverlayコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): オーバーレイの要素

### スタイル

- `Hero`: `hero`クラスを適用
- `HeroContent`: `hero-content`クラスを適用
- `HeroOverlay`: `hero-overlay`クラスを適用

## Test Cases

### TC-001: Heroの基本表示

- Given: Heroコンポーネントをレンダリング
- When: ページを表示
- Then: `hero`クラスが適用された`<div>`要素が表示される

### TC-002: HeroContentの表示

- Given: HeroContentコンポーネントをレンダリング
- When: ページを表示
- Then: `hero-content`クラスが適用された`<div>`要素が表示される

### TC-003: HeroOverlayの表示

- Given: HeroOverlayコンポーネントをレンダリング
- When: ページを表示
- Then: `hero-overlay`クラスが適用された`<div>`要素が表示される

### TC-004: Heroのカスタムクラス

- Given: Heroにclass="hero-min-h-screen"を設定
- When: Heroコンポーネントをレンダリング
- Then: ベースクラスに加えて"hero-min-h-screen"が追加される

### TC-005: HeroContentのカスタムクラス

- Given: HeroContentにclass="text-center"を設定
- When: HeroContentコンポーネントをレンダリング
- Then: ベースクラスに加えて"text-center"が追加される

### TC-006: HeroOverlayのカスタムクラス

- Given: HeroOverlayにclass="bg-opacity-60"を設定
- When: HeroOverlayコンポーネントをレンダリング
- Then: ベースクラスに加えて"bg-opacity-60"が追加される

### TC-007: Hero構造の組み合わせ

- Given: Hero内にHeroOverlayとHeroContentを配置
- When: Heroコンポーネントをレンダリング
- Then: HeroOverlayとHeroContentが正しく表示される

### TC-008: ランディングページの使用

- Given: Hero内にHeroOverlayとHeroContent（タイトル、説明、ボタンを含む）を配置
- When: Heroコンポーネントをレンダリング
- Then: ランディングページのメインセクションとして正しく表示される

