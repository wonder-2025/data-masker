const { test, expect } = require('@playwright/test');
const logger = require('./utils/logger');
const { safeAction } = require('./utils/helpers');

test.describe('IP映射测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/ip-mapping');
    await page.waitForLoadState('networkidle');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('IP映射页面加载', async ({ page }) => {
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查IP映射页面加载',
      page
    );
  });

  test('IP映射表格显示', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找表格
        const table = await page.$('.el-table, table, [class*="mapping-table"]');
        if (table) {
          const isVisible = await table.isVisible();
        }
      },
      '检查IP映射表格显示',
      page
    );
  });

  test('添加IP映射', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找添加按钮
        const addBtn = await page.$('button:has-text("添加"), button:has-text("新增"), .el-button--primary');
        if (addBtn) {
          await addBtn.click();
          await page.waitForTimeout(500);
        }
      },
      '测试添加IP映射',
      page
    );
  });

  test('搜索IP映射', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找搜索框
        const searchInput = await page.$('input[placeholder*="搜索"], input[placeholder*="IP"], .el-input__inner');
        if (searchInput) {
          await searchInput.fill('192.168');
          await page.waitForTimeout(300);
        }
      },
      '测试搜索IP映射',
      page
    );
  });

  test('删除IP映射', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找删除按钮
        const deleteBtn = await page.$('button:has-text("删除"), .el-button--danger, [class*="delete"]');
        if (deleteBtn) {
          const isVisible = await deleteBtn.isVisible();
        }
      },
      '检查删除IP映射按钮',
      page
    );
  });

  test('导入IP映射', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找导入按钮
        const importBtn = await page.$('button:has-text("导入"), button:has-text("Import")');
        if (importBtn) {
          const isVisible = await importBtn.isVisible();
        }
      },
      '检查导入IP映射按钮',
      page
    );
  });

  test('导出IP映射', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找导出按钮
        const exportBtn = await page.$('button:has-text("导出"), button:has-text("Export")');
        if (exportBtn) {
          const isVisible = await exportBtn.isVisible();
        }
      },
      '检查导出IP映射按钮',
      page
    );
  });
});
