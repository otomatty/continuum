# スタイリングガイド

このプロジェクトでは、**Tailwind CSS v4**と**DaisyUI**を使用してスタイリングを実装しています。

## セットアップ

### 依存関係

以下のパッケージがインストールされています：

- `tailwindcss@^4.1.17` - Tailwind CSS v4
- `daisyui@^5.5.1-beta.2` - DaisyUIコンポーネントライブラリ
- `@tailwindcss/cli@^4.1.17` - Tailwind CSS CLI

### ファイル構造

```
style/
├── main.scss          # メインスタイルファイル（Leptosがコンパイル）
└── output.css         # Tailwind CSS生成ファイル（開発用）

app/src/components/
├── mod.rs            # コンポーネントモジュール
├── navbar.rs         # ナビゲーションバーコンポーネント
├── button.rs         # ボタンコンポーネント
└── card.rs           # カードコンポーネント
```

## 使用方法

### Tailwind CSSクラスの使用

Tailwind CSSのユーティリティクラスを直接使用できます：

```rust
view! {
    <div class="container mx-auto py-8">
        <h1 class="text-5xl font-bold mb-4">"Title"</h1>
        <p class="text-xl text-gray-600">"Description"</p>
    </div>
}
```

### DaisyUI風コンポーネントクラス

カスタムコンポーネントクラスも使用できます：

- `.btn`, `.btn-primary`, `.btn-secondary`, `.btn-ghost` - ボタンスタイル
- `.card` - カードコンテナ
- `.navbar` - ナビゲーションバー

### カスタムコンポーネントの使用

```rust
use components::{navbar::Navbar, button::Button, button::ButtonVariant, card::{Card, CardTitle, CardBody}};

view! {
    <Navbar/>
    <Card>
        <CardTitle>"Card Title"</CardTitle>
        <CardBody>
            <p>"Card content"</p>
            <Button variant=ButtonVariant::Primary>
                "Click Me"
            </Button>
        </CardBody>
    </Card>
}
```

## 開発ワークフロー

### スタイルの変更

1. `style/main.scss`を編集
2. Leptosが自動的にSCSSをコンパイル（`cargo leptos watch`実行時）
3. ブラウザが自動リロード

### Tailwind CSSクラスの追加

新しいTailwind CSSクラスを使用する場合、Leptosが自動的に検出してコンパイルします。

### カスタムスタイルの追加

`style/main.scss`にカスタムスタイルを追加できます：

```scss
/* Custom Styles */
.my-custom-class {
  @apply bg-primary text-white p-4 rounded-lg;
}
```

## テーマカスタマイズ

`style/main.scss`の`@theme`ブロックでテーマをカスタマイズできます：

```scss
@theme {
  --color-primary: #3b82f6;
  --color-secondary: #8b5cf6;
  --color-base-100: #ffffff;
  --color-base-200: #f2f2f2;
  // ... その他のカスタムプロパティ
}
```

## トラブルシューティング

### スタイルが適用されない

1. サーバーを再起動：`cargo leptos watch`
2. ブラウザのキャッシュをクリア
3. `style/main.scss`が正しく保存されているか確認

### Tailwind CSSクラスが認識されない

Leptosが自動的にTailwind CSSをコンパイルしますが、新しいクラスを使用する場合はサーバーの再起動が必要な場合があります。

## 参考資料

- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs)
- [DaisyUI Documentation](https://daisyui.com/)
- [Leptos Documentation](https://leptos.dev/)

