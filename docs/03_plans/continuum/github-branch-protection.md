# GitHub ブランチ保護設定

## 概要

ContinuumプロジェクトのGitHubリポジトリにおけるブランチ保護ルールの設定手順です。

## ブランチ戦略

### ブランチ構成

```
main (本番環境)
  ↑ (PR必須、developまたはhotfix/*からのみ)
develop (開発環境)
  ↑ (PR必須)
feature/* (機能開発ブランチ)
bugfix/* (バグ修正ブランチ)
hotfix/* (緊急修正ブランチ、mainから直接作成)
```

### ブランチの役割

- **main**: 本番環境にデプロイされるブランチ。developブランチまたはhotfix/*ブランチからのPRのみマージ可能。
- **develop**: 開発環境にデプロイされるブランチ。すべての作業ブランチからのPRを受け付ける。
- **feature/***: 新機能開発用ブランチ
- **bugfix/***: バグ修正用ブランチ
- **hotfix/***: 緊急修正用ブランチ（mainから直接作成）

## ブランチ保護ルール設定

### 1. mainブランチの保護設定

**設定項目:**

1. **Require a pull request before merging**
   - ✅ 有効化
   - Required number of approvals: `0`（レビュワーの承認は不要）
   - Dismiss stale pull request approvals when new commits are pushed: ❌ 無効化
   - **Do not allow bypassing the above settings**: ✅ 有効化（バイパス権限を無効化）

2. **Require status checks to pass before merging**
   - ✅ 有効化
   - Required status checks:
     - CI (テスト)
     - Build (ビルド)

3. **Require conversation resolution before merging**
   - ✅ 有効化

4. **Require linear history**
   - ✅ 有効化

5. **Restrict who can push to matching branches**
   - ✅ 有効化
   - 管理者も含めて直接pushは禁止（PR必須）

6. **Branch name pattern**: `main`

7. **Restrict pushes that create files larger than 100 MB**
   - ✅ 有効化

**重要な設定:**
- **Restrict pushes that create files matching a specified pattern**: 設定しない（developからのPRを受け付けるため）

### 2. developブランチの保護設定

**設定項目:**

1. **Require a pull request before merging**
   - ✅ 有効化
   - Required number of approvals: `0`（レビュワーの承認は不要）
   - Dismiss stale pull request approvals when new commits are pushed: ❌ 無効化
   - **Do not allow bypassing the above settings**: ✅ 有効化（バイパス権限を無効化）

2. **Require status checks to pass before merging**
   - ✅ 有効化
   - Required status checks:
     - CI (テスト)
     - Build (ビルド)

3. **Require conversation resolution before merging**
   - ✅ 有効化

4. **Require linear history**
   - ❌ 無効化（開発ブランチのため）

5. **Restrict who can push to matching branches**
   - ✅ 有効化
   - 管理者も含めて直接pushは禁止（PR必須）

6. **Branch name pattern**: `develop`

7. **Restrict pushes that create files larger than 100 MB**
   - ✅ 有効化

### 3. mainブランチへのPR制限（developからのみ）

**実装方法:**

GitHubの標準機能では「developブランチからのPRのみ受け付ける」という設定は直接できません。以下のいずれかの方法で実現します：

#### 方法1: GitHub Actionsで自動チェック（推奨）

`.github/workflows/pr-check.yml` が自動的に以下をチェックします：

- mainブランチへのPRはdevelopブランチまたはhotfix/*ブランチからのみ許可
- developブランチへのPRは作業ブランチからのみ許可（mainは不可）

このワークフローはPR作成時に自動実行され、ルール違反がある場合はPRをブロックします。

#### 方法2: ブランチ保護ルールで警告

GitHubのブランチ保護ルールでは直接制限できませんが、以下の設定で警告を表示できます：

1. **Require pull request reviews** を有効化
2. **Required reviewers** にレビュアーを設定
3. レビュアーが適切なブランチからのPRかどうかを確認

#### 方法3: 手動での確認

PR作成時に、レビュアーが以下を確認：
- mainへのPRはdevelopブランチまたはhotfix/*ブランチからのみ
- developへのPRは任意の作業ブランチから

## 設定手順

### 方法1: スクリプトを使用した自動設定（推奨）

プロジェクトルートで以下のコマンドを実行：

```bash
# GitHub CLIがインストールされていることを確認
gh --version

# GitHub CLIで認証（未認証の場合）
gh auth login

# ブランチ保護ルールを設定
./scripts/setup-branch-protection.sh
```

スクリプトは以下を自動的に設定します：
- mainブランチの保護ルール
- developブランチの保護ルール

### 方法2: GitHub Web UIでの設定

1. **リポジトリにアクセス**
   - https://github.com/otomatty/continuum

2. **Settings > Branches に移動**

3. **Add branch protection rule をクリック**

4. **mainブランチの保護設定**
   - Branch name pattern: `main`
   - 上記の設定項目を有効化
   - Create をクリック

5. **developブランチの保護設定**
   - Branch name pattern: `develop`
   - 上記の設定項目を有効化
   - Create をクリック

### 方法3: GitHub CLIでの手動設定

```bash
# mainブランチの保護設定
gh api repos/otomatty/continuum/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["CI","Build"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true}' \
  --field restrictions=null

# developブランチの保護設定
gh api repos/otomatty/continuum/branches/develop/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["CI","Build"]}' \
  --field enforce_admins=false \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true}' \
  --field restrictions=null
```

### PRルールチェック（自動）

`.github/workflows/pr-check.yml` が自動的に以下をチェックします：

- mainブランチへのPRはdevelopブランチまたはhotfix/*ブランチからのみ許可
- developブランチへのPRは作業ブランチからのみ許可（mainは不可）

このワークフローはPR作成時に自動実行され、ルール違反がある場合はPRをブロックします。

## 作業フロー

### 1. 新機能開発

```bash
# developブランチから作業ブランチを作成
git checkout develop
git pull origin develop
git checkout -b feature/new-feature

# 作業・コミット
git add .
git commit -m "feat: add new feature"

# リモートにpush
git push origin feature/new-feature

# GitHubでPRを作成（developへのPR）
```

### 2. バグ修正

```bash
# developブランチから作業ブランチを作成
git checkout develop
git pull origin develop
git checkout -b bugfix/fix-bug

# 作業・コミット
git add .
git commit -m "fix: fix bug"

# リモートにpush
git push origin bugfix/fix-bug

# GitHubでPRを作成（developへのPR）
```

### 3. developからmainへのマージ

```bash
# developブランチを最新に更新
git checkout develop
git pull origin develop

# GitHubでPRを作成（mainへのPR）
# PRが承認・マージされたら、mainブランチを更新
git checkout main
git pull origin main
```

### 4. 緊急修正（hotfix）

```bash
# mainブランチから作業ブランチを作成
git checkout main
git pull origin main
git checkout -b hotfix/urgent-fix

# 作業・コミット
git add .
git commit -m "hotfix: urgent fix"

# リモートにpush
git push origin hotfix/urgent-fix

# GitHubでPRを作成（mainへのPR）
# hotfix/*ブランチからmainへのPRは許可されています
# マージ後、developにもマージ
git checkout develop
git merge main
git push origin develop
```

## PRテンプレート

### PR作成時のチェックリスト

**developへのPR:**
- [ ] 作業ブランチからdevelopへのPRである
- [ ] テストが通過している
- [ ] コードレビューを依頼した
- [ ] 関連ドキュメントを更新した

**mainへのPR:**
- [ ] developブランチまたはhotfix/*ブランチからmainへのPRである
- [ ] すべてのテストが通過している
- [ ] コードレビューが承認されている
- [ ] 本番環境へのデプロイ準備ができている

## 注意事項

1. **mainブランチへの直接pushは禁止**
   - 管理者でも直接pushは避ける
   - 必ずPR経由でマージする

2. **developブランチへの直接pushは禁止**
   - 管理者でも直接pushは避ける
   - 必ずPR経由でマージする

3. **作業は必ず作業ブランチで行う**
   - `feature/*`, `bugfix/*`, `hotfix/*` などのブランチを作成
   - main/developブランチで直接作業しない

4. **コミットメッセージ**
   - コンベンショナルコミット形式を使用
   - `feat:`, `fix:`, `docs:`, `refactor:`, `test:` など

## 関連ドキュメント

- [実装計画書](./20250101_01_setup-plan.md)
- [CI/CD設定](./phase3-cicd/README.md)

