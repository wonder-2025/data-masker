#!/bin/bash

echo "================================"
echo "Data Masker E2E 测试开始"
echo "================================"
echo ""

# 设置颜色
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 进入项目目录
cd "$(dirname "$0")"

# 清理旧结果
echo -e "${YELLOW}清理旧的测试结果...${NC}"
rm -rf test-results/*
mkdir -p test-results/{logs,screenshots,videos,html-report}

# 检查依赖
echo -e "${YELLOW}检查测试依赖...${NC}"
if ! command -v npx &> /dev/null; then
    echo -e "${RED}错误: npx 未安装${NC}"
    exit 1
fi

if [ ! -d "node_modules/@playwright" ]; then
    echo -e "${YELLOW}安装 Playwright...${NC}"
    npm install -D @playwright/test
    npx playwright install chromium
fi

# 运行测试
echo ""
echo -e "${GREEN}开始运行 E2E 测试...${NC}"
echo ""

npx playwright test

# 检查测试结果
if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}测试完成！所有测试通过 ✅${NC}"
    echo -e "${GREEN}================================${NC}"
else
    echo ""
    echo -e "${RED}================================${NC}"
    echo -e "${RED}测试完成！部分测试失败 ❌${NC}"
    echo -e "${RED}================================${NC}"
fi

echo ""
echo "测试报告位置:"
echo "  - HTML报告: test-results/html-report/index.html"
echo "  - JSON结果: test-results/results.json"
echo "  - 错误日志: test-results/logs/test-report.json"
echo "  - 截图: test-results/screenshots/"
echo "  - 视频: test-results/videos/"
echo ""

# 如果有HTML报告，提示如何查看
if [ -f "test-results/html-report/index.html" ]; then
    echo "查看HTML报告: npx playwright show-report test-results/html-report"
fi
