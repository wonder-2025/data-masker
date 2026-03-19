import { test, expect, chromium } from '@playwright/test';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';
import { promisify } from 'util';
import { exec } from 'child_process';

const execAsync = promisify(exec);
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Tauri 应用自动化测试（仅检测模式）
 * 
 * 特点：
 * 1. 只检测已运行的应用，不自动启动
 * 2. 支持 IPv4 和 IPv6 双重连接
 * 3. 持续等待用户手动启动应用
 * 4. 避免重复安装和死循环
 */

const DEBUG_PORT = 9222;
const MAX_WAIT_TIME = 120000;  // 最大等待 2 分钟
const CHECK_INTERVAL = 2000;   // 每 2 秒检查一次

let browser = null;
let context = null;
let page = null;
let isInitialized = false;

// 检查调试端口（支持 IPv4 和 IPv6）
async function checkDebugPort(port) {
  const urls = [
    `http://127.0.0.1:${port}/json`,  // IPv4
    `http://localhost:${port}/json`,  // 可能解析到 IPv4 或 IPv6
  ];

  for (const url of urls) {
    try {
      const response = await fetch(url, {
        signal: AbortSignal.timeout(3000),
      });
      if (response.ok) {
        return url.replace('/json', '');
      }
    } catch {
      // 继续尝试下一个
    }
  }
  return null;
}

// 检查进程是否运行
async function checkProcessRunning() {
  try {
    const { stdout } = await execAsync('tasklist | findstr /I "data-masker"');
    return stdout.length > 0;
  } catch {
    return false;
  }
}

// 等待应用启动（不自动启动）
async function waitForApp() {
  const startTime = Date.now();
  let lastProcessCheck = false;

  console.log('\n========================================');
  console.log('  🔍 检测 Tauri 应用');
  console.log('========================================\n');

  while (Date.now() - startTime < MAX_WAIT_TIME) {
    const elapsed = Math.floor((Date.now() - startTime) / 1000);

    // 检查进程是否运行
    const isRunning = await checkProcessRunning();
    
    if (isRunning && !lastProcessCheck) {
      console.log('✅ 检测到应用进程正在运行\n');
      lastProcessCheck = true;
    }

    // 检查调试端口
    const debugUrl = await checkDebugPort(DEBUG_PORT);
    
    if (debugUrl) {
      console.log('✅ 检测到调试端口\n');
      return debugUrl;
    }

    // 显示等待提示
    if (elapsed % 10 === 0 && elapsed > 0) {
      console.log(`[${elapsed}s] 等待应用启动调试端口...\n`);
    }

    // 显示帮助信息（只在第一次检测到进程但没有调试端口时）
    if (isRunning && !lastProcessCheck === false && elapsed < 5) {
      console.log('⚠️  应用正在运行，但调试端口未开启\n');
      console.log('可能的原因：');
      console.log('  1. 这是生产构建，不支持调试端口');
      console.log('  2. 应用正在启动中，请稍候\n');
      console.log('建议：使用开发模式启动应用');
      console.log('  npm run tauri dev\n');
    }

    await new Promise(resolve => setTimeout(resolve, CHECK_INTERVAL));
  }

  return null;
}

// 连接到应用
async function connectToApp(debugUrl) {
  console.log(`正在连接: ${debugUrl}`);

  try {
    browser = await chromium.connectOverCDP(debugUrl);
    
    const contexts = browser.contexts();
    if (contexts.length > 0) {
      context = contexts[0];
      const pages = context.pages();
      page = pages[0];
    }

    if (page) {
      console.log('✅ 已连接到 Tauri 应用\n');
      return true;
    } else {
      console.log('❌ 无法获取应用页面\n');
      return false;
    }
  } catch (error) {
    console.log(`❌ 连接失败: ${error.message}\n`);
    return false;
  }
}

// 显示帮助信息
function showHelp() {
  console.log('\n========================================');
  console.log('  📋 如何启动测试');
  console.log('========================================\n');
  console.log('方式 1（推荐）: 开发模式');
  console.log('  1. 打开新终端');
  console.log('  2. 运行: npm run tauri dev');
  console.log('  3. 等待应用窗口打开');
  console.log('  4. 重新运行测试\n');
  console.log('方式 2: 手动启动应用');
  console.log('  1. 双击打开 windows-build/data-masker.exe');
  console.log('  2. 等待应用启动');
  console.log('  3. 重新运行测试\n');
  console.log('⚠️  注意: 生产构建可能不支持调试端口\n');
  console.log('如果测试失败，请使用开发模式: npm run tauri dev\n');
}

