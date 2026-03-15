# Data Masker E2E 测试指南

## 测试框架概述

本测试框架基于 Playwright 构建，提供完整的端到端自动化测试能力。

### 功能特性

- ✅ **详细的错误日志记录** - 每个测试步骤都有日志记录
- ✅ **测试报告生成** - HTML 和 JSON 双格式报告
- ✅ **截图和视频录制** - 失败时自动截图和录制视频
- ✅ **覆盖所有核心功能** - UI、文件处理、规则配置、设置等

---

## 快速开始

### 1. 安装依赖

```bash
# 安装项目依赖
npm install

# 安装 Playwright 测试依赖
npm install -D @playwright/test

# 安装浏览器
npx playwright install chromium
```

### 2. 运行测试

```bash
# 运行所有测试
npm run test:e2e

# 运行测试并显示 UI
npm run test:e2e:ui

# 调试模式
npm run test:e2e:debug

# 显示浏览器窗口运行
npm run test:e2e:headed

# 或使用脚本
./run-e2e-tests.sh
```

### 3. 查看测试报告

```bash
# 查看 HTML 报告
npm run test:e2e:report
```

---

## 测试文件结构

```
data-masker/
├── playwright.config.js          # Playwright 配置文件
├── run-e2e-tests.sh              # 测试运行脚本
├── tests/
│   ├── e2e/                      # E2E 测试目录
│   │   ├── utils/
│   │   │   ├── logger.js         # 日志记录器
│   │   │   └── helpers.js        # 测试辅助函数
│   │   ├── ui-basic.spec.js      # 基础 UI 测试
│   │   ├── file-processing.spec.js # 文件处理测试
│   │   ├── rule-config.spec.js   # 规则配置测试
│   │   ├── settings.spec.js      # 设置功能测试
│   │   ├── preview.spec.js       # 预览确认测试
│   │   ├── result.spec.js        # 处理结果测试
│   │   └── ip-mapping.spec.js    # IP 映射测试
│   └── fixtures/                 # 测试数据文件
├── test-results/                 # 测试结果输出
│   ├── logs/                     # 日志文件
│   ├── screenshots/              # 失败截图
│   ├── videos/                   # 失败视频
│   ├── html-report/              # HTML 报告
│   └── results.json              # JSON 结果
```

---

## 测试用例说明

### 1. 基础 UI 测试 (`ui-basic.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 首页显示正常 | 验证首页基本元素渲染 |
| 菜单导航测试 | 测试所有菜单项的导航功能 |
| 响应式布局测试 | 测试不同视口尺寸下的布局 |

### 2. 文件处理测试 (`file-processing.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 导航到文件选择页 | 测试文件选择页导航 |
| 文件上传区域显示 | 验证上传区域渲染 |
| 支持的文件类型显示 | 检查文件类型提示 |
| 批量文件选择 | 测试批量选择功能 |
| 文件列表显示 | 验证文件列表渲染 |

### 3. 规则配置测试 (`rule-config.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 规则配置页面加载 | 验证规则配置页渲染 |
| 规则分类显示 | 检查规则分类展示 |
| 规则开关交互 | 测试规则开关切换 |
| 规则搜索功能 | 测试规则搜索 |
| 规则详情展开 | 测试详情展开功能 |
| 自定义规则标签 | 测试自定义规则切换 |
| 添加自定义规则 | 测试添加规则功能 |
| 规则重置功能 | 测试重置功能 |
| 规则保存功能 | 检查保存按钮 |

### 4. 设置功能测试 (`settings.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 设置页面加载 | 验证设置页渲染 |
| 设置分类显示 | 检查设置分类展示 |
| 输出目录设置 | 测试输出目录配置 |
| 脱敏设置标签 | 测试脱敏设置切换 |
| 保存设置按钮 | 检查保存按钮 |
| 重置设置功能 | 检查重置按钮 |
| 语言设置 | 测试语言配置 |
| 主题设置 | 测试主题切换 |

### 5. 预览确认测试 (`preview.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 预览页面加载 | 验证预览页渲染 |
| 预览内容显示 | 检查预览内容 |
| 原始内容与脱敏内容对比 | 测试内容对比 |
| 确认处理按钮 | 检查确认按钮 |
| 返回修改功能 | 检查返回按钮 |

