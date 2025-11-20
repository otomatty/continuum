# Tooltip Component Specification

## Related Files

- Implementation: `app/src/components/tooltip/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/tooltip/

## Requirements

### 責務

- ツールチップの表示
- DaisyUIの`tooltip`クラスを使用したスタイリング
- ホバー時の補足情報表示

### Props構造

#### Tooltipコンポーネント

- `content` (String): ツールチップのテキスト（必須）
- `position` (optional, TooltipPosition): ツールチップの位置（デフォルト: Top）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ツールチップを表示する要素

### TooltipPosition

以下の位置をサポート：

- `Top`: 上側（デフォルト）
- `Bottom`: 下側
- `Left`: 左側
- `Right`: 右側

### スタイル

- `Tooltip`: `tooltip`クラスと位置クラス（`tooltip-top`, `tooltip-bottom`, `tooltip-left`, `tooltip-right`）を適用、`data-tip`属性にコンテンツを設定

### 機能

- 子要素にホバーするとツールチップが表示される
- `data-tip`属性に設定されたテキストがツールチップとして表示される

## Test Cases

### TC-001: Tooltipの基本表示

- Given: content="ツールチップテキスト", positionが指定されていない
- When: Tooltipコンポーネントをレンダリング
- Then: `tooltip tooltip-top`クラスが適用され、`data-tip`属性に"ツールチップテキスト"が設定される

### TC-002: Top位置の適用

- Given: position=TooltipPosition::Top
- When: Tooltipコンポーネントをレンダリング
- Then: `tooltip tooltip-top`クラスが適用される

### TC-003: Bottom位置の適用

- Given: position=TooltipPosition::Bottom
- When: Tooltipコンポーネントをレンダリング
- Then: `tooltip tooltip-bottom`クラスが適用される

### TC-004: Left位置の適用

- Given: position=TooltipPosition::Left
- When: Tooltipコンポーネントをレンダリング
- Then: `tooltip tooltip-left`クラスが適用される

### TC-005: Right位置の適用

- Given: position=TooltipPosition::Right
- When: Tooltipコンポーネントをレンダリング
- Then: `tooltip tooltip-right`クラスが適用される

### TC-006: Tooltipのコンテンツ表示

- Given: content="これはツールチップです"
- When: Tooltipコンポーネントをレンダリング
- Then: `data-tip`属性に"これはツールチップです"が設定される

### TC-007: Tooltipのカスタムクラス

- Given: Tooltipにclass="tooltip-open"を設定
- When: Tooltipコンポーネントをレンダリング
- Then: ベースクラスに加えて"tooltip-open"が追加される

### TC-008: Tooltipの子要素表示

- Given: Tooltip内にボタン要素を配置
- When: Tooltipコンポーネントをレンダリング
- Then: ボタン要素が表示され、ホバー時にツールチップが表示される

### TC-009: 複数のTooltipの使用

- Given: ページ内に複数のTooltipコンポーネントを配置
- When: Tooltipコンポーネントをレンダリング
- Then: それぞれのTooltipが正しく表示され、個別に動作する

