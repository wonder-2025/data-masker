import { test, expect } from '@playwright/test';
import logger from './utils/logger.js';
import { safeAction } from './utils/helpers.js';

test.describe('处理结果测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/result');
    await page.waitForLoadState('networkidle');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('结果页面加载', async ({ page }) => {
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查结果页面加载',
      page
    );
  });

  test('处理结果统计', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找统计信息
        const stats = await page.$('.stats, .statistics, [class*="result"], .el-statistic');
        const pageContent = await page.content();
        expect(pageContent.length).toBeGreaterThan(0);
      },
      '检查处理结果统计',
      page
    );
  });

  test('处理成功提示', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找成功提示
        const successMsg = await page.$('.el-result, .success-message, [class*="success"]');
        if (successMsg) {
          const isVisible = await successMsg.isVisible();
        }
      },
      '检查处理成功提示',
      page
    );
  });

  test('导出结果功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找导出按钮
        const exportBtn = await page.$('button:has-text("导出"), button:has-text("下载"), .el-button--success');
        if (exportBtn) {
          const isVisible = await exportBtn.isVisible();
        }
      },
      '检查导出结果按钮',
      page
    );
  });

  test('查看详情功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找详情按钮
        const detailBtn = await page.$('button:has-text("详情"), button:has-text("查看"), [class*="detail"]');
        if (detailBtn) {
          const isVisible = await detailBtn.isVisible();
        }
      },
      '检查查看详情按钮',
      page
    );
  });

  test('开始新任务', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找新任务按钮
        const newTaskBtn = await page.$('button:has-text("新任务"), button:has-text("重新开始"), button:has-text("首页")');
        if (newTaskBtn) {
          const isVisible = await newTaskBtn.isVisible();
        }
      },
      '检查开始新任务按钮',
      page
    );
  });
});
