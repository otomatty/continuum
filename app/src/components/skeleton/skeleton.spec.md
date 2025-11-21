# Skeleton Component Specification

## Related Files

- Implementation: `app/src/components/skeleton/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/skeleton/

## Requirements

### 責務

- スケルトンローディングの表示と管理
- DaisyUIの`skeleton`クラスを使用したスタイリング
- コンテンツ読み込み中のプレースホルダー表示
- `skeleton-text`修飾子による、テキスト色のアニメーション表示

### Props構造

- `class` (optional, String): 追加のCSSクラス（サイズ、形状などの調整用）
- `text` (optional, String): テキストコンテンツ（指定時は`skeleton-text`修飾子を適用）

### スタイル

- **通常のSkeleton**: `skeleton`クラスを適用した`<div>`要素（背景色をアニメーション化）
- **skeleton-text**: `skeleton skeleton-text`クラスを適用した`<span>`要素（テキスト色をアニメーション化）
- 幅や高さは`class`プロパティでTailwindユーティリティクラスを使用してカスタマイズ可能（例: `h-32 w-32`, `rounded-full`）

## Test Cases

### TC-001: Skeletonの基本表示

- Given: classとtextが指定されていない
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton`クラスが適用された空の`<div>`要素が表示される

### TC-002: カスタムクラスの追加

- Given: class="h-32 w-32"
- When: Skeletonコンポーネントをレンダリング
- Then: ベースクラスに加えて"h-32 w-32"が追加される（`skeleton h-32 w-32`）

### TC-003: 円形のスケルトン

- Given: class="h-16 w-16 shrink-0 rounded-full"
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton h-16 w-16 shrink-0 rounded-full`クラスが適用され、円形のプレースホルダーが表示される

### TC-004: 幅と高さのカスタマイズ

- Given: class="h-8 w-full"
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton h-8 w-full`クラスが適用され、フル幅のプレースホルダーが表示される

### TC-005: skeleton-textの基本表示

- Given: text="AI is thinking harder..."
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton skeleton-text`クラスが適用された`<span>`要素が表示され、テキストが表示される

### TC-006: skeleton-textとカスタムクラス

- Given: text="Loading...", class="text-lg font-bold"
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton skeleton-text text-lg font-bold`クラスが適用され、カスタムスタイルのテキストが表示される

### TC-007: 複数のSkeleton要素の組み合わせ

- Given: 複数のSkeletonコンポーネントを組み合わせて配置
- When: 各Skeletonコンポーネントに異なるclass（h-4 w-20, h-4 w-28など）を設定
- Then: それぞれのSkeletonが独立したプレースホルダーとして表示される

### TC-008: div要素とspan要素の使い分け

- Given: textが指定されていない場合とされている場合
- When: Skeletonコンポーネントをレンダリング
- Then: textなし→`<div>`要素、textあり→`<span>`要素が使用される

