# Phase 2-3: キャッシュ戦略実装

## 概要

GitHub APIのレート制限対策とパフォーマンス向上のため、Cloudflare KV互換のキャッシュレイヤーを実装します。

## 目標

- GitHub API呼び出し回数の削減
- レスポンス時間の短縮
- Cloudflare KV互換のキャッシュ実装
- キャッシュ無効化戦略の確立

## 実装内容

### 1. ディレクトリ構造

```
app/src/cache/
├── mod.rs              # モジュール定義
├── kv.rs               # Cloudflare KV実装
├── memory.rs           # ローカル開発用メモリキャッシュ
└── strategy.rs         # キャッシュ戦略定義
```

### 2. キャッシュ戦略

#### 2.1 キャッシュ対象データ

| データ種別 | TTL | 無効化条件 |
|:---|:---|:---|
| 組織情報 | 1時間 | 手動無効化 |
| リポジトリ一覧 | 30分 | 新規リポジトリ追加時 |
| リポジトリ詳細 | 1時間 | リポジトリ更新時 |
| ユーザー情報 | 30分 | ユーザープロフィール更新時 |
| コントリビューション | 15分 | 新しいコントリビューション発生時 |
| ダッシュボードサマリー | 10分 | アクティビティ発生時 |

#### 2.2 キャッシュキー設計

**命名規則:** `{prefix}:{type}:{identifier}:{params}`

**例:**
- `org:info:organization-name`
- `repo:list:organization-name:page:1:per_page:30`
- `repo:detail:organization-name/repo-name`
- `user:profile:username`
- `user:contributions:username:year:2024`
- `dashboard:summary:organization-name`

### 3. キャッシュインターフェース

```rust
pub trait Cache {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned;
    
    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError>
    where
        T: Serialize;
    
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
    
    async fn invalidate_pattern(&self, pattern: &str) -> Result<(), CacheError>;
}
```

### 4. Cloudflare KV実装 (`kv.rs`)

**機能:**
- Cloudflare KVへの接続
- キー・バリューの読み書き
- TTL設定
- パターンマッチによる一括削除

**実装:**
```rust
pub struct CloudflareKV {
    namespace: KvNamespace,
}

impl Cache for CloudflareKV {
    // Cloudflare KV APIを使用した実装
}
```

**注意点:**
- Cloudflare Workers環境でのみ動作
- ローカル開発ではメモリキャッシュを使用

### 5. メモリキャッシュ実装 (`memory.rs`)

**機能:**
- ローカル開発用のインメモリキャッシュ
- TTL管理
- LRUキャッシュ（オプション）

**実装:**
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub struct MemoryCache {
    data: Arc<RwLock<HashMap<String, CachedValue>>>,
}

struct CachedValue {
    value: String,
    expires_at: Instant,
}

impl Cache for MemoryCache {
    // インメモリキャッシュ実装
}
```

### 6. キャッシュ戦略実装 (`strategy.rs`)

**機能:**
- データ種別ごとのTTL設定
- キャッシュキー生成
- 無効化パターン定義

**実装:**
```rust
pub struct CacheStrategy {
    pub ttl: Duration,
    pub key_prefix: String,
    pub invalidation_patterns: Vec<String>,
}

impl CacheStrategy {
    pub fn for_organization() -> Self {
        Self {
            ttl: Duration::from_secs(3600),
            key_prefix: "org:info".to_string(),
            invalidation_patterns: vec!["org:info:*".to_string()],
        }
    }
    
    // 他のデータ種別用の戦略定義
}
```

### 7. GitHub APIクライアントとの統合

**実装方針:**
- GitHub APIクライアントにキャッシュレイヤーを追加
- キャッシュヒット時はAPI呼び出しをスキップ
- キャッシュミス時はAPI呼び出し後、結果をキャッシュ

**例:**
```rust
pub async fn get_organization_with_cache(
    client: &GitHubClient,
    cache: &dyn Cache,
    org_name: &str,
) -> Result<Organization, Error> {
    let cache_key = format!("org:info:{}", org_name);
    
    // キャッシュから取得を試みる
    if let Some(cached) = cache.get::<Organization>(&cache_key).await? {
        return Ok(cached);
    }
    
    // キャッシュミス時はAPI呼び出し
    let org = client.get_organization(org_name).await?;
    
    // 結果をキャッシュ
    let strategy = CacheStrategy::for_organization();
    cache.set(&cache_key, &org, strategy.ttl).await?;
    
    Ok(org)
}
```

### 8. キャッシュ無効化

**無効化タイミング:**
- 手動無効化（管理画面から）
- Webhook経由（GitHub Events）
- TTL経過時（自動）

**実装:**
```rust
pub async fn invalidate_cache(
    cache: &dyn Cache,
    pattern: &str,
) -> Result<(), CacheError> {
    cache.invalidate_pattern(pattern).await
}
```

### 9. 依存関係

```toml
# app/Cargo.toml に追加
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 10. テスト

**テスト項目:**
- キャッシュの読み書きテスト
- TTLの動作テスト
- 無効化テスト
- パターンマッチテスト

## 実装手順

1. `app/src/cache/` ディレクトリ作成
2. `Cache` トレイト定義
3. `memory.rs` でメモリキャッシュ実装（ローカル開発用）
4. `strategy.rs` でキャッシュ戦略定義
5. GitHub APIクライアントにキャッシュ統合
6. `kv.rs` でCloudflare KV実装（Phase 3で実装）
7. テスト作成

## パフォーマンス目標

- API呼び出し回数: 50%削減
- 平均レスポンス時間: 30%短縮
- キャッシュヒット率: 70%以上

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- Cloudflare Workers設定: `../phase3-cloudflare-workers/README.md`
- APIエンドポイント: `../phase2-api-endpoints/README.md`

