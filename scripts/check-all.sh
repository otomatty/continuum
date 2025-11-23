#!/bin/bash

# 全てのチェック項目を順番に実行するスクリプト
# 使用方法:
#   ./scripts/check-all.sh              # すべてのチェックを実行
#   ./scripts/check-all.sh --format     # フォーマットチェックのみ
#   ./scripts/check-all.sh --clippy     # Clippyチェックのみ
#   ./scripts/check-all.sh --test       # テストのみ
#   ./scripts/check-all.sh --build      # ビルドチェックのみ
#   ./scripts/check-all.sh --skip-test  # テストをスキップして実行

set -e

# カラー出力の設定
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# チェックフラグ
CHECK_FORMAT=true
CHECK_CLIPPY=true
CHECK_TEST=true
CHECK_BUILD=true
CHECK_PR=false

# 引数解析
SKIP_TEST=false
for arg in "$@"; do
    case $arg in
        --format)
            CHECK_FORMAT=true
            CHECK_CLIPPY=false
            CHECK_TEST=false
            CHECK_BUILD=false
            ;;
        --clippy)
            CHECK_FORMAT=false
            CHECK_CLIPPY=true
            CHECK_TEST=false
            CHECK_BUILD=false
            ;;
        --test)
            CHECK_FORMAT=false
            CHECK_CLIPPY=false
            CHECK_TEST=true
            CHECK_BUILD=false
            ;;
        --build)
            CHECK_FORMAT=false
            CHECK_CLIPPY=false
            CHECK_TEST=false
            CHECK_BUILD=true
            ;;
        --skip-test)
            SKIP_TEST=true
            ;;
        --pr-check)
            CHECK_PR=true
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --format      Run formatting check only"
            echo "  --clippy      Run Clippy check only"
            echo "  --test        Run tests only"
            echo "  --build       Run build check only"
            echo "  --skip-test   Skip tests (run other checks)"
            echo "  --pr-check    Include PR branch rules check"
            echo "  --help, -h    Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                    # Run all checks"
            echo "  $0 --format           # Run formatting check only"
            echo "  $0 --skip-test        # Run all checks except tests"
            echo "  $0 --pr-check         # Run all checks including PR rules"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $arg${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# テストをスキップする場合
if [ "$SKIP_TEST" = true ]; then
    CHECK_TEST=false
fi

# 開始時刻を記録
START_TIME=$(date +%s)

# ヘッダー表示
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}           Running All Checks${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# チェックカウンター
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# チェック実行関数
run_check() {
    local check_name=$1
    local check_command=$2
    local check_number=$3
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    echo -e "${BLUE}[$check_number/$TOTAL_CHECKS] ${YELLOW}$check_name${NC}"
    echo -e "${CYAN}───────────────────────────────────────────────────────────${NC}"
    
    local check_start=$(date +%s)
    
    if eval "$check_command"; then
        local check_end=$(date +%s)
        local check_duration=$((check_end - check_start))
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        echo -e "${GREEN}✅ $check_name passed${NC} (${check_duration}s)"
        echo ""
        return 0
    else
        local check_end=$(date +%s)
        local check_duration=$((check_end - check_start))
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        echo -e "${RED}❌ $check_name failed${NC} (${check_duration}s)"
        echo ""
        return 1
    fi
}

# 1. フォーマットチェック
if [ "$CHECK_FORMAT" = true ]; then
    if ! run_check "Formatting Check" "cargo fmt --check --all" "$((PASSED_CHECKS + FAILED_CHECKS + 1))"; then
        echo -e "${YELLOW}💡 Tip: Run 'cargo fmt --all' to fix formatting issues${NC}"
        echo ""
        exit 1
    fi
fi

# 2. Clippyチェック
if [ "$CHECK_CLIPPY" = true ]; then
    if ! run_check "Clippy Check" "cargo clippy --all-targets -- -D warnings" "$((PASSED_CHECKS + FAILED_CHECKS + 1))"; then
        echo -e "${YELLOW}💡 Tip: Run 'cargo clippy --all-targets -- -D warnings' to see details${NC}"
        echo ""
        exit 1
    fi
fi

# 3. テスト実行
if [ "$CHECK_TEST" = true ]; then
    if ! run_check "Tests" "cargo test --all" "$((PASSED_CHECKS + FAILED_CHECKS + 1))"; then
        echo -e "${YELLOW}💡 Tip: Run 'cargo test --all -- --nocapture' for detailed output${NC}"
        echo ""
        exit 1
    fi
fi

# 4. ビルドチェック
if [ "$CHECK_BUILD" = true ]; then
    if ! run_check "Build Check" "cargo check --all" "$((PASSED_CHECKS + FAILED_CHECKS + 1))"; then
        echo -e "${YELLOW}💡 Tip: Run 'cargo check --all' to see compilation errors${NC}"
        echo ""
        exit 1
    fi
fi

# 5. PRブランチルールチェック（オプション）
if [ "$CHECK_PR" = true ]; then
    current_branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "")
    
    if [ -n "$current_branch" ]; then
        echo -e "${BLUE}[$((PASSED_CHECKS + FAILED_CHECKS + 1))/$((TOTAL_CHECKS + 1))] ${YELLOW}PR Branch Rules Check${NC}"
        echo -e "${CYAN}───────────────────────────────────────────────────────────${NC}"
        echo "Current branch: $current_branch"
        
        protected_branches=("main" "develop")
        is_protected=false
        
        for branch in "${protected_branches[@]}"; do
            if [ "$current_branch" = "$branch" ]; then
                is_protected=true
                break
            fi
        done
        
        if [ "$is_protected" = true ]; then
            echo -e "${YELLOW}⚠️  Warning: You are on a protected branch ($current_branch)${NC}"
            echo "Work should be done in feature/bugfix/hotfix branches"
        else
            echo -e "${GREEN}✅ Branch name is valid for PR${NC}"
        fi
        echo ""
        TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    fi
fi

# 終了時刻を記録
END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

# サマリー表示
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}                    Summary${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo ""

if [ $FAILED_CHECKS -eq 0 ]; then
    echo -e "${GREEN}🎉 All checks passed!${NC}"
    echo ""
    echo -e "Total checks: ${CYAN}$TOTAL_CHECKS${NC}"
    echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
    echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
    echo -e "Duration: ${CYAN}${DURATION}s${NC}"
    echo ""
    echo -e "${GREEN}✅ You can safely commit your changes.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some checks failed${NC}"
    echo ""
    echo -e "Total checks: ${CYAN}$TOTAL_CHECKS${NC}"
    echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
    echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
    echo -e "Duration: ${CYAN}${DURATION}s${NC}"
    echo ""
    echo -e "${YELLOW}Please fix the errors above before committing.${NC}"
    exit 1
fi

