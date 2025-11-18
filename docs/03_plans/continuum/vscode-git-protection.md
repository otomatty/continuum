# VSCode Git操作保護設定

## 概要

VSCodeでのGit操作において、main/developブランチへの直接コミットを防ぐための設定です。

## 設定内容

### 1. Git Pre-commit Hook

`.git/hooks/pre-commit` にフックを設定しています。

**機能:**
- main/developブランチへの直接コミットを検出
- コミットを拒否し、エラーメッセージを表示
- 作業ブランチの作成を促す

**動作:**
```bash
# mainブランチでコミットを試みた場合
$ git commit -m "test"
❌ Error: Direct commits to 'main' branch are not allowed.

Please create a feature/bugfix/hotfix branch and create a PR instead:
  git checkout -b feature/your-feature-name
  git commit ...
  git push origin feature/your-feature-name

Then create a pull request on GitHub.
```

### 2. VSCode設定

`.vscode/settings.json` に以下の設定を追加：

- **git.branchProtection**: 保護するブランチリスト（main, develop）
- **git.branchProtectionPrompt**: 保護されたブランチでの操作時に常に確認
- **git.confirmSync**: 同期前に確認
- **git.enableSmartCommit**: スマートコミットを無効化（明示的なコミットのみ）
- **git.confirmEmptyCommits**: 空のコミットを確認
- **git.allowForcePush**: 強制プッシュを無効化

### 3. 推奨拡張機能

`.vscode/extensions.json` に推奨拡張機能を追加：

- **rust-lang.rust-analyzer**: Rust開発用
- **vadimcn.vscode-lldb**: Rustデバッガー
- **serayuzgur.crates**: Cargo.toml管理
- **tamasfe.even-better-toml**: TOMLファイル編集
- **esbenp.prettier-vscode**: Markdown等のフォーマッター
- **eamodio.gitlens**: Git操作の強化

## 使用方法

### 新規リポジトリに設定を適用

1. **Git hooksをコピー**
   ```bash
   # 既存のリポジトリに設定をコピー
   cp .git/hooks/pre-commit /path/to/other/repo/.git/hooks/pre-commit
   chmod +x /path/to/other/repo/.git/hooks/pre-commit
   ```

2. **VSCode設定をコピー**
   ```bash
   # .vscodeディレクトリごとコピー
   cp -r .vscode /path/to/other/repo/
   ```

### チームメンバーへの共有

1. **Git hooksの共有**
   - Git hooksは`.git/hooks/`に配置されるため、通常はリポジトリに含まれません
   - 代わりに、`scripts/install-git-hooks.sh`を作成して共有

2. **VSCode設定の共有**
   - `.vscode/`ディレクトリはリポジトリに含まれます
   - チームメンバーがリポジトリをクローンすると自動的に適用されます

## Git Hooksのインストールスクリプト

チームメンバーが簡単にGit hooksを設定できるように、インストールスクリプトを作成します。

### scripts/install-git-hooks.sh

```bash
#!/bin/bash

# Git hooksインストールスクリプト

HOOKS_DIR=".git/hooks"
PRE_COMMIT_HOOK="$HOOKS_DIR/pre-commit"

# pre-commit hookを作成
cat > "$PRE_COMMIT_HOOK" << 'EOF'
#!/bin/bash

# Git pre-commit hook
# main/developブランチへの直接コミットを防ぐ

current_branch=$(git symbolic-ref --short HEAD 2>/dev/null)

if [ -z "$current_branch" ]; then
    exit 0
fi

protected_branches=("main" "develop")

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
```

## 動作確認

### テスト方法

1. **mainブランチでコミットを試みる**
   ```bash
   git checkout main
   echo "test" > test.txt
   git add test.txt
   git commit -m "test"  # エラーが表示される
   ```

2. **developブランチでコミットを試みる**
   ```bash
   git checkout develop
   echo "test" > test.txt
   git add test.txt
   git commit -m "test"  # エラーが表示される
   ```

3. **作業ブランチでコミット**
   ```bash
   git checkout -b feature/test
   echo "test" > test.txt
   git add test.txt
   git commit -m "test"  # 成功
   ```

## 注意事項

1. **Git hooksの制限**
   - Git hooksはローカルのみで動作します
   - リモートリポジトリには含まれません
   - チームメンバー全員が個別に設定する必要があります

2. **VSCode設定の制限**
   - VSCode設定は警告を表示しますが、強制的には防げません
   - Git hooksと組み合わせることで、より強力な保護が可能です

3. **緊急時の回避方法**
   - 緊急時は `git commit --no-verify` でフックをスキップできます
   - ただし、ブランチ保護ルールにより、PRなしではマージできません

## 関連ドキュメント

- [ブランチ保護設定](./github-branch-protection.md)
- [Git Hooks公式ドキュメント](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)