### 6. 处理结果测试 (`result.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| 结果页面加载 | 验证结果页渲染 |
| 处理结果统计 | 检查统计信息 |
| 处理成功提示 | 验证成功提示 |
| 导出结果功能 | 检查导出按钮 |
| 查看详情功能 | 检查详情按钮 |
| 开始新任务 | 检查新任务按钮 |

### 7. IP 映射测试 (`ip-mapping.spec.js`)

| 测试用例 | 描述 |
|---------|------|
| IP映射页面加载 | 验证IP映射页渲染 |
| IP映射表格显示 | 检查表格渲染 |
| 添加IP映射 | 测试添加功能 |
| 搜索IP映射 | 测试搜索功能 |
| 删除IP映射 | 检查删除按钮 |
| 导入IP映射 | 检查导入按钮 |
| 导出IP映射 | 检查导出按钮 |

---

## 测试报告

### HTML 报告

运行测试后，打开 `test-results/html-report/index.html` 查看可视化报告：

```bash
npx playwright show-report test-results/html-report
```

### JSON 结果

`test-results/results.json` 包含详细的测试结果数据。

### 错误日志

`test-results/logs/test-report.json` 包含所有错误的详细信息：

```json
{
  "timestamp": "2026-03-15T09:46:00.000Z",
  "totalTests": "All passed",
  "errors": [],
  "summary": {
    "totalErrors": 0,
    "errorTypes": {}
  }
}
```

### 截图和视频

- **截图**: `test-results/screenshots/` - 测试失败时自动截图
- **视频**: `test-results/videos/` - 测试失败时保留视频

---

## 配置说明

### playwright.config.js 主要配置

```javascript
{
  testDir: './tests/e2e',           // 测试目录
  fullyParallel: false,             // 串行执行
  retries: 2,                       // 失败重试次数
  workers: 1,                       // 单线程执行
  
  use: {
    baseURL: 'http://localhost:1420', // 应用地址
    trace: 'on-first-retry',         // 首次重试时记录 trace
    screenshot: 'only-on-failure',   // 失败时截图
    video: 'retain-on-failure',      // 失败时保留视频
    actionTimeout: 10000,            // 操作超时时间
    navigationTimeout: 30000,        // 导航超时时间
  }
}
```

---

## 调试技巧

### 1. 使用 UI 模式调试

```bash
npm run test:e2e:ui
```

### 2. 使用 Debug 模式

```bash
npm run test:e2e:debug
```

### 3. 只运行特定测试

```bash
# 运行特定文件
npx playwright test tests/e2e/ui-basic.spec.js

# 运行特定测试用例
npx playwright test -g "首页显示正常"
```

### 4. 查看测试日志

```bash
# 详细日志
npx playwright test --reporter=list

# 非常详细的日志
DEBUG=pw:api npx playwright test
```

---

## 持续集成

在 CI 环境中运行测试：

```yaml
# GitHub Actions 示例
- name: Run E2E tests
  run: npm run test:e2e
  env:
    CI: true
```

---

## 最佳实践

1. **使用 safeAction 包装操作** - 自动捕获错误并记录日志
2. **每个测试独立** - 不依赖其他测试的结果
3. **使用有意义的测试名称** - 便于定位问题
4. **检查元素存在性** - 使用 try-catch 或辅助函数
5. **适当的等待** - 使用 waitForSelector 而非固定超时

---

## 故障排查

### 问题: 测试超时

**解决方案**: 增加 `playwright.config.js` 中的超时时间

### 问题: 元素找不到

**解决方案**: 检查选择器是否正确，可能需要等待页面加载

### 问题: 测试不稳定

**解决方案**: 
- 检查是否有竞态条件
- 使用 `waitForLoadState('networkidle')`
- 增加重试次数

---

## 联系支持

如有问题，请查看：
- 项目 README: `README.md`
- 安全测试指南: `SECURITY_TEST_GUIDE.md`
- 测试指南: `TEST_GUIDE.md`
