# Tailwind CSS スタイリング実装ガイド

このプロジェクトでは、**Tailwind CSS v4**と**DaisyUI v5**を使用してスタイリングを実装しています。Leptosフレームワークと統合され、SSR（Server-Side Rendering）とクライアントサイドの両方で動作します。

## 目次

- [概要](#概要)
- [アーキテクチャ](#アーキテクチャ)
- [セットアップ](#セットアップ)
- [ファイル構造](#ファイル構造)
- [Tailwind CSS v4の使用方法](#tailwind-css-v4の使用方法)
- [DaisyUIの使用方法](#daisyuiの使用方法)
- [テーマシステム](#テーマシステム)
- [カスタムスタイル](#カスタムスタイル)
- [コンポーネントでの実装例](#コンポーネントでの実装例)
- [ビルドプロセス](#ビルドプロセス)
- [開発ワークフロー](#開発ワークフロー)
- [トラブルシューティング](#トラブルシューティング)

## 概要

### 技術スタック

- **Tailwind CSS v4.1.17**: ユーティリティファーストのCSSフレームワーク
- **DaisyUI v5.5.1-beta.2**: Tailwind CSSベースのコンポーネントライブラリ
- **@tailwindcss/cli v4.1.17**: Tailwind CSS CLIツール
- **Leptos**: RustベースのWebフレームワーク（SSR対応）

### 主な特徴

- **Tailwind CSS v4の新機能**: `@import`と`@theme`ディレクティブを使用したモダンな設定
- **DaisyUI統合**: 30以上のテーマと豊富なコンポーネントクラス
- **テーマ切り替え**: Light/Dark/Systemの3つのテーマモードをサポート
- **カスタムアニメーション**: Fade-inアニメーションなどのカスタムスタイル
- **SSR対応**: LeptosのSSR機能と完全に統合

## アーキテクチャ

### スタイルの読み込みフロー

```
1. style/input.css (Tailwind CSS v4の設定とカスタムスタイル)
   ↓
2. bun run build:css (Tailwind CSS CLIでコンパイル)
   ↓
3. style/output.css (生成されたCSSファイル)
   ↓
4. Leptosがoutput.cssを読み込み
   ↓
5. ブラウザで適用
```

### テーマ適用の仕組み

```rust
// app/src/lib.rs
<div data-theme=effective_theme>
  // DaisyUIのテーマがdata-theme属性に基づいて適用される
</div>
```

`data-theme`属性の値に応じて、DaisyUIが自動的にテーマを切り替えます。

## セットアップ

### 依存関係のインストール

```bash
# Node.jsパッケージのインストール
bun install

# または npm を使用する場合
npm install
```

### 必要なパッケージ

`package.json`に以下のパッケージが定義されています：

```json
{
  "devDependencies": {
    "@tailwindcss/cli": "^4.1.17",
    "daisyui": "^5.5.1-beta.2",
    "tailwindcss": "^4.1.17"
  }
}
```

## ファイル構造

```
continuum/
├── style/
│   ├── input.css          # Tailwind CSS v4の設定とカスタムスタイル
│   └── output.css         # コンパイル済みCSS（生成ファイル）
│
├── tailwind.config.ts      # Tailwind CSS設定ファイル（DaisyUI統合）
│
├── package.json            # Node.js依存関係とビルドスクリプト
│
└── app/src/
    ├── lib.rs              # メインアプリケーション（スタイルシート読み込み）
    ├── components/         # UIコンポーネント（Tailwindクラスを使用）
    └── concepts/
        └── theme/          # テーマ管理Concept
```

## Tailwind CSS v4の使用方法

### 基本的な使い方

Tailwind CSS v4では、`@import`ディレクティブを使用してTailwindを読み込みます：

```css
/* style/input.css */
@import "tailwindcss";
```

### コンポーネントでの使用例

Leptosコンポーネント内でTailwindクラスを直接使用できます：

```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div class="container mx-auto py-8">
            <h1 class="text-5xl font-bold mb-4">"タイトル"</h1>
            <p class="text-xl text-gray-600">"説明文"</p>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                "クリック"
            </button>
        </div>
    }
}
```

### レスポンシブデザイン

Tailwindのレスポンシブプレフィックスを使用：

```rust
view! {
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        // モバイル: 1列、タブレット: 2列、デスクトップ: 3列
    </div>
}
```

### 動的クラス

Leptosの`signal`を使用して動的にクラスを変更：

```rust
let (is_active, set_is_active) = signal(false);

view! {
    <button
        class=move || format!("btn {}", if is_active.get() { "btn-active" } else { "" })
        on:click=move |_| set_is_active.update(|x| *x = !*x)
    >
        "トグル"
    </button>
}
```

## DaisyUIの使用方法

### DaisyUIコンポーネントクラス

DaisyUIが提供するコンポーネントクラスを使用できます：

```rust
// ボタン
<button class="btn btn-primary">"プライマリボタン"</button>
<button class="btn btn-secondary">"セカンダリボタン"</button>
<button class="btn btn-ghost">"ゴーストボタン"</button>

// カード
<div class="card bg-base-100 shadow-xl">
    <div class="card-body">
        <h2 class="card-title">"カードタイトル"</h2>
        <p>"カードの内容"</p>
    </div>
</div>

// アラート
<div class="alert alert-info">
    <span>"情報メッセージ"</span>
</div>
```

### カスタムコンポーネントでの使用

プロジェクト内のカスタムコンポーネントでもDaisyUIクラスを使用：

```rust
// app/src/components/button/mod.rs
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Ghost => "btn-ghost",
    };

    view! {
        <button class=format!("btn {}", variant_class)>
            {children()}
        </button>
    }
}
```

## テーマシステム

### テーマの種類

`tailwind.config.ts`で30以上のDaisyUIテーマを有効化：

```typescript
daisyui: {
  themes: [
    "light", "dark", "cupcake", "bumblebee",
    "emerald", "corporate", "synthwave", "retro",
    // ... その他多数
  ],
}
```

### カスタムテーマ変数

`style/input.css`の`@theme`ブロックでカスタムテーマ変数を定義：

```css
@theme {
  --color-primary: #3b82f6;
  --color-secondary: #8b5cf6;
  --color-accent: #06b6d4;
  --color-neutral: #3d4451;
  --color-base-100: #ffffff;
  --color-base-200: #f2f2f2;
  --color-base-300: #e5e6e6;
  --color-base-content: #1f2937;
  
  --rounded-box: 1rem;
  --rounded-btn: 0.5rem;
  --rounded-badge: 1.9rem;
}
```

### テーマ切り替えの実装

`app/src/lib.rs`でテーマ状態を管理：

```rust
// テーマ状態の初期化
let initial_theme_state = {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(stored_theme) = get_theme_from_storage() {
            ThemeState { current_theme: stored_theme }
        } else {
            ThemeState::default()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        ThemeState::default()
    }
};

// data-theme属性に適用
let effective_theme = move || {
    let state = theme_state.get();
    match state.current_theme {
        Theme::Light => "light",
        Theme::Dark => "dark",
        Theme::System => {
            // システム設定を確認
            // ...
        }
    }
};

view! {
    <div data-theme=effective_theme>
        // アプリケーションコンテンツ
    </div>
}
```

### テーマの切り替え方法

`Theme` Conceptのアクションを使用：

```rust
use concepts::theme::{Theme, ThemeState};

let (theme_state, set_theme_state) = signal(ThemeState::default());

// テーマを切り替え
set_theme_state.update(|state| {
    state.current_theme = Theme::Dark;
});
```

## カスタムスタイル

### カスタムCSSクラス

`style/input.css`にカスタムスタイルを追加：

```css
/* Custom Styles */
body {
  font-family: var(--font-sans, system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif);
  background-color: var(--color-base-100);
  color: var(--color-base-content);
}

/* カスタムボタンスタイル */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem 1rem;
  border-radius: var(--rounded-btn);
  font-weight: 500;
  transition: all 0.2s;
  cursor: pointer;
  border: none;
}

.btn-primary {
  background-color: var(--color-primary);
  color: white;
}
```

### カスタムアニメーション

Fade-inアニメーションの実装例：

```css
/* Fade-in Animation */
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.fade-in {
  opacity: 0;
  transform: translateY(20px);
}

.fade-in-visible {
  animation: fade-in 600ms ease-out forwards;
}

/* Delay classes for sequential animations */
.delay-100 { animation-delay: 100ms; }
.delay-200 { animation-delay: 200ms; }
.delay-300 { animation-delay: 300ms; }
```

### コンポーネントでのアニメーション使用

`FadeIn`コンポーネントの実装例：

```rust
// app/src/components/fade_in/mod.rs
#[component]
pub fn FadeIn(
    children: Children,
    #[prop(optional, into)] delay: Option<String>,
) -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    
    // Intersection Observerで表示を検知
    // ...
    
    let delay_class = delay
        .as_ref()
        .map(|d| format!("delay-{}", d))
        .unwrap_or_default();
    
    let base_class = if is_visible.get() {
        "fade-in-visible"
    } else {
        "fade-in"
    };
    
    view! {
        <div class=format!("{} {}", base_class, delay_class)>
            {children()}
        </div>
    }
}
```

## コンポーネントでの実装例

### Buttonコンポーネント

```rust
// app/src/components/button/mod.rs
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Ghost => "btn-ghost",
    };

    let combined_class = if class.is_empty() {
        format!("btn {}", variant_class)
    } else {
        format!("btn {} {}", variant_class, class)
    };

    view! {
        <button class=combined_class>
            {children()}
        </button>
    }
}
```

### Cardコンポーネント

```rust
// app/src/components/card/mod.rs
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional, into)] class: String,
    children: Children
) -> impl IntoView {
    let card_class = if class.is_empty() {
        "card".to_string()
    } else {
        format!("card {}", class)
    };

    view! {
        <div class=card_class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional, into)] class: String,
    children: Children
) -> impl IntoView {
    let title_class = if class.is_empty() {
        "text-2xl font-bold mb-4".to_string()
    } else {
        format!("text-2xl font-bold mb-4 {}", class)
    };

    view! {
        <h2 class=title_class>
            {children()}
        </h2>
    }
}
```

### Containerコンポーネント

```rust
// app/src/components/container/mod.rs
use leptos::prelude::*;

#[component]
pub fn Container(
    #[prop(optional, into)] class: String,
    children: Children
) -> impl IntoView {
    let container_class = if class.is_empty() {
        "max-w-7xl mx-auto px-4 md:px-6 lg:px-8".to_string()
    } else {
        format!("max-w-7xl mx-auto px-4 md:px-6 lg:px-8 {}", class)
    };

    view! {
        <div class=container_class>
            {children()}
        </div>
    }
}
```

## ビルドプロセス

### CSSのビルド

`package.json`に定義されたビルドスクリプト：

```json
{
  "scripts": {
    "build:css": "tailwindcss -i style/input.css -o style/output.css"
  }
}
```

### 開発時のビルド

`scripts/dev.sh`が自動的にCSSをビルド：

```bash
#!/bin/bash
# Build Tailwind CSS with DaisyUI
echo "📦 Building Tailwind CSS styles..."
bun run build:css

# Start the development server
cargo leptos watch
```

### 本番ビルド

```bash
# CSSをビルド
bun run build:css

# Leptosアプリケーションをビルド
cargo leptos build --release
```

## 開発ワークフロー

### 1. スタイルの変更

1. `style/input.css`を編集
2. `bun run build:css`を実行（または`scripts/dev.sh`が自動実行）
3. `style/output.css`が更新される
4. Leptosが自動的にリロード

### 2. 新しいTailwindクラスの追加

新しいTailwindクラスを使用する場合：

1. コンポーネントにクラスを追加
2. `bun run build:css`を実行
3. Tailwindが自動的に必要なスタイルを生成

### 3. カスタムスタイルの追加

`style/input.css`にカスタムスタイルを追加：

```css
/* Custom Styles */
.my-custom-class {
  background-color: var(--color-primary);
  padding: 1rem;
  border-radius: var(--rounded-box);
}
```

### 4. テーマ変数の追加

`@theme`ブロックに新しい変数を追加：

```css
@theme {
  --color-custom: #ff6b6b;
  --spacing-custom: 2rem;
}
```

## トラブルシューティング

### スタイルが適用されない

**原因と対処法：**

1. **CSSがビルドされていない**
   ```bash
   bun run build:css
   ```

2. **ブラウザのキャッシュ**
   - ブラウザのキャッシュをクリア
   - ハードリロード（Cmd+Shift+R / Ctrl+Shift+R）

3. **Leptosサーバーの再起動**
   ```bash
   # 開発サーバーを停止して再起動
   cargo leptos watch
   ```

### Tailwindクラスが認識されない

**原因と対処法：**

1. **contentパスの確認**
   `tailwind.config.ts`の`content`配列に該当ファイルが含まれているか確認：
   ```typescript
   content: [
     "./app/src/**/*.rs",
     "./frontend/src/**/*.rs",
     "./server/src/**/*.rs",
   ],
   ```

2. **CSSの再ビルド**
   ```bash
   bun run build:css
   ```

3. **サーバーの再起動**
   新しいクラスを使用する場合は、開発サーバーの再起動が必要な場合があります。

### DaisyUIテーマが適用されない

**原因と対処法：**

1. **data-theme属性の確認**
   `app/src/lib.rs`で`data-theme`属性が正しく設定されているか確認：
   ```rust
   <div data-theme=effective_theme>
   ```

2. **テーマ名の確認**
   `tailwind.config.ts`で使用しているテーマ名が`themes`配列に含まれているか確認。

3. **DaisyUIプラグインの確認**
   `tailwind.config.ts`でDaisyUIプラグインが有効になっているか確認：
   ```typescript
   plugins: [daisyui],
   ```

### カスタム変数が適用されない

**原因と対処法：**

1. **@themeブロックの確認**
   `style/input.css`の`@theme`ブロックで変数が正しく定義されているか確認。

2. **CSS変数の参照方法**
   CSS変数は`var(--variable-name)`で参照：
   ```css
   .my-class {
     color: var(--color-primary);
   }
   ```

3. **CSSの再ビルド**
   変数を追加・変更した場合は、CSSを再ビルド：
   ```bash
   bun run build:css
   ```

## 参考資料

### 公式ドキュメント

- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs)
- [DaisyUI Documentation](https://daisyui.com/)
- [Leptos Documentation](https://leptos.dev/)

### プロジェクト内の関連ファイル

- `style/input.css` - Tailwind CSS設定とカスタムスタイル
- `tailwind.config.ts` - Tailwind CSS設定ファイル
- `app/src/lib.rs` - スタイルシートの読み込みとテーマ管理
- `app/src/components/` - UIコンポーネント実装例
- `app/src/concepts/theme/` - テーマ管理Concept

### 関連ドキュメント

- [Legible Architecture ガイドライン](../.cursor/rules/) - コンポーネント設計原則
- [コンポーネント仕様書](../app/src/components/) - 各コンポーネントの仕様

## まとめ

このプロジェクトでは、Tailwind CSS v4とDaisyUIを組み合わせることで、以下のメリットを実現しています：

- **開発効率**: ユーティリティクラスによる迅速なスタイリング
- **一貫性**: DaisyUIによる統一されたデザインシステム
- **柔軟性**: カスタムテーマとスタイルによる拡張性
- **パフォーマンス**: Tailwindの最適化による軽量なCSS出力
- **保守性**: 明確なファイル構造とドキュメント

新しいスタイルを追加する際は、このドキュメントを参照し、既存のパターンに従って実装してください。
