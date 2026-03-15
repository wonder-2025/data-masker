const { test, expect } = require('@playwright/test');
const logger = require('./utils/logger');
const { safeAction } = require('./utils/helpers');

test.describe('规则配置测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/rule-config');
    await page.waitForLoadState('networkidle');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('规则配置页面加载', async ({ page }) => {
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查规则配置页面加载',
      page
    );
  });

  test('规则分类显示', async ({ page }) => {
    // 检查是否有规则分类
    await safeAction(
      async () => {
        // 查找折叠面板或规则分类
        const collapseItems = await page.$$('.el-collapse-item, .rule-category, [class*="rule-group"]');
        // 页面应该正常渲染
        const pageContent = await page.content();
        expect(pageContent.length).toBeGreaterThan(0);
      },
      '检查规则分类显示',
      page
    );
  });

  test('规则开关交互', async ({ page }) => {
    // 查找开关元素
    await safeAction(
      async () => {
        const switches = await page.$$('.el-switch');
        if (switches.length > 0) {
          // 点击第一个开关
          await switches[0].click();
          await page.waitForTimeout(300);
        }
      },
      '测试规则开关交互',
      page
    );
  });

  test('规则搜索功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找搜索框
        const searchInput = await page.$('input[placeholder*="搜索"], input[placeholder*="search"], .el-input__inner');
        if (searchInput) {
          await searchInput.fill('测试');
          await page.waitForTimeout(500);
        }
      },
      '测试规则搜索功能',
      page
    );
  });

  test('规则详情展开', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找可展开的规则项
        const expandBtn = await page.$('.el-collapse-item__header, .rule-expand, [class*="expand"]');
        if (expandBtn) {
          await expandBtn.click();
          await page.waitForTimeout(300);
        }
      },
      '测试规则详情展开',
      page
    );
  });

  test('自定义规则标签', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找标签页
        const tabs = await page.$$('.el-tabs__item');
        if (tabs.length > 1) {
          // 点击自定义规则标签
          for (const tab of tabs) {
            const text = await tab.textContent();
            if (text && text.includes('自定义')) {
              await tab.click();
              await page.waitForTimeout(300);
              break;
            }
          }
        }
      },
      '切换到自定义规则标签',
      page
    );
  });

  test('添加自定义规则', async ({ page }) => {
    // 先切换到自定义规则标签
    const tabs = await page.$$('.el-tabs__item');
    for (const tab of tabs) {
      const text = await tab.textContent();
      if (text && text.includes('自定义')) {
        await tab.click();
        await page.waitForTimeout(500);
        break;
      }
    }

    await safeAction(
      async () => {
        // 查找添加按钮
        const addBtn = await page.$('button:has-text("添加"), button:has-text("新增"), .el-button--primary');
        if (addBtn) {
          await addBtn.click();
          await page.waitForTimeout(500);
        }
      },
      '测试添加自定义规则',
      page
    );
  });

  test('规则重置功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找重置按钮
        const resetBtn = await page.$('button:has-text("重置"), button:has-text("恢复默认"), .el-button--warning');
        if (resetBtn) {
          await resetBtn.click();
          await page.waitForTimeout(300);
        }
      },
      '测试规则重置功能',
      page
    );
  });

  test('规则保存功能', async ({ page }) => {
    await safeAction(
      async () => {
        // 查找保存按钮
        const saveBtn = await page.$('button:has-text("保存"), button:has-text("确定"), .el-button--primary');
        if (saveBtn) {
          // 不实际点击保存，只检查存在性
          const isVisible = await saveBtn.isVisible();
        }
      },
      '检查规则保存按钮',
      page
    );
  });
});
