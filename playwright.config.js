import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: false,  // 串行执行，避免并发问题
  retries: 0,            // 禁用重试，避免重复运行
  workers: 1,            // 单线程执行
  
  reporter: [
    ['list'],  // 使用 list 格式，输出更清晰
    ['html', { outputFolder: 'test-results/html-report' }],
  ],

  use: {
    baseURL: 'http://localhost:1420',
    trace: 'off',  // 禁用 trace，减少开销
    screenshot: 'only-on-failure',
    video: 'off',  // 禁用视频录制
    actionTimeout: 10000,
    navigationTimeout: 30000,
  },

  projects: [
    {
      name: 'web-ui',
      use: { ...devices['Desktop Chrome'] },
      testMatch: /^(?!tauri-).*\.spec\.js$/,
    },
    {
      name: 'tauri-cdp',
      use: { ...devices['Desktop Chrome'] },
      testMatch: /tauri-cdp\.spec\.js/,
    },
  ],

  // Web 测试自动启动开发服务器
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:1420',
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },
});
