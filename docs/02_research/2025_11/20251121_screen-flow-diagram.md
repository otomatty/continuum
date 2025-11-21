# 画面遷移図 (Screen Flow Diagram)

**作成日:** 2025-11-21  
**関連ドキュメント:** 20251121_screen-design-proposal.md, PRD.md  
**スタイル:** ダークモード準拠

## Mermaid形式の画面遷移図

> **Note:** 全てのダイアグラムはダークモード準拠のカラースキームを使用しています。各ノードはダークな背景色と明るいストローク、読みやすいテキストカラーで表示されます。

### 全体フロー

```mermaid
graph TB
    Landing["ランディングページ<br/>/"]
    Auth["GitHub OAuth"]
    Dashboard["ダッシュボード<br/>/dashboard"]
    Knowledge["知見共有<br/>/knowledge"]
    KnowledgeDetail["ナレッジ詳細<br/>/knowledge/:id"]
    Contributors["コントリビューター一覧<br/>/contributors"]
    Repositories["リポジトリ一覧<br/>/repositories"]
    RepositoryDetail["リポジトリ詳細<br/>/repository/:owner/:repo"]
    Portfolio["ポートフォリオ<br/>/portfolio/:username"]
    Settings["設定<br/>/settings"]
    NotFound["404 Not Found"]
    
    %% ランディングから認証
    Landing -->|GitHub OAuth でログイン| Auth
    Auth -->|認証成功| Dashboard
    Auth -->|認証失敗| Landing
    
    %% ダッシュボードからの遷移
    Dashboard -->|ナビゲーション: Knowledge| Knowledge
    Dashboard -->|ナビゲーション: Contributors| Contributors
    Dashboard -->|ナビゲーション: Repositories| Repositories
    Dashboard -->|ユーザーメニュー: My Portfolio| Portfolio
    Dashboard -->|ユーザーメニュー: Settings| Settings
    Dashboard -->|タイムライン: ユーザー名クリック| Portfolio
    Dashboard -->|タイムライン: リポジトリ名クリック| RepositoryDetail
    Dashboard -->|ランキング: ユーザー名クリック| Portfolio
    Dashboard -->|リポジトリ一覧: 詳細ボタン| RepositoryDetail
    
    %% 知見共有フロー
    Knowledge -->|カードクリック| KnowledgeDetail
    Knowledge -->|投稿者クリック| Portfolio
    Knowledge -->|ナビゲーション: Dashboard| Dashboard
    KnowledgeDetail -->|投稿者クリック| Portfolio
    KnowledgeDetail -->|パンくずリスト| Knowledge
    
    %% コントリビューターフロー
    Contributors -->|カードクリック / View Portfolio| Portfolio
    Contributors -->|ナビゲーション: Dashboard| Dashboard
    
    %% リポジトリフロー
    Repositories -->|詳細ボタン| RepositoryDetail
    Repositories -->|ナビゲーション: Dashboard| Dashboard
    RepositoryDetail -->|コントリビューター名クリック| Portfolio
    RepositoryDetail -->|パンくずリスト| Repositories
    
    %% ポートフォリオフロー
    Portfolio -->|リポジトリ名クリック| RepositoryDetail
    Portfolio -->|戻るボタン| Dashboard
    
    %% 設定フロー
    Settings -->|戻るボタン| Dashboard
    Settings -->|ナビゲーション: Dashboard| Dashboard
    
    %% 404
    Dashboard -.->|存在しないURL| NotFound
    NotFound -->|ホームに戻る| Dashboard
    
    %% スタイル定義 (ダークモード)
    classDef publicPage fill:#1a237e,stroke:#64b5f6,stroke-width:2px,color:#e3f2fd
    classDef authPage fill:#bf360c,stroke:#ff9800,stroke-width:2px,color:#fff3e0
    classDef mainPage fill:#4a148c,stroke:#ab47bc,stroke-width:2px,color:#f3e5f5
    classDef detailPage fill:#1b5e20,stroke:#66bb6a,stroke-width:2px,color:#e8f5e9
    classDef errorPage fill:#b71c1c,stroke:#ef5350,stroke-width:2px,color:#ffebee
    
    class Landing publicPage
    class Auth authPage
    class Dashboard,Knowledge,Contributors,Repositories mainPage
    class KnowledgeDetail,RepositoryDetail,Portfolio,Settings detailPage
    class NotFound errorPage
```

---

## ユーザージャーニー別フロー

### 1. 初回訪問ユーザーのジャーニー

