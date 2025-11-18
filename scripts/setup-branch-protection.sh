#!/bin/bash

# GitHub ブランチ保護ルール設定スクリプト
# 使用方法: ./scripts/setup-branch-protection.sh

set -e

REPO_OWNER="otomatty"
REPO_NAME="continuum"

echo "Setting up branch protection rules for ${REPO_OWNER}/${REPO_NAME}..."

# GitHub CLIがインストールされているか確認
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed."
    echo "Please install it from: https://cli.github.com/"
    exit 1
fi

# GitHub CLIで認証されているか確認
if ! gh auth status &> /dev/null; then
    echo "Error: GitHub CLI is not authenticated."
    echo "Please run: gh auth login"
    exit 1
fi

# mainブランチの保護設定
echo "Setting up protection for main branch..."
gh api repos/${REPO_OWNER}/${REPO_NAME}/branches/main/protection \
  --method PUT \
  --input - <<EOF
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["CI", "Build"]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false
  },
  "restrictions": null,
  "required_linear_history": false,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": false
}
EOF

echo "✅ Main branch protection configured"

# developブランチの保護設定
echo "Setting up protection for develop branch..."
gh api repos/${REPO_OWNER}/${REPO_NAME}/branches/develop/protection \
  --method PUT \
  --input - <<EOF
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["CI", "Build"]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false
  },
  "restrictions": null,
  "required_linear_history": false,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_conversation_resolution": true,
  "lock_branch": false,
  "allow_fork_syncing": false
}
EOF

echo "✅ Develop branch protection configured"

echo ""
echo "Branch protection rules have been set up successfully!"
echo ""
echo "Note: The restriction that main only accepts PRs from develop"
echo "      needs to be enforced manually or via GitHub Actions."
echo "      See docs/03_plans/continuum/github-branch-protection.md for details."

