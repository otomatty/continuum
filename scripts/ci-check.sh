#!/bin/bash

# CIと同じチェックをローカルで実行するスクリプト
# 使用方法: ./scripts/ci-check.sh
# または: bun run ci:check

set -e

# カラー出力の設定
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Running CI checks locally..."
echo ""

# 1. フォーマットチェック
echo -e "${YELLOW}[1/4] Checking code formatting...${NC}"
if cargo fmt --check --all; then
    echo -e "${GREEN}✅ Formatting check passed${NC}"
else
    echo -e "${RED}❌ Formatting check failed${NC}"
    echo "Run 'cargo fmt --all' to fix formatting issues"
    exit 1
fi
echo ""

# 2. Clippyチェック
echo -e "${YELLOW}[2/4] Running Clippy...${NC}"
if cargo clippy --all-targets -- -D warnings; then
    echo -e "${GREEN}✅ Clippy check passed${NC}"
else
    echo -e "${RED}❌ Clippy check failed${NC}"
    echo "Fix the warnings/errors above"
    exit 1
fi
echo ""

# 3. テスト実行
echo -e "${YELLOW}[3/4] Running tests...${NC}"
if cargo test --all; then
    echo -e "${GREEN}✅ Tests passed${NC}"
else
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
fi
echo ""

# 4. ビルドチェック
echo -e "${YELLOW}[4/4] Checking build...${NC}"
if cargo check --all; then
    echo -e "${GREEN}✅ Build check passed${NC}"
else
    echo -e "${RED}❌ Build check failed${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}🎉 All CI checks passed!${NC}"
echo "You can safely commit your changes."

