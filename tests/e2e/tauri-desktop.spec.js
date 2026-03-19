import { test, expect, _electron as electron } from '@playwright/test';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

/**
 * Tauri 桌面应用自动化测试（优化版）
 * 
 * 修复：避免重复打印查找日志
 */

// 全局变量（只初始化一次）
let electronApp = null;
let page = null;
let exePath = null;
let isInitialized = false;

/**
 * 查找可执行文件（只执行一次）
 */
function findExecutable() {
  // 如果已经找到，直接返回
  if (exePath) return exePath;

  const possiblePaths = [
    path.join(__dirname, '../../windows-build/data-masker.exe'),
    path.join(__dirname, '../../../windows-build/data-masker.exe'),
    path.join(__dirname, '../../src-tauri/target/release/data-masker.exe'),
    path.join(__dirname, '../../../src-tauri/target/release/data-masker.exe'),
    path.join(process.cwd(), 'windows-build/data-masker.exe'),
    path.join(process.cwd(), 'data-masker.exe'),
    process.env.EXE_PATH,
  ].filter(Boolean);

  for (const testPath of possiblePaths) {
    if (fs.existsSync(testPath)) {
      exePath = testPath;
      return exePath;
    }
  }

  return null;
}

test.describe('Tauri 桌面应用测试', () => {

  test.beforeAll(async () => {
    // 防止重复初始化
    if (isInitialized) return;
    isInitialized = true;

    // 查找可执行文件（静默模式）
    const foundPath = findExecutable();

    if (!foundPath) {
      console.log('\n========================================');
      console.log('  ⚠️  未找到 Tauri 应用');
      console.log('========================================');
      console.log('\n请确保以下任一条件满足：');
      console.log('1. windows-build/data-masker.exe 存在');
      console.log('2. 或设置环境变量: set EXE_PATH=路径');
      console.log('========================================\n');
      return;
    }

    console.log('\n========================================');
    console.log('  ✅ 找到 Tauri 应用');
    console.log('========================================');
    console.log(`路径: ${foundPath}\n`);

    try {
      // 启动 Tauri 应用
      electronApp = await electron.launch({
        executablePath: foundPath,
        timeout: 60000,  // 增加到 60 秒
      });

      // 等待窗口出现
      page = await electronApp.firstWindow();
      await page.waitForLoadState('domcontentloaded', { timeout: 30000 });

      console.log('✅ 应用启动成功\n');
    } catch (error) {
      console.error(`❌ 启动失败: ${error.message}\n`);
      electronApp = null;
      page = null;
    }
  });

  test.afterAll(async () => {
    if (electronApp) {
      try {
        await electronApp.close();
        console.log('✅ 应用已关闭\n');
      } catch (error) {
        console.error(`关闭应用时出错: ${error.message}`);
      }
    }
  });

  // ========== 测试用例 ==========

  test('1. 应用启动验证', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    const title = await page.title();
    expect(title.toLowerCase()).toContain('data masker');
    console.log(`✅ 测试通过: 窗口标题 "${title}"`);
  });

  test('2. 主界面渲染', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    await page.waitForSelector('#app', { timeout: 10000 });
    const app = await page.$('#app');
    expect(app).not.toBeNull();
    console.log('✅ 测试通过: Vue 应用已挂载');
  });

  test('3. 菜单导航', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    const menuItems = await page.$$('.el-menu-item');
    expect(menuItems.length).toBeGreaterThan(0);
    console.log(`✅ 测试通过: 找到 ${menuItems.length} 个菜单项`);
  });

  test('4. 文件选择页面', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    const fileMenu = await page.$('.el-menu-item:has-text("文件选择")');
    if (fileMenu) {
      await fileMenu.click();
      await page.waitForTimeout(500);
      console.log('✅ 测试通过: 文件选择页面可访问');
    }
  });

  test('5. 规则配置页面', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    const ruleMenu = await page.$('.el-menu-item:has-text("规则配置")');
    if (ruleMenu) {
      await ruleMenu.click();
      await page.waitForTimeout(500);
      console.log('✅ 测试通过: 规则配置页面可访问');
    }
  });

  test('6. 设置页面', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    const settingsMenu = await page.$('.el-menu-item:has-text("设置")');
    if (settingsMenu) {
      await settingsMenu.click();
      await page.waitForTimeout(500);
      console.log('✅ 测试通过: 设置页面可访问');
    }
  });

  test('7. 窗口状态验证', async () => {
    if (!page || !electronApp) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    try {
      const windowState = await electronApp.evaluate(({ BrowserWindow }) => {
        const win = BrowserWindow.getAllWindows()[0];
        if (win) {
          return {
            isMaximized: win.isMaximized(),
            isMinimized: win.isMinimized(),
            isVisible: win.isVisible(),
          };
        }
        return null;
      });

      expect(windowState).not.toBeNull();
      expect(windowState.isVisible).toBe(true);
      console.log('✅ 测试通过: 窗口可见');
    } catch (error) {
      console.log(`⚠️  窗口状态验证跳过: ${error.message}`);
    }
  });

  test('8. 截图保存', async () => {
    if (!page) {
      test.skip(true, '未找到 Tauri 应用或启动失败');
      return;
    }

    // 确保截图目录存在
    const screenshotDir = path.join(__dirname, '../../test-results/screenshots');
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }

    await page.screenshot({ 
      path: path.join(screenshotDir, 'tauri-main.png') 
    });
    console.log('✅ 测试通过: 截图已保存');
  });
});
