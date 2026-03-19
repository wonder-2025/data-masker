# Data Masker E2E 测试 - 快速开始

## ⚠️ 重要：先启动应用，再运行测试

**测试脚本只检测已运行的应用，不会自动启动**

---

## 🚀 两步完成测试

### 第一步：启动应用

#### 方式 A：开发模式（推荐）

```powershell
# 终端 1：
npm run tauri dev

# 等待应用窗口打开，保持运行
```

#### 方式 B：手动启动

```powershell
# 双击打开：
.\windows-build\data-masker.exe

# 等待应用启动
# ⚠️ 首次运行需要安装
```

---

### 第二步：运行测试

```powershell
# 终端 2：
npm run test:e2e:tauri

# 查看报告：
npm run test:e2e:report
```

---

## 📊 测试脚本行为

### ✅ 会做
- 检测已运行的应用
- 连接调试端口
- 执行测试用例

### ❌ 不会做
- 自动启动应用
- 重复安装
- 关闭应用

---

## 💡 推荐工作流程

```powershell
# 终端 1：启动应用（保持运行）
npm run tauri dev

# 终端 2：多次运行测试
npm run test:e2e:tauri
npm run test:e2e:tauri  # 可以重复运行
npm run test:e2e:tauri

# 应用保持运行，测试可以反复执行
```

---

## ⚠️ 常见问题

### 问题：未检测到应用

**解决**：
1. 确保应用窗口已打开
2. 使用开发模式：`npm run tauri dev`
3. 等待应用完全启动后再测试

---

## 📚 详细文档

- **TAURI_TEST_GUIDE.md** - 完整测试指南
- **E2E_TEST_GUIDE.md** - Web UI 测试指南

---

**核心要点**：先启动应用，再运行测试！测试脚本不会自动启动应用。
