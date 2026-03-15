const logger = require('./logger');
const fs = require('fs');
const path = require('path');

/**
 * 安全执行操作，捕获所有错误
 */
async function safeAction(action, description, page) {
  try {
    logger.log(`执行: ${description}`);
    const result = await action();
    logger.log(`✓ 完成: ${description}`);
    return { success: true, result };
  } catch (error) {
    logger.logError(error, description);
    
    // 截图
    const screenshotDir = 'test-results/screenshots';
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }
    const screenshot = path.join(screenshotDir, `error-${Date.now()}.png`);
    try {
      await page.screenshot({ path: screenshot, fullPage: true });
      logger.log(`截图已保存: ${screenshot}`);
    } catch (e) {
      logger.log(`截图失败: ${e.message}`);
    }
    
    return { success: false, error };
  }
}

/**
 * 等待元素出现
 */
async function waitForElement(page, selector, timeout = 10000) {
  try {
    await page.waitForSelector(selector, { timeout });
    return true;
  } catch (error) {
    logger.logError(error, `等待元素: ${selector}`);
    return false;
  }
}

/**
 * 检查元素是否存在
 */
async function elementExists(page, selector) {
  try {
    const element = await page.$(selector);
    return element !== null;
  } catch {
    return false;
  }
}

/**
 * 等待页面加载完成
 */
async function waitForPageLoad(page, timeout = 30000) {
  try {
    await page.waitForLoadState('networkidle', { timeout });
    return true;
  } catch (error) {
    logger.logError(error, '等待页面加载');
    return false;
  }
}

/**
 * 获取元素文本
 */
async function getElementText(page, selector) {
  try {
    const element = await page.$(selector);
    if (element) {
      return await element.textContent();
    }
    return null;
  } catch (error) {
    logger.logError(error, `获取元素文本: ${selector}`);
    return null;
  }
}

/**
 * 安全点击元素
 */
async function safeClick(page, selector, description = '') {
  return safeAction(
    async () => {
      await page.click(selector);
    },
    description || `点击元素: ${selector}`,
    page
  );
}

/**
 * 安全输入文本
 */
async function safeFill(page, selector, value, description = '') {
  return safeAction(
    async () => {
      await page.fill(selector, value);
    },
    description || `输入文本到: ${selector}`,
    page
  );
}

module.exports = {
  safeAction,
  waitForElement,
  elementExists,
  waitForPageLoad,
  getElementText,
  safeClick,
  safeFill
};
