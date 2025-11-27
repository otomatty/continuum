# Task 21: レスポンシブ最適化

## 1. 目的と背景

### なぜこのタスクが必要か
すべてのページとコンポーネントがモバイル、タブレット、デスクトップで適切に表示されるようにレスポンシブデザインを最適化します。DaisyUI + Tailwind CSS のユーティリティを活用します。

### 完成時のユーザー体験
- どのデバイスでも快適に利用できる
- モバイルでは必要な情報が優先的に表示される
- タッチ操作に最適化されたUIサイズ

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 4-20: すべてのページとコンポーネントが実装済み

### 必要な知識
- Tailwind CSS のレスポンシブユーティリティ
- DaisyUI のコンポーネント

---

## 3. ブレークポイント定義

Tailwind CSS のデフォルトブレークポイントを使用：

| プレフィックス | 最小幅 | デバイス |
|--------------|-------|---------|
| `sm` | 640px | モバイル（横向き） |
| `md` | 768px | タブレット |
| `lg` | 1024px | 小型デスクトップ |
| `xl` | 1280px | デスクトップ |
| `2xl` | 1536px | 大型デスクトップ |

---

## 4. チェックリストと最適化ポイント

### 4.1 ヘッダー / ナビゲーション

`app/src/components/header/mod.rs`:

```rust
// ✅ 最適化ポイント
// - モバイル: ハンバーガーメニュー + ドロワー
// - デスクトップ: 水平ナビゲーション

view! {
    <header class="navbar bg-base-100 sticky top-0 z-50">
        // モバイルメニューボタン（md以下で表示）
        <div class="navbar-start lg:hidden">
            <label for="mobile-drawer" class="btn btn-ghost btn-circle">
                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                </svg>
            </label>
        </div>

        // デスクトップナビゲーション（lg以上で表示）
        <div class="navbar-center hidden lg:flex">
            <ul class="menu menu-horizontal px-1">
                // メニュー項目
            </ul>
        </div>
    </header>
}
```

**チェック項目:**
- [ ] モバイルでハンバーガーメニューが表示される
- [ ] デスクトップで水平ナビゲーションが表示される
- [ ] ドロワーが正しく開閉する
- [ ] ヘッダーが固定されている

### 4.2 ダッシュボード

`app/src/pages/dashboard/mod.rs`:

```rust
// ✅ 最適化ポイント
// - モバイル: 1列表示、縦スクロール
// - タブレット: 2列グリッド
// - デスクトップ: 複数列グリッド + サイドバー

view! {
    <div class="flex flex-col lg:flex-row gap-8">
        // メインコンテンツ
        <div class="flex-1">
            // KPIカード: 2列 → 3列 → 6列
            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                // ...
            </div>

            // ランキングとアクティビティ: 縦並び → 横並び
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mt-8">
                // ...
            </div>
        </div>

        // サイドバー: モバイルでは非表示 or 下部に移動
        <aside class="w-full lg:w-80 order-first lg:order-last">
            // ...
        </aside>
    </div>
}
```

**チェック項目:**
- [ ] KPIカードが画面幅に応じて列数が変わる
- [ ] ランキングとアクティビティがレスポンシブに配置される
- [ ] 情報の優先度に応じた表示順序

### 4.3 テーブル

`app/src/pages/repositories/components/repository_table.rs`:

```rust
// ✅ 最適化ポイント
// - モバイル: カード形式 or 横スクロール
// - デスクトップ: フルテーブル

view! {
    // モバイル用カード表示
    <div class="block md:hidden space-y-4">
        {repositories.iter().map(|repo| {
            view! {
                <div class="card bg-base-200">
                    <div class="card-body p-4">
                        <h3 class="font-bold">{repo.name.clone()}</h3>
                        <p class="text-sm text-base-content/60">{repo.description.clone()}</p>
                        <div class="flex gap-4 text-sm mt-2">
                            <span>"⭐ " {repo.stars}</span>
                            <span>"🍴 " {repo.forks}</span>
                        </div>
                    </div>
                </div>
            }
        }).collect_view()}
    </div>

    // デスクトップ用テーブル表示
    <div class="hidden md:block overflow-x-auto">
        <table class="table">
            // ...
        </table>
    </div>
}
```

**チェック項目:**
- [ ] モバイルでカード形式になる（または横スクロール可能）
- [ ] デスクトップでフルテーブルが表示される
- [ ] 重要な情報が最初に表示される

### 4.4 カードグリッド

`app/src/pages/knowledge/components/knowledge_grid.rs`:

```rust
// ✅ 最適化ポイント
// - モバイル: 1列
// - タブレット: 2列
// - デスクトップ: 3列

view! {
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        // ...
    </div>
}
```

**チェック項目:**
- [ ] 列数が画面幅に応じて変わる
- [ ] カード間の余白が適切
- [ ] カード内のコンテンツが収まっている