```mermaid
graph LR
    A[ランディングページ訪問]
    B[価値提案を理解]
    C[GitHub OAuth でログイン]
    D[ダッシュボード表示]
    E[組織活動を閲覧]
    F[気になるユーザーを<br/>クリック]
    G[ポートフォリオで<br/>実績を確認]
    
    A --> B --> C --> D --> E --> F --> G
    
    classDef journeyStep fill:#1e3a5f,stroke:#64b5f6,stroke-width:2px,color:#e3f2fd
    class A,B,C,D,E,F,G journeyStep
```

### 2. 現役社員のジャーニー (自分の実績確認)

```mermaid
graph LR
    A[ログイン]
    B[ダッシュボード]
    C[ユーザーメニューから<br/>マイポートフォリオ]
    D[自分の<br/>コントリビューション<br/>グラフを確認]
    E[PDFエクスポート]
    F[職務経歴書に添付]
    
    A --> B --> C --> D --> E --> F
    
    classDef employeeStep fill:#1b5e20,stroke:#66bb6a,stroke-width:2px,color:#e8f5e9
    class A,B,C,D,E,F employeeStep
```

### 3. マネージャーのジャーニー (組織活動の把握)

```mermaid
graph LR
    A[ログイン]
    B[ダッシュボード]
    C[組織活動サマリーを確認]
    D[外部PR数の増加を確認]
    E[リポジトリ詳細へ]
    F[コントリビューター<br/>構成を確認]
    G[オープン文化の<br/>効果を実感]
    
    A --> B --> C --> D --> E --> F --> G
    
    classDef managerStep fill:#e65100,stroke:#ffb74d,stroke-width:2px,color:#fff3e0
    class A,B,C,D,E,F,G managerStep
```

### 4. 元社員/外部コントリビューターのジャーニー

```mermaid
graph LR
    A[ログイン]
    B[ダッシュボード]
    C[自分の名前が<br/>タイムラインに表示]
    D[マイポートフォリオへ]
    E[在籍中の実績を確認]
    F[公開URLをシェア]
    G[SNSや職務経歴書に掲載]
    
    A --> B --> C --> D --> E --> F --> G
    
    classDef alumniStep fill:#880e4f,stroke:#f06292,stroke-width:2px,color:#fce4ec
    class A,B,C,D,E,F,G alumniStep
```

### 5. 知見探索のジャーニー

```mermaid
graph LR
    A[ログイン]
    B[ナビゲーションから<br/>Knowledge]
    C[カテゴリで<br/>フィルタリング]
    D[気になる記事を<br/>クリック]
    E[記事を読む]
    F[投稿者の<br/>ポートフォリオへ]
    G[他の投稿も確認]
    
    A --> B --> C --> D --> E --> F --> G
    
    classDef knowledgeStep fill:#4a148c,stroke:#ba68c8,stroke-width:2px,color:#f3e5f5
    class A,B,C,D,E,F,G knowledgeStep
```

---

## ページ間の関係性マップ

```mermaid
graph TD
    subgraph Public
        Landing[ランディングページ]
        Auth[GitHub OAuth]
    end
    
    subgraph Main_Pages["メインページ (認証必須)"]
        Dashboard[ダッシュボード<br/>組織活動の概観]
        Knowledge[知見共有<br/>ナレッジベース]
        Contributors[コントリビューター一覧<br/>貢献者の一覧]
        Repositories[リポジトリ一覧<br/>プロジェクト一覧]
    end
    
    subgraph Detail_Pages["詳細ページ"]
        Portfolio[ポートフォリオ<br/>個人の実績]
        RepositoryDetail[リポジトリ詳細<br/>プロジェクト詳細]
        KnowledgeDetail[ナレッジ詳細<br/>記事詳細]
    end
    
    subgraph User_Pages["ユーザー設定"]
        Settings[設定<br/>個人設定]
    end
    
    Landing --> Auth
    Auth --> Dashboard
    
    Dashboard -.->|ナビゲーション| Knowledge
    Dashboard -.->|ナビゲーション| Contributors
    Dashboard -.->|ナビゲーション| Repositories
    Dashboard -.->|ユーザーメニュー| Settings
    
    Dashboard --> Portfolio
    Dashboard --> RepositoryDetail
    
    Knowledge --> KnowledgeDetail
    Knowledge --> Portfolio
    KnowledgeDetail --> Portfolio
    
    Contributors --> Portfolio
    
    Repositories --> RepositoryDetail
    RepositoryDetail --> Portfolio
    
    Portfolio --> RepositoryDetail
    
    classDef publicStyle fill:#1a237e,stroke:#64b5f6,stroke-width:3px,color:#e3f2fd
    classDef mainStyle fill:#4a148c,stroke:#ab47bc,stroke-width:3px,color:#f3e5f5
    classDef detailStyle fill:#1b5e20,stroke:#66bb6a,stroke-width:3px,color:#e8f5e9
    classDef userStyle fill:#bf360c,stroke:#ff9800,stroke-width:3px,color:#fff3e0
    
    class Landing,Auth publicStyle
    class Dashboard,Knowledge,Contributors,Repositories mainStyle
    class Portfolio,RepositoryDetail,KnowledgeDetail detailStyle
    class Settings userStyle
```

