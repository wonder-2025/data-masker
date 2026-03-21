// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'

import App from './App.vue'
import router from './router'
import './styles/main.css'
import { logCollector } from './utils/logCollector'
import { useSettingsStore } from './stores/settings'

const app = createApp(App)

// 注册所有 Element Plus 图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

const pinia = createPinia()
app.use(pinia)
app.use(router)
app.use(ElementPlus, {
  locale: zhCn,
})

// 配置 Vue 全局错误处理器
app.config.errorHandler = (err, vm, info) => {
  console.error('Vue Error:', err)
  logCollector.error('VUE_ERROR', err.message || String(err), {
    stack: err.stack || '',
    info: info || '',
    component: vm?.$options?.name || 'unknown'
  })
}

app.mount('#app')

// 将 app 实例暴露给日志收集器（用于 Vue 错误捕获）
window.app = app

// 初始化日志收集器（从设置中读取配置）
const settingsStore = useSettingsStore()
logCollector.init({
  enabled: settingsStore.settingsData.errorReport?.enabled ?? false,
  serverUrl: settingsStore.settingsData.errorReport?.serverUrl || '',
  appName: 'data-masker',
  version: '1.0.0',
  collectErrors: settingsStore.settingsData.errorReport?.collectErrors ?? true,
  collectOperations: settingsStore.settingsData.errorReport?.collectOperations ?? true,
  collectAnalytics: settingsStore.settingsData.errorReport?.collectAnalytics ?? true
})

// 记录应用启动信息
logCollector.info('APP_INITIALIZED', {
  version: '1.0.0',
  platform: navigator.platform,
  language: navigator.language,
  viewport: `${window.innerWidth}x${window.innerHeight}`
})

// 路由变化时记录页面访问
router.afterEach((to) => {
  logCollector.pageView(to.name || to.path)
})