### 4.5 サイドバー + メインコンテンツ

`app/src/pages/knowledge/mod.rs`:

```rust
// ✅ 最適化ポイント
// - モバイル: サイドバーが上部に移動 or 折りたたみ
// - デスクトップ: 左サイドバー + 右メインコンテンツ

view! {
    <div class="flex flex-col lg:flex-row gap-8">
        // サイドバー: モバイルでは全幅、デスクトップでは固定幅
        <aside class="w-full lg:w-64 shrink-0 order-first">
            // カテゴリフィルターなど
        </aside>

        // メインコンテンツ
        <main class="flex-1">
            // ...
        </main>
    </div>
}
```

**チェック項目:**
- [ ] モバイルでサイドバーが上部に表示される
- [ ] デスクトップでサイドバーが左側に固定される
- [ ] サイドバーの幅が適切

### 4.6 フォーム入力

```rust
// ✅ 最適化ポイント
// - モバイル: 入力フィールドが全幅
// - タッチターゲットが44px以上

view! {
    <input
        type="text"
        class="input input-bordered w-full md:w-auto md:min-w-[300px]"
        // タッチターゲットサイズは input のデフォルトで確保される
    />

    // ボタン
    <button class="btn btn-primary w-full md:w-auto">
        "送信"
    </button>
}
```

**チェック項目:**
- [ ] 入力フィールドがモバイルで全幅
- [ ] ボタンがタップしやすいサイズ（44px以上）
- [ ] フォーム全体のレイアウトが適切

### 4.7 モーダル / ドロワー

```rust
// ✅ 最適化ポイント
// - モバイル: フルスクリーンモーダル or ボトムシート
// - デスクトップ: 中央配置のモーダル

view! {
    <dialog class="modal modal-bottom md:modal-middle">
        <div class="modal-box w-full max-w-lg">
            // コンテンツ
        </div>
    </dialog>
}
```

**チェック項目:**
- [ ] モバイルでボトムから表示される（modal-bottom）
- [ ] デスクトップで中央に表示される（modal-middle）
- [ ] 閉じるボタンがタップしやすい

### 4.8 画像 / アバター

```rust
// ✅ 最適化ポイント
// - レスポンシブ画像サイズ
// - 適切なアスペクト比の維持

view! {
    // アバター: サイズをブレークポイントで調整
    <div class="avatar">
        <div class="w-12 md:w-16 lg:w-20 rounded-full">
            <img src=avatar_url alt="" />
        </div>
    </div>

    // ヒーロー画像
    <img
        class="w-full h-48 md:h-64 lg:h-80 object-cover"
        src=image_url
        alt=""
    />
}
```

**チェック項目:**
- [ ] アバターサイズが画面幅に応じて変わる
- [ ] 画像のアスペクト比が維持される

---

## 5. テスト手順

### 5.1 ブラウザ開発者ツール

1. Chrome DevTools を開く（F12）
2. Device Toolbar をオン（Ctrl+Shift+M）
3. 以下のデバイスでテスト:
   - iPhone SE (375px)
   - iPhone 12 Pro (390px)
   - iPad (768px)
   - iPad Pro (1024px)
   - Desktop (1440px)

### 5.2 チェックコマンド

```bash
# 開発サーバー起動
bun run dev

# 各ページにアクセスして確認
# - http://localhost:3000/ (ホーム)
# - http://localhost:3000/dashboard
# - http://localhost:3000/knowledge
# - http://localhost:3000/repositories
# - http://localhost:3000/portfolio/{username}
# - http://localhost:3000/settings
```

---

## 6. 完了条件チェックリスト

### ヘッダー / ナビゲーション
- [ ] モバイルでハンバーガーメニュー
- [ ] デスクトップで水平ナビゲーション

### ダッシュボード
- [ ] KPIカードのレスポンシブグリッド
- [ ] ランキングとアクティビティのレイアウト

### 一覧ページ（リポジトリ、コントリビューター、知見）
- [ ] テーブル/グリッドのレスポンシブ対応
- [ ] カード形式のフォールバック

### 詳細ページ
- [ ] サイドバーのレスポンシブ配置
- [ ] コンテンツの読みやすさ

### フォーム
- [ ] 入力フィールドの幅
- [ ] タッチターゲットサイズ

### モーダル
- [ ] モバイルでボトムシート
- [ ] デスクトップで中央モーダル

### 全般
- [ ] すべてのページで横スクロールが不要
- [ ] テキストが読みやすいサイズ
- [ ] タップ可能な要素が44px以上

---

## 7. 参照ドキュメント

- Tailwind CSS Responsive Design: https://tailwindcss.com/docs/responsive-design
- DaisyUI Components: https://daisyui.com/components/
- WCAG Touch Target Size: https://www.w3.org/WAI/WCAG21/Understanding/target-size.html