---

## 情報アーキテクチャ (IA)

```mermaid
graph TD
    Root[Continuum]
    
    Root --> Public[Public Area<br/>未認証]
    Root --> Private[Private Area<br/>認証済み]
    
    Public --> Landing["/ - ランディングページ"]
    Public --> AuthError["/auth/error - 認証エラー"]
    
    Private --> Dashboard["/dashboard - ダッシュボード"]
    Private --> KnowledgeArea[Knowledge - 知見共有]
    Private --> PeopleArea[People - 人]
    Private --> ProjectArea[Projects - プロジェクト]
    Private --> UserArea[User - ユーザー]
    
    KnowledgeArea --> KnowledgeList["/knowledge - 一覧"]
    KnowledgeArea --> KnowledgeItem["/knowledge/:id - 詳細"]
    
    PeopleArea --> ContributorsList["/contributors - 一覧"]
    PeopleArea --> PortfolioPage["/portfolio/:username - ポートフォリオ"]
    
    ProjectArea --> RepositoriesList["/repositories - 一覧"]
    ProjectArea --> RepositoryPage["/repository/:owner/:repo - 詳細"]
    
    UserArea --> SettingsPage["/settings - 設定"]
    
    Root --> ErrorPages[Error Pages]
    ErrorPages --> NotFoundPage["/404 - Not Found"]
    ErrorPages --> MaintenancePage["/maintenance - メンテナンス"]
    
    classDef rootStyle fill:#880e4f,stroke:#f06292,stroke-width:4px,font-weight:bold,color:#fce4ec
    classDef areaStyle fill:#1a237e,stroke:#5c6bc0,stroke-width:3px,font-weight:bold,color:#c5cae9
    classDef pageStyle fill:#2e7d32,stroke:#66bb6a,stroke-width:2px,color:#c8e6c9
    
    class Root rootStyle
    class Public,Private,KnowledgeArea,PeopleArea,ProjectArea,UserArea,ErrorPages areaStyle
    class Landing,AuthError,Dashboard,KnowledgeList,KnowledgeItem,ContributorsList,PortfolioPage,RepositoriesList,RepositoryPage,SettingsPage,NotFoundPage,MaintenancePage pageStyle
```

---

## ナビゲーション構造

```mermaid
graph TD
    GNav[グローバルナビゲーション<br/>全認証済みページ共通]
    
    GNav --> Logo["ロゴ<br/>→ /dashboard"]
    GNav --> MainMenu[メインメニュー]
    GNav --> UserMenu[ユーザーメニュー]
    
    MainMenu --> NavDashboard["Dashboard<br/>→ /dashboard"]
    MainMenu --> NavKnowledge["Knowledge<br/>→ /knowledge"]
    MainMenu --> NavContributors["Contributors<br/>→ /contributors"]
    MainMenu --> NavRepositories["Repositories<br/>→ /repositories"]
    
    UserMenu --> NavPortfolio["My Portfolio<br/>→ /portfolio/:me"]
    UserMenu --> NavSettings["Settings<br/>→ /settings"]
    UserMenu --> NavLogout["Logout<br/>→ GitHub OAuth Revoke"]
    
    classDef navStyle fill:#1a237e,stroke:#5c6bc0,stroke-width:2px,color:#e3f2fd
    classDef menuStyle fill:#e65100,stroke:#ff9800,stroke-width:2px,color:#fff3e0
    classDef navLinkStyle fill:#2e7d32,stroke:#66bb6a,stroke-width:1px,color:#e8f5e9
    
    class GNav navStyle
    class MainMenu,UserMenu menuStyle
    class Logo,NavDashboard,NavKnowledge,NavContributors,NavRepositories,NavPortfolio,NavSettings,NavLogout navLinkStyle
```

