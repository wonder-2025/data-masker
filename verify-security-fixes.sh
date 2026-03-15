#!/bin/bash
# 安全加固验证脚本

echo "=== Data Masker 安全加固验证 ==="
echo ""

# 检查新增文件
echo "1. 检查新增安全工具文件..."
files=(
  "src/utils/secureStorage.js"
  "src/utils/regexValidator.js"
  "src/utils/errorHandler.js"
  "SECURITY_HARDENING_GUIDE.md"
  "SECURITY_TEST_GUIDE.md"
)

for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    size=$(wc -c < "$file")
    echo "  ✓ $file ($size bytes)"
  else
    echo "  ✗ $file 不存在"
  fi
done

echo ""
echo "2. 检查代码修改..."

# 检查密码加密
if grep -q "secureStorage" src/stores/settings.js; then
  echo "  ✓ settings.js 使用加密存储"
else
  echo "  ✗ settings.js 未使用加密存储"
fi

# 检查路径验证
if grep -q "validate_path\|validate_user_path" src-tauri/src/commands/file.rs; then
  echo "  ✓ file.rs 添加路径验证"
else
  echo "  ✗ file.rs 未添加路径验证"
fi

# 检查命令注入防护
if grep -q "escape_path_for_command" src-tauri/src/commands/file.rs; then
  echo "  ✓ file.rs 添加命令注入防护"
else
  echo "  ✗ file.rs 未添加命令注入防护"
fi

# 检查正则验证
if grep -q "regexValidator" src/views/RuleConfig.vue; then
  echo "  ✓ RuleConfig.vue 添加正则验证"
else
  echo "  ✗ RuleConfig.vue 未添加正则验证"
fi

# 检查日志级别配置
if grep -q "DATA_MASKER_LOG_LEVEL" src-tauri/src/lib.rs; then
  echo "  ✓ lib.rs 支持环境变量配置"
else
  echo "  ✗ lib.rs 未支持环境变量配置"
fi

echo ""
echo "3. 统计修复代码..."

# 统计新增文件大小
total_size=0
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    size=$(wc -c < "$file")
    total_size=$((total_size + size))
  fi
done

echo "  - 新增文件总大小: $total_size bytes"
echo "  - 新增文件数量: ${#files[@]}"

# 统计修改的函数数量
modified_functions=$(grep -r "validate_path\|validate_user_path\|escape_path_for_command\|setPassword\|validatePasswordStrength" src/ src-tauri/ 2>/dev/null | wc -l)
echo "  - 新增/修改函数: ~$modified_functions 处"

echo ""
echo "=== 验证完成 ==="
echo ""
echo "详细报告请查看："
echo "  - BUG_FIX_REPORT.md"
echo "  - SECURITY_FIX_SUMMARY.md"
echo "  - SECURITY_TEST_GUIDE.md"
