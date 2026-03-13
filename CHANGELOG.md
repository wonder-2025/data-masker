# Data Masker 更新日志

## [v1.0.0-tauri-v2] - 2026-03-12

### 重大变更
- 迁移 Tauri v1 → Tauri v2 API

### 修复
- **Cargo.toml**
  - 修复注释格式：`//` → `#`（TOML规范）
  - 新增 `tauri-plugin-dialog = "2"` - 文件对话框插件
  - 新增 `tauri-plugin-fs = "2"` - 文件系统插件
  - 新增 `tauri-plugin-shell = "2"` - Shell命令插件

- **tauri.conf.json**
  - 完全重写为 Tauri v2 格式
  - `devPath` → `devUrl`
  - `distDir` → `frontendDist`
  - `tauri.windows` → `app.windows`
  - `tauri.bundle` → `bundle`（移至顶层）
  - 移除已废弃的 `allowlist` 配置

- **src/lib.rs**
  - 添加插件初始化：`tauri_plugin_dialog::init()`
  - 添加插件初始化：`tauri_plugin_fs::init()`
  - 添加插件初始化：`tauri_plugin_shell::init()`

- **src/commands/file.rs**
  - `tauri::api::dialog::blocking::FileDialogBuilder` → `tauri_plugin_dialog::DialogExt`
  - `app.path_resolver().app_data_dir()` → `app.path().app_data_dir()`

- **src/commands/settings.rs**
  - `app.path_resolver()` → `app.path()`
  - `.ok_or()` → `.map_err()`（错误处理改进）

---

## 更新记录格式说明

每次更新按以下格式记录：

```markdown
## [版本号] - YYYY-MM-DD

### 新增
- 新功能描述

### 修复
- BUG修复描述

### 变更
- API变更描述

### 移除
- 废弃功能描述
```

---

*最后更新: 2026-03-12 17:22*
