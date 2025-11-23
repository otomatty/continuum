#!/bin/bash

# Git hooksインストールスクリプト
# 使用方法: ./scripts/install-git-hooks.sh

set -e

HOOKS_DIR=".git/hooks"
PRE_COMMIT_HOOK="$HOOKS_DIR/pre-commit"

# .gitディレクトリが存在するか確認
if [ ! -d ".git" ]; then
    echo "Error: This script must be run from the root of a git repository"
    exit 1
fi

# hooksディレクトリが存在しない場合は作成
if [ ! -d "$HOOKS_DIR" ]; then
    mkdir -p "$HOOKS_DIR"
fi

# pre-commit hookを作成
cat > "$PRE_COMMIT_HOOK" << 'EOF'
#!/bin/bash

# Git pre-commit hook
# 1. main/developブランチへの直接コミットを防ぐ
# 2. CIと同じチェックを実行（フォーマット、Clippy）

# カラー出力の設定
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 現在のブランチ名を取得
current_branch=$(git symbolic-ref --short HEAD 2>/dev/null)

# ブランチ名が取得できない場合（detached HEAD状態など）はスキップ
if [ -z "$current_branch" ]; then
    exit 0
fi

# 保護されたブランチリスト
protected_branches=("main" "develop")

# 保護されたブランチへのコミットをチェック
for branch in "${protected_branches[@]}"; do
    if [ "$current_branch" = "$branch" ]; then
        echo -e "${RED}❌ Error: Direct commits to '$branch' branch are not allowed.${NC}"
        echo ""
        echo "Please create a feature/bugfix/hotfix branch and create a PR instead:"
        echo "  git checkout -b feature/your-feature-name"
        echo "  git commit ..."
        echo "  git push origin feature/your-feature-name"
        echo ""
        echo "Then create a pull request on GitHub."
        exit 1
    fi
done

# CIチェックを実行（SKIP_CI_CHECK環境変数でスキップ可能）
if [ -z "$SKIP_CI_CHECK" ]; then
    echo -e "${YELLOW}🔍 Running pre-commit checks...${NC}"
    
    # 1. フォーマットチェック
    echo -e "${YELLOW}[1/2] Checking code formatting...${NC}"
    if ! cargo fmt --check --all > /dev/null 2>&1; then
        echo -e "${RED}❌ Formatting check failed${NC}"
        echo "Run 'cargo fmt --all' to fix formatting issues"
        echo "Or set SKIP_CI_CHECK=1 to skip this check"
        exit 1
    fi
    echo -e "${GREEN}✅ Formatting check passed${NC}"
    
    # 2. Clippyチェック（警告をエラーとして扱う）
    echo -e "${YELLOW}[2/2] Running Clippy...${NC}"
    if ! cargo clippy --all-targets -- -D warnings > /dev/null 2>&1; then
        echo -e "${RED}❌ Clippy check failed${NC}"
        echo "Run 'cargo clippy --all-targets -- -D warnings' to see details"
        echo "Or set SKIP_CI_CHECK=1 to skip this check"
        exit 1
    fi
    echo -e "${GREEN}✅ Clippy check passed${NC}"
    
    echo -e "${GREEN}✅ All pre-commit checks passed!${NC}"
fi

exit 0
EOF

chmod +x "$PRE_COMMIT_HOOK"

echo "✅ Git hooks installed successfully!"
echo "Protected branches: main, develop"
echo ""
echo "The pre-commit hook will now:"
echo "  1. Prevent direct commits to main/develop branches"
echo "  2. Run CI checks (formatting, Clippy) before each commit"
echo ""
echo "To skip CI checks temporarily, use:"
echo "  SKIP_CI_CHECK=1 git commit -m 'your message'"

