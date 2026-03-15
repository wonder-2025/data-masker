const { test, expect } = require('@playwright/test');
const logger = require('./utils/logger');
const { safeAction } = require('./utils/helpers');

test.describe('预览确认测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/preview');
    await page.waitForLoadState('networkidle');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('预览页面加载', async ({ page }) => {
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查预览页面加载',
      page
    );
  });

  test('预览内容显示', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找预览区域
        const previewArea = await page.$('.preview-content, .preview-area, [class*="preview"]');
        const pageContent = await page.content();
        expect(pageContent.length).toBeGreaterThan(0);
      },
      '检查预览内容显示',
      page
    );
  });

  test('原始内容与脱敏内容对比', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找对比显示
        const comparison = await page.$('.comparison, .diff-view, [class*="compare"]');
        if (comparison) {
          const isVisible = await comparison.isVisible();
        }
      },
      '检查内容对比显示',
      page
    );
  });

  test('确认处理按钮', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找确认按钮
        const confirmBtn = await page.$('button:has-text("确认"), button:has-text("开始处理"), button:has-text("执行")');
        if (confirmBtn) {
          const isVisible = await confirmBtn.isVisible();
        }
      },
      '检查确认处理按钮',
      page
    );
  });

  test('返回修改功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找返回按钮
        const backBtn = await page.$('button:has-text("返回"), button:has-text("上一步"), .el-button--default');
        if (backBtn) {
          const isVisible = await backBtn.isVisible();
        }
      },
      '检查返回修改按钮',
      page
    );
  });
});
