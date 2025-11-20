# PR #6 レビューコメント対応計画

## 概要

PR #6「feat: implement prototype pages with mock data」に対するレビューコメントの対応計画です。

**PR URL**: https://github.com/otomatty/continuum/pull/6

## レビューコメントの分類

### 1. コード品質の改善（必須対応）

#### 1.1 未使用変数の削除
- **ファイル**: `app/src/pages/dashboard/mod.rs:33`
  - `_monthly_stats` 変数を削除
- **ファイル**: `app/src/mock/data.rs:451`
  - `_user` 変数を削除

#### 1.2 コメントの追加
- **ファイル**: `app/src/mock/data.rs:100`
  - `ContributionDay` 構造体に `NaiveDate` を使用する理由をコメントで説明
- **ファイル**: `app/src/pages/portfolio/mod.rs:35`
  - パスパラメータルーティングのTODOコメントを追加

#### 1.3 エラーハンドリングの改善
- **ファイル**: `app/src/mock/data.rs:430`
  - `unwrap_or` を `expect` に変更（より明確なエラーメッセージ）

#### 1.4 ハードコードされた値の動的化
- **ファイル**: `app/src/mock/data.rs:427`
  - ハードコードされた日付 `2024-01-01` を動的な日付に変更

### 2. パフォーマンス改善（推奨対応）

#### 2.1 データの再生成を避ける
- **ファイル**: `app/src/pages/portfolio/mod.rs:56`
  - `repository_contributions` クロージャが `view!` マクロ内で複数回呼び出されている
  - `view!` マクロの前に一度だけ生成し、変数に格納

#### 2.2 キャッシュの使用
- **ファイル**: `app/src/mock/data.rs:122`
  - `generate_mock_users` と `generate_mock_repositories` に `once_cell::sync::Lazy` を使用してキャッシュ

#### 2.3 効率的な計算
- **ファイル**: `app/src/pages/portfolio/components.rs:183`
  - ベクターの5回のイテレーションを `fold` で1回に統合

### 3. コンポーネントの柔軟性向上（推奨対応）

以下のコンポーネントの `class` プロパティを `&'static str` から `String` に変更：

- `app/src/components/button.rs:20`
- `app/src/components/card.rs:5` (Card, CardTitle)
- `app/src/components/card.rs:49` (CardBody - `card-body` クラスも追加)
- `app/src/components/table.rs:5` (Table, TableHead, TableBody, TableRow, TableHeader, TableCell)
- `app/src/components/avatar.rs:7` (Avatar, AvatarGroup)
- `app/src/components/badge.rs:18`

### 4. Leptosの慣用的な書き方（推奨対応）

#### 4.1 `class:` ディレクティブの使用
- **ファイル**: `app/src/pages/dashboard/mod.rs:95`
- **ファイル**: `app/src/pages/portfolio/mod.rs:95`
  - 条件付きクラスの適用に `class:` ディレクティブを使用

#### 4.2 `on:click` の簡略化
- **ファイル**: `app/src/components/button.rs:43`
  - `on:click` ハンドラを簡略化（`Option<Callback>` を直接扱う）

### 5. ルーティングの一元管理（将来対応）

- **ファイル**: `app/src/components/navbar.rs:22`
- **ファイル**: `app/src/pages/home/mod.rs:35`
  - ルート定義を `Routable` トレイトを実装したenumで一元管理
  - 現時点では優先度低（プロトタイプ段階のため）

### 6. 設計書の更新（必須対応）

- **ファイル**: `docs/03_plans/continuum/prototype-pages.md:242`
  - `ContributionDay` の `date` フィールドの型を `DateTime` から `NaiveDate` に更新

## 対応優先度

### 優先度: 高（必須対応）
1. 未使用変数の削除
2. コメントの追加
3. エラーハンドリングの改善
4. ハードコードされた値の動的化
5. 設計書の更新

### 優先度: 中（推奨対応）
1. パフォーマンス改善（データの再生成を避ける、キャッシュの使用）
2. コンポーネントの柔軟性向上（`class` プロパティの型変更）
3. Leptosの慣用的な書き方

