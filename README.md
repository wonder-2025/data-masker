# Data Masker - 文件脱敏工具

<div align="center">

![Data Masker Logo](docs/logo.png)

**安全、高效的本地文件脱敏工具**

敏感数据不上传云端 | 格式无损保持 | 预览确认机制 | 自定义规则系统

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-1.5-blue.svg)](https://tauri.app/)
[![Vue 3](https://img.shields.io/badge/Vue-3.4-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.75-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 🎯 产品定位

Data Masker 是一款桌面端本地文件脱敏工具，专门解决市面同类工具的核心痛点：

### 解决的核心痛点

| 痛点 | 解决方案 |
|------|----------|
| ⚠️ **云端泄露风险** | 100% 本地处理，零网络请求 |
| 📄 **格式破坏问题** | XML层级操作，保持样式/公式/布局 |
| 👁️ **无预览确认** | 左右对照视图，高亮修改位置 |
| ⚙️ **规则固化** | 可视化规则编辑器，支持导入导出 |

---

## ✨ 核心特性

### 🔒 本地处理，零上传
- 所有文件处理 100% 在本地完成
- 代码中无任何网络请求（除主动检查更新）
- 设置页明确标注"本地处理，零上传"

### 📑 格式无损保持
- **Word**: XML 层级操作，不破坏样式/表格/修订
- **Excel**: 保持公式、图表、条件格式
- **PDF**: 文本层定位替换，不破坏布局

### 👀 预览确认机制
- 左右对照视图：原文 vs 脱敏后
- 敏感信息高亮标记
- 支持手动调整个别脱敏结果
- 必须用户确认后才执行导出

### 📝 自定义规则系统
- 可视化规则编辑器（非纯文本输入）
- 规则测试功能（输入文本测试匹配效果）
- 规则导入/导出（JSON格式）
- 预设规则模板库

---

## 📦 支持的敏感信息类型

| 类型 | 说明 | 示例 |
|------|------|------|
| 身份证号 | 支持15位和18位，带校验 | 110101199001011234 |
| 手机号 | 中国大陆手机号码 | 13812345678 |
| 银行卡号 | 16-19位，带Luhn校验 | 6222021234567890123 |
| 护照号 | 中国护照格式 | G12345678 |
| 港澳通行证 | 港澳居民来往内地通行证 | H1234567890 |
| 台湾通行证 | 台湾居民来往大陆通行证 | T123456789 |
| 统一社会信用代码 | 18位，带校验 | 91110108MA01WXXX |
| 邮箱 | 电子邮箱地址 | user@example.com |
| 车牌号 | 中国车牌号码 | 京A12345 |
| IPv4/IPv6 | 网络地址 | 192.168.1.1 |
| MAC地址 | 设备物理地址 | 00:1A:2B:3C:4D:5E |
| API密钥 | JSON配置中的敏感值 | "api_key": "xxx" |
| 金额 | 货币金额 | 1,234,567.00元 |
| 自定义 | 用户自定义正则规则 | - |

---

## 🛠️ 脱敏策略

```javascript
// 1. 完全隐藏
"13812345678" → "***********"

// 2. 部分掩码（可配置保留位数）
"13812345678" → "138****5678"     // 保留前3后4
"110101199001011234" → "110101********1234"  // 保留前6后4

// 3. 假数据替换（保持格式真实）
"张三" → "李四"
"user@example.com" → "x8k2m@example.com"

// 4. 可逆加密（AES-256-GCM）
"敏感信息" → "加密字符串"（可恢复）

// 5. 哈希脱敏（不可逆）
"敏感信息" → "a1b2c3d4"

// 6. 自定义替换
用户指定替换文本
```

---

## 📁 支持的文件格式

| 格式 | 处理方式 | 格式保持 |
|------|----------|----------|
| PDF | 文本层提取+定位替换 | 样式、布局、图片 |
| Word (.docx) | XML解析+DOM操作 | 样式、表格、修订 |
| Excel (.xlsx) | OpenXML处理 | 公式、图表、条件格式 |
| TXT/MD/JSON/XML | 纯文本处理 | 编码格式 |
| CSV | 结构化处理 | 分隔符、编码 |

---

## 🚀 快速开始

### 环境要求

- Node.js >= 18
- Rust >= 1.70
- 系统支持: Windows / macOS / Linux

### 开发运行

```bash
# 克隆仓库
git clone https://github.com/your-repo/data-masker.git
cd data-masker

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev
```

### 打包发布

```bash
# 构建生产版本
npm run tauri build

# 输出位置
# Windows: src-tauri/target/release/bundle/msi/
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/deb/
```

---

## 📖 使用指南

### 1. 选择文件
- 拖拽文件或点击上传
- 支持批量选择多个文件
- 显示文件预览和预估处理时间

### 2. 配置规则
- 开启/关闭内置规则
- 配置脱敏策略
- 添加自定义规则

### 3. 预览确认
- 左右对照查看脱敏效果
- 高亮显示敏感信息位置
- 手动调整个别结果

### 4. 导出结果
- 下载脱敏后文件
- 导出脱敏报告（PDF/Excel）
- 导出映射表（可逆脱敏时）

---

## 🔐 安全特性

### 本地处理保障
- 所有敏感数据在本地内存中处理
- 处理完成立即清除敏感数据
- 临时文件存储在系统临时目录

### 映射表加密
- 可逆脱敏的映射表使用 AES-256-GCM 加密
- 密钥由用户密码通过 PBKDF2 派生
- 支持用户指定存储位置

### 审计日志
- 记录每次脱敏操作
- 日志中敏感信息自动脱敏
- 支持导出审计报告

---

## 📂 项目结构

```
data-masker/
├── src/                    # 前端源码 (Vue3)
│   ├── views/              # 页面组件
│   ├── components/         # 公共组件
│   ├── stores/             # Pinia 状态管理
│   ├── utils/              # 工具函数
│   └── styles/             # 样式文件
├── src-tauri/              # 后端源码 (Rust)
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── services/       # 业务服务
│   │   ├── models/         # 数据模型
│   │   └── utils/          # 工具函数
│   └── Cargo.toml          # Rust 依赖配置
├── package.json            # Node.js 依赖配置
└── README.md               # 本文件
```

---

## 🏗️ 技术栈

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **Vite** - 下一代前端构建工具
- **Element Plus** - Vue 3 组件库
- **Pinia** - Vue 状态管理
- **ECharts** - 数据可视化

### 后端
- **Rust** - 系统编程语言
- **Tauri** - 构建桌面应用的框架
- **正则表达式** - 敏感信息匹配
- **加密库** - AES-256-GCM, PBKDF2

---

## 👥 设计团队

| 角色 | 负责人 |
|------|--------|
| 产品设计 | wonder-宏 |
| 架构设计/开发实现 | JARVIS AI Assistant |

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 许可证开源。

---

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📮 联系方式

- Issues: [GitHub Issues](https://github.com/your-repo/data-masker/issues)
- Discussions: [GitHub Discussions](https://github.com/your-repo/data-masker/discussions)

---

<div align="center">

**⚠️ 重要提示：所有数据处理 100% 在本地完成，敏感数据不会上传到任何服务器**

Made with ❤️ by wonder-宏 & JARVIS AI Assistant

</div>
