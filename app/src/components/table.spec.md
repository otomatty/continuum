# Table Component Specification

## Related Files

- Implementation: `app/src/components/table.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/table/

## Requirements

### 責務

- テーブルの表示と管理
- DaisyUIの`table`クラスを使用したスタイリング
- TableHead, TableBody, TableRow, TableHeader, TableCellコンポーネントとの連携

### Props構造

#### Tableコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): テーブルの子要素（TableHead, TableBodyなど）

#### TableHeadコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): テーブルヘッダーの行（TableRowなど）

#### TableBodyコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): テーブル本文の行（TableRowなど）

#### TableRowコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): テーブル行のセル（TableHeader, TableCellなど）

#### TableHeaderコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ヘッダーセルのテキスト

#### TableCellコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): セルのテキスト

### スタイル

- `Table`: `table`クラスを適用
- `TableHead`: `<thead>`要素を生成
- `TableBody`: `<tbody>`要素を生成
- `TableRow`: `<tr>`要素を生成
- `TableHeader`: `<th>`要素を生成
- `TableCell`: `<td>`要素を生成

## Test Cases

### TC-001: Tableの基本表示

- Given: Tableコンポーネントをレンダリング
- When: ページを表示
- Then: `table`クラスが適用された`<table>`要素が表示される

### TC-002: TableHeadの表示

- Given: TableHeadコンポーネントをレンダリング
- When: ページを表示
- Then: `<thead>`要素が表示される

### TC-003: TableBodyの表示

- Given: TableBodyコンポーネントをレンダリング
- When: ページを表示
- Then: `<tbody>`要素が表示される

### TC-004: TableRowの表示

- Given: TableRowコンポーネントをレンダリング
- When: ページを表示
- Then: `<tr>`要素が表示される

### TC-005: TableHeaderの表示

- Given: TableHeaderコンポーネントをレンダリング
- When: ページを表示
- Then: `<th>`要素が表示される

### TC-006: TableCellの表示

- Given: TableCellコンポーネントをレンダリング
- When: ページを表示
- Then: `<td>`要素が表示される

### TC-007: カスタムクラスの追加

- Given: Tableにclass="table-zebra"を設定
- When: Tableコンポーネントをレンダリング
- Then: ベースクラスに加えて"table-zebra"が追加される

### TC-008: テーブル構造の組み合わせ

- Given: Table内にTableHeadとTableBodyを配置し、それぞれにTableRowとTableHeader/TableCellを含める
- When: Tableコンポーネントをレンダリング
- Then: 正しいHTMLテーブル構造が生成される

