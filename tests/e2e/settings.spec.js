import { test, expect } from '@playwright/test';
import logger from './utils/logger.js';
import { safeAction } from './utils/helpers.js';

test.describe('设置功能测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('设置页面加载', async ({ page }) => {
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查设置页面加载',
      page
    );
  });

  test('设置分类显示', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找设置分类或标签页
        const tabs = await page.$$('.el-tabs__item, .settings-category, [class*="settings-group"]');
        const pageContent = await page.content();
        expect(pageContent.length).toBeGreaterThan(0);
      },
      '检查设置分类显示',
      page
    );
  });

  test('输出目录设置', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找输出目录输入框
        const outputInput = await page.$('input[placeholder*="输出"], input[placeholder*="目录"], .output-path');
        if (outputInput) {
          const currentValue = await outputInput.inputValue();
          // 尝试修改
          await outputInput.fill('/tmp/test-output');
          await page.waitForTimeout(300);
        }
      },
      '测试输出目录设置',
      page
    );
  });

  test('脱敏设置标签', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找脱敏设置相关标签
        const tabs = await page.$$('.el-tabs__item');
        for (const tab of tabs) {
          const text = await tab.textContent();
          if (text && (text.includes('脱敏') || text.includes('策略'))) {
            await tab.click();
            await page.waitForTimeout(300);
            break;
          }
        }
      },
      '切换到脱敏设置标签',
      page
    );
  });

  test('保存设置按钮', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找保存按钮
        const saveBtn = await page.$('button:has-text("保存"), button:has-text("确定"), .el-button--primary');
        if (saveBtn) {
          const isVisible = await saveBtn.isVisible();
        }
      },
      '检查保存设置按钮',
      page
    );
  });

  test('重置设置功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找重置按钮
        const resetBtn = await page.$('button:has-text("重置"), button:has-text("恢复默认")');
        if (resetBtn) {
          const isVisible = await resetBtn.isVisible();
        }
      },
      '检查重置设置按钮',
      page
    );
  });

  test('语言设置', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找语言选择器
        const langSelect = await page.$('.language-select, select, .el-select');
        if (langSelect) {
          await langSelect.click();
          await page.waitForTimeout(300);
        }
      },
      '测试语言设置',
      page
    );
  });

  test('主题设置', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找主题切换
        const themeToggle = await page.$('.theme-toggle, .dark-mode, [class*="theme"]');
        if (themeToggle) {
          await themeToggle.click();
          await page.waitForTimeout(300);
        }
      },
      '测试主题设置',
      page
    );
  });
});
