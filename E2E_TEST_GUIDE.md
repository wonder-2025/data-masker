# Data Masker E2E 测试完整操作手册

## 📋 目录

1. [环境准备](#环境准备)
2. [目录结构说明](#目录结构说明)
3. [测试执行](#测试执行)
4. [故障排查](#故障排查)
5. [测试报告](#测试报告)

---

## 🔧 环境准备

### 前置要求

- **操作系统**: Windows 10/11
- **Node.js**: 18.x 或更高版本
- **WebView2 运行时**: Windows 11 已内置，Windows 10 需要安装
- **PowerShell**: 5.1 或更高版本

### 安装步骤

```powershell
# 步骤 1: 解压项目
tar -xzf data-masker-e2e-tauri.tar.gz
cd data-masker

# 步骤 2: 安装依赖
npm install

# 步骤 3: 安装 Playwright 浏览器
npx playwright install chromium

# 步骤 4: 创建截图目录
mkdir test-results\screenshots
```

---

## 📂 目录结构说明

### 标准目录结构

解压后的项目结构：

```
data-masker/
├── windows-build/              ⭐ Tauri 应用（需解压到这里）
│   └── data-masker.exe         # 主程序
│
├── src-tauri/                  # Tauri 源码
│   └── target/release/         # 开发构建输出（可选）
│
├── tests/e2e/                  # E2E 测试脚本
│   ├── utils/
│   │   ├── logger.js           # 日志记录
│   │   └── helpers.js          # 辅助函数
│   │
│   ├── tauri-desktop.spec.js   ⭐ Tauri 应用测试
│   ├── check-tauri-env.bat     ⭐ 环境检测脚本
│   │
│   ├── ui-basic.spec.js        # Web UI 测试
│   ├── file-processing.spec.js
│   ├── rule-config.spec.js
│   ├── settings.spec.js
│   ├── preview.spec.js
│   ├── result.spec.js
│   └── ip-mapping.spec.js
│
├── playwright.config.js        # Playwright 配置
├── package.json
└── E2E_TEST_GUIDE.md           # 本手册
```

### ⚠️ 关键文件位置

| 文件 | 位置 | 说明 |
|------|------|------|
| **data-masker.exe** | `windows-build/data-masker.exe` | Tauri 应用主程序 |
| **测试脚本** | `tests/e2e/tauri-desktop.spec.js` | Tauri 应用测试 |
| **环境检测** | `tests/e2e/check-tauri-env.bat` | 检测 .exe 是否存在 |

---

## 🚀 测试执行

### 第一步：环境检测

运行环境检测脚本，确认 Tauri 应用存在：

```powershell
# 方式 1: 使用批处理脚本
.\tests\e2e\check-tauri-env.bat

# 方式 2: 手动检查
dir windows-build\data-masker.exe
```

**预期输出**：
```
✅ 找到: windows-build\data-masker.exe
✅ 环境检测通过
```

**如果未找到**：
```powershell
# 解压 windows-build.zip 到项目根目录
Expand-Archive windows-build.zip -DestinationPath .

# 确认文件存在
dir windows-build\data-masker.exe
```

---

### 第二步：运行测试

#### 方案 A：仅测试 Tauri 应用（推荐）

```powershell
# 运行 Tauri 应用测试
npx playwright test tests/e2e/tauri-desktop.spec.js

# 或使用 npm 脚本
npm run test:e2e -- tests/e2e/tauri-desktop.spec.js
```

#### 方案 B：仅测试 Web UI

```powershell
# 终端 1: 启动前端开发服务器
npm run dev

# 终端 2: 运行 Web UI 测试
npm run test:e2e -- --project=web-ui
```

#### 方案 C：运行所有测试

```powershell
# 同时运行 Web UI 和 Tauri 测试
npm run test:e2e
```

---

### 第三步：查看测试结果

```powershell
# 打开 HTML 测试报告
npm run test:e2e:report

# 或直接打开文件
start test-results\html-report\index.html
```

---

## 🧪 测试用例说明

### Tauri 应用测试（tauri-desktop.spec.js）

| 测试用例 | 说明 | 预期结果 |
|---------|------|---------|
| 1. 应用启动验证 | 检查应用是否正常启动 | 窗口标题包含 "Data Masker" |
| 2. 主界面渲染 | 验证 Vue 应用挂载 | #app 容器存在 |
| 3. 菜单导航 | 测试菜单项点击 | 至少有 1 个菜单项 |
| 4. 文件选择页面 | 导航到文件选择页 | 上传区域存在 |
| 5. 规则配置页面 | 导航到规则配置页 | 规则列表存在 |
| 6. 设置页面 | 导航到设置页 | 设置表单存在 |
| 7. 窗口操作 | 测试窗口状态 | 窗口可见 |
| 8. 截图功能测试 | 对各页面截图 | 截图文件生成 |

---

## ⚠️ 故障排查

### 问题 1：未找到 data-masker.exe

**症状**：
```
❌ 未找到 Tauri 应用
```

**解决方案**：
```powershell
# 1. 检查当前目录
pwd

# 2. 检查文件是否存在
dir windows-build

# 3. 如果没有，解压 windows-build.zip
Expand-Archive windows-build.zip -DestinationPath .

# 4. 确认文件存在
dir windows-build\data-masker.exe
```

---

### 问题 2：应用启动超时

**症状**：
```
❌ 启动失败: Timeout exceeded
```

**解决方案**：
```powershell
# 1. 手动启动应用，检查是否正常
.\windows-build\data-masker.exe

# 2. 检查 WebView2 运行时
# Windows 11: 已内置
# Windows 10: 下载并安装
# https://developer.microsoft.com/en-us/microsoft-edge/webview2/

# 3. 检查防病毒软件是否拦截
```

---

### 问题 3：Playwright 无法连接应用

**原因**：Tauri 使用 WebView2，不是标准 Electron

**解决方案**：
```powershell
# 方案 A: 增加超时时间
$env:PLAYWRIGHT_TIMEOUT=60000
npm run test:e2e -- tests/e2e/tauri-desktop.spec.js

# 方案 B: 手动启动应用后测试
# 1. 双击运行 data-masker.exe
# 2. 等待应用完全启动（约 10 秒）
# 3. 运行测试
```

---

### 问题 4：测试跳过（test.skip）

**症状**：
```
⚠️  跳过 Tauri 应用测试（未找到 .exe 文件）
```

**解决方案**：
```powershell
# 检查环境变量
echo $env:EXE_PATH

# 手动设置路径
$env:EXE_PATH="C:\完整路径\data-masker.exe"
npm run test:e2e -- tests/e2e/tauri-desktop.spec.js
```

---

## 📊 测试报告

### 报告位置

```
test-results/
├── html-report/
│   └── index.html              # HTML 可视化报告
│
├── screenshots/
│   ├── tauri-main.png          # 主界面截图
│   ├── tauri-首页.png
│   ├── tauri-文件选择.png
│   ├── tauri-规则配置.png
│   └── tauri-设置.png
│
├── results.json                # JSON 格式结果
└── logs/
    └── test-report.json        # 详细日志
```

### 查看报告

```powershell
# 方式 1: 使用 Playwright 命令
npm run test:e2e:report

# 方式 2: 直接打开 HTML
start test-results\html-report\index.html

# 方式 3: 查看截图
explorer test-results\screenshots
```

---

## 🔍 完整测试流程

### 标准流程（推荐）

```powershell
# ========== 第一步：环境检测 ==========
.\tests\e2e\check-tauri-env.bat

# 如果输出 "✅ 环境检测通过"，继续下一步

# ========== 第二步：运行测试 ==========
npx playwright test tests/e2e/tauri-desktop.spec.js

# ========== 第三步：查看结果 ==========
npm run test:e2e:report

# ========== 第四步：检查截图 ==========
explorer test-results\screenshots
```

### 如果环境检测失败

```powershell
# 1. 解压 windows-build.zip
Expand-Archive windows-build.zip -DestinationPath .

# 2. 确认文件存在
dir windows-build\data-masker.exe

# 3. 重新运行环境检测
.\tests\e2e\check-tauri-env.bat

# 4. 如果仍然失败，手动设置路径
$env:EXE_PATH="$(pwd)\windows-build\data-masker.exe"

# 5. 运行测试
npx playwright test tests/e2e/tauri-desktop.spec.js
```

---

## 📝 命令速查表

| 命令 | 说明 |
|------|------|
| `.\tests\e2e\check-tauri-env.bat` | 检测 Tauri 应用环境 |
| `npm run test:e2e` | 运行所有测试 |
| `npm run test:e2e -- tests/e2e/tauri-desktop.spec.js` | 仅测试 Tauri 应用 |
| `npm run test:e2e -- --project=web-ui` | 仅测试 Web UI |
| `npm run test:e2e:headed` | 带浏览器界面测试 |
| `npm run test:e2e:report` | 查看测试报告 |

---

## 🎯 测试目标

### Web UI 测试（前端逻辑）
- ✅ 页面加载和渲染
- ✅ 菜单导航
- ✅ 表单交互
- ✅ UI 组件功能

### Tauri 应用测试（完整功能）
- ✅ 应用启动和窗口
- ✅ 原生文件对话框
- ✅ 文件读取和写入
- ✅ 脱敏处理功能
- ✅ 结果导出

---

## 📞 技术支持

如有问题，请检查：
1. 本手册"故障排查"章节
2. 环境检测脚本输出
3. 测试报告中的错误信息
4. `test-results/logs/test-report.json` 详细日志

---

**最后更新**: 2026-03-18
**版本**: v2.0
