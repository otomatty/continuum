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
# main/developブランチへの直接コミットを防ぐ

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
        echo "❌ Error: Direct commits to '$branch' branch are not allowed."
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

exit 0
EOF

chmod +x "$PRE_COMMIT_HOOK"

echo "✅ Git hooks installed successfully!"
echo "Protected branches: main, develop"
echo ""
echo "The pre-commit hook will now prevent direct commits to main/develop branches."

