# Data Masker 安全加固指南

## 环境变量配置

### 日志级别配置

可以通过环境变量 `DATA_MASKER_LOG_LEVEL` 设置日志级别：

```bash
# 设置日志级别为 DEBUG
export DATA_MASKER_LOG_LEVEL=debug

# 设置日志级别为 TRACE（最详细）
export DATA_MASKER_LOG_LEVEL=trace

# 其他可选级别：error, warn, info（默认）, debug, trace
```

### Windows 设置环境变量

```powershell
# PowerShell
$env:DATA_MASKER_LOG_LEVEL="debug"

# 或在系统环境变量中设置
setx DATA_MASKER_LOG_LEVEL "debug"
```

### macOS/Linux 设置环境变量

```bash
# 临时设置（当前终端会话）
export DATA_MASKER_LOG_LEVEL=debug

# 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export DATA_MASKER_LOG_LEVEL=debug' >> ~/.bashrc
source ~/.bashrc
```

## 运行时配置

日志级别也可以在应用设置中配置（高级设置 -> 日志级别），该设置将覆盖环境变量。

## 生产环境建议

在生产环境中，建议使用以下配置：

```bash
# 生产环境推荐设置
export DATA_MASKER_LOG_LEVEL=warn
```

这样可以记录警告和错误信息，但不会产生过多的日志输出。

## 开发环境建议

在开发或调试时，可以使用更详细的日志级别：

```bash
# 开发环境推荐设置
export DATA_MASKER_LOG_LEVEL=debug

# 或最详细的日志（仅用于深度调试）
export DATA_MASKER_LOG_LEVEL=trace
```