---

## 画面の階層構造

```mermaid
graph TD
    L0[Level 0: Entry]
    L1[Level 1: Overview]
    L2[Level 2: List]
    L3[Level 3: Detail]
    
    L0 --> Landing[ランディングページ<br/>未認証の入り口]
    
    L1 --> Dashboard[ダッシュボード<br/>組織全体の概観]
    
    L2 --> Knowledge[知見共有一覧<br/>記事の一覧]
    L2 --> Contributors[コントリビューター一覧<br/>人の一覧]
    L2 --> Repositories[リポジトリ一覧<br/>プロジェクトの一覧]
    
    L3 --> KnowledgeDetail[ナレッジ詳細<br/>記事の詳細]
    L3 --> Portfolio[ポートフォリオ<br/>個人の詳細]
    L3 --> RepositoryDetail[リポジトリ詳細<br/>プロジェクトの詳細]
    
    Landing -.->|認証| Dashboard
    Dashboard -->|ナビゲーション| Knowledge
    Dashboard -->|ナビゲーション| Contributors
    Dashboard -->|ナビゲーション| Repositories
    Dashboard -->|直接遷移| Portfolio
    Dashboard -->|直接遷移| RepositoryDetail
    
    Knowledge -->|クリック| KnowledgeDetail
    Contributors -->|クリック| Portfolio
    Repositories -->|クリック| RepositoryDetail
    
    KnowledgeDetail -->|投稿者| Portfolio
    RepositoryDetail -->|コントリビューター| Portfolio
    Portfolio -->|リポジトリ| RepositoryDetail
    
    classDef level0 fill:#b71c1c,stroke:#ef5350,stroke-width:3px,color:#ffebee
    classDef level1 fill:#1a237e,stroke:#5c6bc0,stroke-width:3px,color:#c5cae9
    classDef level2 fill:#00695c,stroke:#4db6ac,stroke-width:3px,color:#e0f2f1
    classDef level3 fill:#e65100,stroke:#ff9800,stroke-width:3px,color:#fff3e0
    
    class L0,Landing level0
    class L1,Dashboard level1
    class L2,Knowledge,Contributors,Repositories level2
    class L3,KnowledgeDetail,Portfolio,RepositoryDetail level3
```

---

## 実装時の技術マッピング

```mermaid
graph LR
    subgraph Frontend["Frontend (Leptos)"]
        Pages["Pages<br/>src/pages/"]
        Components["Components<br/>src/components/"]
        Router["Router<br/>Leptos Router"]
    end
    
    subgraph Backend["Backend (Axum on Workers)"]
        API["API Endpoints<br/>src/api/"]
        Auth["Auth Service<br/>GitHub OAuth"]
        Cache["Cache Layer<br/>Cloudflare KV"]
    end
    
    subgraph External["External Services"]
        GitHub["GitHub API v4<br/>GraphQL"]
        GitHubDiscussions["GitHub Discussions"]
    end
    
    Pages -->|SSR| Backend
    Pages -->|CSR| Components
    Router -->|Route Matching| Pages
    
    API --> Auth
    API --> Cache
    Cache --> GitHub
    API --> GitHub
    API --> GitHubDiscussions
    
    classDef frontendStyle fill:#0d47a1,stroke:#64b5f6,stroke-width:2px,color:#e3f2fd
    classDef backendStyle fill:#4a148c,stroke:#ab47bc,stroke-width:2px,color:#f3e5f5
    classDef externalStyle fill:#e65100,stroke:#ffb74d,stroke-width:2px,color:#fff3e0
    
    class Pages,Components,Router frontendStyle
    class API,Auth,Cache backendStyle
    class GitHub,GitHubDiscussions externalStyle
```

---

## まとめ

本ドキュメントでは、Continuumアプリケーションの画面遷移を以下の視点から図示しました：

1. **全体フロー:** すべての画面の遷移関係
2. **ユーザージャーニー別フロー:** 各利用者タイプの代表的な使い方
3. **ページ間の関係性マップ:** 画面のグルーピングと関係性
4. **情報アーキテクチャ (IA):** URL構造と階層
5. **ナビゲーション構造:** グローバルナビゲーションの構成
6. **画面の階層構造:** 画面の深さレベル
7. **実装時の技術マッピング:** フロントエンド/バックエンド/外部サービスの関係

これらの図を参考に、効率的かつ一貫性のある画面実装を進めることができます。