### 優先度: 低（将来対応）
1. ルーティングの一元管理

## 対応手順

1. **必須対応項目の実装**
   - 未使用変数の削除
   - コメントの追加
   - エラーハンドリングの改善
   - ハードコードされた値の動的化
   - 設計書の更新

2. **推奨対応項目の実装**
   - パフォーマンス改善
   - コンポーネントの柔軟性向上
   - Leptosの慣用的な書き方

3. **動作確認**
   - 開発サーバーで動作確認
   - ビルドエラーの確認

4. **PRへの反映**
   - 変更をコミット
   - PRにコメントで対応状況を報告

## 関連ファイル

- `app/src/mock/data.rs`
- `app/src/pages/dashboard/mod.rs`
- `app/src/pages/portfolio/mod.rs`
- `app/src/pages/portfolio/components.rs`
- `app/src/components/button.rs`
- `app/src/components/card.rs`
- `app/src/components/table.rs`
- `app/src/components/avatar.rs`
- `app/src/components/badge.rs`
- `app/src/components/navbar.rs`
- `app/src/pages/home/mod.rs`
- `docs/03_plans/continuum/prototype-pages.md`

## 対応完了状況

### ✅ 完了した項目（2025-11-20）

#### 必須対応項目
- ✅ 未使用変数の削除（`_monthly_stats`, `_user`）
- ✅ コメントの追加（`NaiveDate`使用理由、TODOコメント）
- ✅ エラーハンドリングの改善（`unwrap_or` → `expect`）
- ✅ ハードコードされた日付の動的化（`Utc::now().date_naive()`を使用）
- ✅ 設計書の更新（`ContributionDay`の型を`NaiveDate`に変更）

#### 推奨対応項目
- ✅ パフォーマンス改善
  - ✅ `repository_contributions`の再生成を回避（`view!`マクロの前に生成）
  - ✅ `generate_mock_users`と`generate_mock_repositories`に`once_cell::sync::Lazy`を使用してキャッシュ
  - ✅ `fold`を使った効率的な計算（5回のイテレーションを1回に統合）
- ✅ コンポーネントの柔軟性向上
  - ✅ すべてのコンポーネントの`class`プロパティを`String`型に変更
  - ✅ `CardBody`に`card-body`クラスを追加
- ✅ Leptosの慣用的な書き方
  - ✅ `class:`ディレクティブを使用（dashboard/mod.rs, portfolio/mod.rs）
  - ✅ `on:click`ハンドラの簡略化（button.rs）

### ⏳ 将来対応項目
- ⏳ ルーティングの一元管理（プロトタイプ段階では優先度低）

## 変更されたファイル一覧

1. `app/src/pages/dashboard/mod.rs` - 未使用変数削除、`class:`ディレクティブ使用
2. `app/src/pages/portfolio/mod.rs` - TODOコメント追加、データ再生成回避、`class:`ディレクティブ使用
3. `app/src/pages/portfolio/components.rs` - `fold`を使った効率的な計算
4. `app/src/mock/data.rs` - コメント追加、エラーハンドリング改善、日付動的化、キャッシュ追加、未使用変数削除
5. `app/src/components/button.rs` - `class`プロパティを`String`型に変更、`on:click`簡略化
6. `app/src/components/card.rs` - `class`プロパティを`String`型に変更、`CardBody`に`card-body`クラス追加
7. `app/src/components/table.rs` - すべてのコンポーネントの`class`プロパティを`String`型に変更
8. `app/src/components/avatar.rs` - `class`プロパティを`String`型に変更
9. `app/src/components/badge.rs` - `class`プロパティを`String`型に変更
10. `docs/03_plans/continuum/prototype-pages.md` - `ContributionDay`の型を`NaiveDate`に更新
11. `Cargo.toml` - `once_cell`依存関係を追加
12. `app/Cargo.toml` - `once_cell`依存関係を追加

## 参考情報

- [Leptos Documentation](https://leptos.dev/)
- [GitHub PR #6](https://github.com/otomatty/continuum/pull/6)