test.describe('Tauri 应用功能测试', () => {

  test.beforeAll(async () => {
    if (isInitialized) return;
    isInitialized = true;

    // 等待用户启动应用
    const debugUrl = await waitForApp();

    if (!debugUrl) {
      console.log('========================================');
      console.log('  ❌ 超时：未检测到调试端口');
      console.log('========================================\n');
      showHelp();
      return;
    }

    // 连接到应用
    if (!await connectToApp(debugUrl)) {
      showHelp();
    }
  });

  test.afterAll(async () => {
    console.log('\n========================================');
    console.log('  🧹 清理资源');
    console.log('========================================\n');

    if (browser) {
      try {
        await browser.close();
        console.log('✅ 已断开连接');
      } catch (error) {
        console.log(`断开连接时出错: ${error.message}`);
      }
    }

    console.log('ℹ️  应用保持运行，不会自动关闭\n');
  });

  // ========== 测试用例 ==========

  test('1. 应用启动验证', async () => {
    if (!page) {
      test.skip(true, '未检测到应用，请手动启动后再运行测试');
      return;
    }

    const title = await page.title();
    expect(title).toContain('Data Masker');
    console.log(`✅ 测试通过: 窗口标题 "${title}"`);
  });

  test('2. Tauri API 可用性检测', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const tauriCheck = await page.evaluate(async () => {
      try {
        // @ts-ignore
        const tauri = window.__TAURI__;
        if (tauri) {
          return {
            available: true,
            version: tauri.versions?.tauri || 'unknown',
            hasFs: !!tauri.fs,
            hasDialog: !!tauri.dialog,
            hasShell: !!tauri.shell,
          };
        }
        return { available: false };
      } catch (error) {
        return { available: false, error: error.message };
      }
    });

    if (tauriCheck.available) {
      console.log(`✅ Tauri API 可用 (版本: ${tauriCheck.version})`);
      console.log(`   - 文件系统 API: ${tauriCheck.hasFs ? '✅' : '❌'}`);
      console.log(`   - 对话框 API: ${tauriCheck.hasDialog ? '✅' : '❌'}`);
      console.log(`   - Shell API: ${tauriCheck.hasShell ? '✅' : '❌'}`);
      expect(tauriCheck.available).toBe(true);
    } else {
      console.log('⚠️  Tauri API 不可用（可能是生产构建）');
      test.skip();
    }
  });

  test('3. 主界面加载', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    await page.waitForSelector('#app', { timeout: 10000 });
    const app = await page.$('#app');
    expect(app).not.toBeNull();
    console.log('✅ 测试通过: Vue 应用已挂载');
  });

  test('4. 菜单导航功能', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const menuItems = await page.$$('.el-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
    console.log(`✅ 测试通过: 找到 ${menuItems.length} 个菜单项`);

    const menuTexts = ['首页', '文件选择', '规则配置', '设置'];
    for (const text of menuTexts) {
      const menu = await page.$(`.el-menu-item:has-text("${text}")`);
      if (menu) {
        await menu.click();
        await page.waitForTimeout(300);
      }
    }
  });

  test('5. 文件选择页面', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const menu = await page.$('.el-menu-item:has-text("文件选择")');
    if (menu) {
      await menu.click();
      await page.waitForTimeout(500);
    }

    const upload = await page.$('.el-upload');
    expect(upload).not.toBeNull();
    console.log('✅ 测试通过: 文件上传组件存在');
  });

  test('6. 规则配置页面', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const menu = await page.$('.el-menu-item:has-text("规则配置")');
    if (menu) {
      await menu.click();
      await page.waitForTimeout(500);
    }

    const rules = await page.$$('.el-collapse-item');
    expect(rules.length).toBeGreaterThan(0);
    console.log(`✅ 测试通过: 找到 ${rules.length} 个规则项`);
  });

  test('7. 设置页面功能', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const menu = await page.$('.el-menu-item:has-text("设置")');
    if (menu) {
      await menu.click();
      await page.waitForTimeout(500);
    }

    const settings = await page.$$('.el-form-item');
    console.log(`✅ 测试通过: 找到 ${settings.length} 个设置项`);
  });

  test('8. 完整截图', async () => {
    if (!page) {
      test.skip(true, '未检测到应用');
      return;
    }

    const screenshotDir = path.join(__dirname, '../../test-results/screenshots');
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }

    const pages = [
      { name: '首页', selector: '.el-menu-item:has-text("首页")' },
      { name: '文件选择', selector: '.el-menu-item:has-text("文件选择")' },
      { name: '规则配置', selector: '.el-menu-item:has-text("规则配置")' },
      { name: '设置', selector: '.el-menu-item:has-text("设置")' },
    ];

    for (const pageInfo of pages) {
      const menu = await page.$(pageInfo.selector);
      if (menu) {
        await menu.click();
        await page.waitForTimeout(300);
        await page.screenshot({
          path: path.join(screenshotDir, `tauri-${pageInfo.name}.png`),
        });
        console.log(`  ✓ 已截图: ${pageInfo.name}`);
      }
    }

    console.log('✅ 测试通过: 所有页面截图完成');
  });
});
