const { test, expect } = require('@playwright/test');
const logger = require('./utils/logger');
const { safeAction, waitForElement } = require('./utils/helpers');

test.describe('基础UI测试', () => {
  
  test.beforeEach(async ({ page }) => {
    logger.startTest(test.info().title);
    await page.goto('/');
  });

  test.afterEach(async ({ page }) => {
    const success = test.info().status === 'passed';
    logger.endTest(success);
  });

  test('首页显示正常', async ({ page }) => {
    // 检查页面加载
    await safeAction(
      async () => {
        await page.waitForLoadState('networkidle');
      },
      '等待页面加载完成',
      page
    );

    // 检查标题
    await safeAction(
      async () => {
        const title = await page.title();
        expect(title).toBeTruthy();
      },
      '检查页面标题',
      page
    );

    // 检查主容器
    await safeAction(
      async () => {
        const app = await page.$('#app');
        expect(app).not.toBeNull();
      },
      '检查应用容器',
      page
    );
  });

  test('菜单导航测试', async ({ page }) => {
    const menuItems = [
      { text: '首页', path: '/' },
      { text: '文件选择', path: '/file-select' },
      { text: '规则配置', path: '/rule-config' },
      { text: '预览确认', path: '/preview' },
      { text: '处理结果', path: '/result' },
      { text: 'IP映射', path: '/ip-mapping' },
      { text: '设置', path: '/settings' }
    ];

    for (const item of menuItems) {
      await safeAction(
        async () => {
          // 尝试点击菜单项
          const menuItem = await page.$(`.el-menu-item:has-text("${item.text}")`);
          if (menuItem) {
            await menuItem.click();
            await page.waitForTimeout(500);
          }
        },
        `导航到${item.text}`,
        page
      );
    }
  });

  test('响应式布局测试', async ({ page }) => {
    // 测试不同视口尺寸
    const viewports = [
      { width: 1920, height: 1080, name: 'Desktop' },
      { width: 1366, height: 768, name: 'Laptop' },
      { width: 768, height: 1024, name: 'Tablet' }
    ];

    for (const viewport of viewports) {
      await safeAction(
        async () => {
          await page.setViewportSize({ width: viewport.width, height: viewport.height });
          await page.waitForTimeout(300);
          const app = await page.$('#app');
          expect(app).not.toBeNull();
        },
        `测试${viewport.name}视口 (${viewport.width}x${viewport.height})`,
        page
      );
    }
  });
});
