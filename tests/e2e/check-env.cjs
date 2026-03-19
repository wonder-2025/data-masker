// 环境检测脚本（CommonJS 格式，避免 ES Module 问题）
const fs = require('fs');
const path = require('path');

console.log('\n========================================');
console.log('  Tauri 应用测试环境检测工具');
console.log('========================================\n');

// 项目根目录
const projectRoot = path.join(__dirname, '..');

// 可能的路径列表
const possiblePaths = [
  path.join(projectRoot, 'windows-build', 'data-masker.exe'),
  path.join(projectRoot, 'src-tauri', 'target', 'release', 'data-masker.exe'),
  path.join(projectRoot, '..', 'windows-build', 'data-masker.exe'),
  path.join(projectRoot, 'data-masker.exe'),
];

console.log('[1/3] 查找 data-masker.exe...\n');

let found = false;
let exePath = '';

for (const testPath of possiblePaths) {
  const relative = path.relative(projectRoot, testPath);
  if (fs.existsSync(testPath)) {
    console.log(`✅ 找到: ${relative}`);
    found = true;
    exePath = testPath;
  } else {
    console.log(`❌ 未找到: ${relative}`);
  }
}

console.log('');

if (found) {
  const stats = fs.statSync(exePath);
  const sizeMB = (stats.size / 1024 / 1024).toFixed(2);

  console.log('[2/3] 验证可执行文件...\n');
  console.log(`✅ 文件存在: ${path.relative(projectRoot, exePath)}`);
  console.log(`   文件大小: ${sizeMB} MB\n`);

  console.log('[3/3] 设置环境变量...\n');
  console.log(`set EXE_PATH=${exePath}\n`);

  console.log('========================================');
  console.log('  ✅ 环境检测通过');
  console.log('========================================\n');
  console.log('可执行文件路径:');
  console.log(exePath);
  console.log('\n现在可以运行测试:');
  console.log('  npm run test:e2e:tauri');
  console.log('');
} else {
  console.log('[2/3] 未找到可执行文件\n');
  console.log('========================================');
  console.log('  ⚠️  需要准备 Tauri 应用');
  console.log('========================================\n');
  console.log('解决方案:\n');
  console.log('方案 1: 解压 windows-build.zip');
  console.log('  1. 将 windows-build.zip 解压到项目根目录');
  console.log('  2. 确保 windows-build/data-masker.exe 存在');
  console.log('  3. 重新运行: npm run check:env\n');
  console.log('方案 2: 构建 Tauri 应用');
  console.log('  1. npm run tauri:build');
  console.log('  2. 构建产物在 src-tauri/target/release/');
  console.log('  3. 重新运行: npm run check:env\n');
  console.log('方案 3: 指定自定义路径');
  console.log('  set EXE_PATH=完整路径\\data-masker.exe');
  console.log('  npm run test:e2e:tauri\n');
}

console.log('');
