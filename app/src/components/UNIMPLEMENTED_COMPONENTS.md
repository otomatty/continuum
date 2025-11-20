# 未実装UIコンポーネント一覧

## 実装済みコンポーネント

以下のコンポーネントは既に実装済みです：

1. Navbar - ナビゲーションバー
2. Button - ボタン
3. Card - カード
4. Table - テーブル
5. Badge - バッジ
6. Avatar - アバター
7. Input - テキスト入力フィールド
8. Textarea - 複数行テキスト入力
9. Select - セレクトボックス
10. Checkbox - チェックボックス
11. Radio - ラジオボタン
12. Toggle - トグルスイッチ
13. Range - レンジスライダー
14. Alert - アラート/通知メッセージ
15. Progress - プログレスバー
16. Loading - ローディングスピナー
17. Skeleton - スケルトンローディング
18. Divider - 区切り線

---

## 未実装コンポーネント（優先度順）

### 優先度：中（ナビゲーション・レイアウト系）

#### 1. Tabs
- **説明**: タブの表示と管理
- **DaisyUIクラス**: `tabs`, `tab`, `tab-active`, `tab-bordered`, `tab-lifted`
- **用途**: コンテンツの切り替え表示
- **関連コンポーネント**: Tab, TabList, TabPanel

#### 2. Accordion
- **説明**: アコーディオンの表示と管理
- **DaisyUIクラス**: `collapse`, `collapse-arrow`, `collapse-plus`, `collapse-open`
- **用途**: 折りたたみ可能なコンテンツの表示
- **関連コンポーネント**: AccordionItem, AccordionHeader, AccordionContent

#### 3. Breadcrumbs
- **説明**: パンくずリストの表示
- **DaisyUIクラス**: `breadcrumbs`
- **用途**: ナビゲーションパスの表示
- **関連コンポーネント**: BreadcrumbItem

#### 4. Pagination
- **説明**: ページネーションの表示と管理
- **DaisyUIクラス**: `pagination`, `btn-group`
- **用途**: ページ番号の表示とナビゲーション
- **プロパティ**: `current_page`, `total_pages`, `on_page_change`

#### 5. Steps
- **説明**: ステップインジケーターの表示
- **DaisyUIクラス**: `steps`, `step`, `step-primary`, `step-success`, `step-warning`, `step-error`
- **用途**: マルチステップフォームやプロセスの進捗表示
- **関連コンポーネント**: StepItem

#### 6. Footer
- **説明**: フッターの表示
- **DaisyUIクラス**: `footer`
- **用途**: ページ下部の情報表示
- **プロパティ**: `class` (optional)

---

### 優先度：低（インタラクティブ系）

#### 7. Modal
- **説明**: モーダルダイアログの表示と管理
- **DaisyUIクラス**: `modal`, `modal-open`, `modal-box`, `modal-backdrop`
- **用途**: 確認ダイアログ、フォーム入力など
- **関連コンポーネント**: ModalBox, ModalHeader, ModalBody, ModalFooter, ModalAction
- **プロパティ**: `open` (Signal), `on_close`

#### 8. Dropdown
- **説明**: ドロップダウンメニューの表示と管理
- **DaisyUIクラス**: `dropdown`, `dropdown-content`, `dropdown-hover`, `dropdown-open`
- **用途**: メニューの表示
- **関連コンポーネント**: DropdownButton, DropdownMenu, DropdownItem
- **プロパティ**: `open` (Signal), `on_toggle`

#### 9. Drawer
- **説明**: サイドドロワーの表示と管理
- **DaisyUIクラス**: `drawer`, `drawer-open`, `drawer-side`, `drawer-content`
- **用途**: サイドメニューの表示
- **関連コンポーネント**: DrawerSide, DrawerContent, DrawerToggle
- **プロパティ**: `open` (Signal), `on_close`, `side` (left/right)

#### 10. Tooltip
- **説明**: ツールチップの表示
- **DaisyUIクラス**: `tooltip`, `tooltip-top`, `tooltip-bottom`, `tooltip-left`, `tooltip-right`
- **用途**: ホバー時の補足情報表示
- **プロパティ**: `content`, `position` (top/bottom/left/right)

#### 11. Popover
- **説明**: ポップオーバーの表示と管理
- **DaisyUIクラス**: `popover`, `popover-open`
- **用途**: クリック時の情報表示
- **プロパティ**: `open` (Signal), `on_toggle`, `content`

#### 12. Toast
- **説明**: トースト通知の表示と管理
- **DaisyUIクラス**: `toast`, `alert`
- **用途**: 一時的な通知メッセージの表示
- **プロパティ**: `message`, `variant`, `duration`, `on_close`

---

### 優先度：低（その他）

#### 13. Hero
- **説明**: ヒーローセクションの表示
- **DaisyUIクラス**: `hero`, `hero-content`, `hero-overlay`
- **用途**: ランディングページのメインセクション
- **関連コンポーネント**: HeroContent, HeroOverlay

#### 14. Stats
- **説明**: 統計情報の表示
- **DaisyUIクラス**: `stats`, `stat`, `stat-title`, `stat-value`, `stat-desc`
- **用途**: 数値統計の表示
- **関連コンポーネント**: StatItem, StatTitle, StatValue, StatDescription

#### 15. Countdown
- **説明**: カウントダウンタイマーの表示
- **DaisyUIクラス**: `countdown`
- **用途**: イベントまでの残り時間表示
- **プロパティ**: `target_date`, `on_complete`

#### 16. Rating
- **説明**: レーティング表示
- **DaisyUIクラス**: `rating`, `rating-half`, `rating-hidden`
- **用途**: 評価値の表示
- **プロパティ**: `value` (0-5), `readonly`, `on_change`

---

## 実装推奨順序

### Phase 1: フィードバック・表示系（優先度：中）✅ 完了
1. ✅ Alert
2. ✅ Progress
3. ✅ Loading
4. ✅ Skeleton
5. ✅ Divider

### Phase 2: ナビゲーション・レイアウト系（優先度：中）
1. Tabs
2. Accordion
3. Breadcrumbs
4. Pagination
5. Steps
6. Footer

### Phase 3: インタラクティブ系（優先度：低）
7. Modal
8. Dropdown
9. Drawer
10. Tooltip
11. Popover
12. Toast

### Phase 4: その他（優先度：低）
13. Hero
14. Stats
15. Countdown
16. Rating

---

## 注意事項

- 各コンポーネント実装時は、既存の実装パターンに従うこと
- Leptosの`#[component]`マクロを使用
- `optional`な`class`プロパティでカスタマイズ可能にする
- `variant` enumでスタイルバリエーションを提供（必要に応じて）
- 実装後は必ず仕様書（`.spec.md`）を作成すること
- `mod.rs`にモジュールを追加すること

