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
    info: info || ''
  })
}

app.mount('#app')

// 初始化日志收集器（从设置中读取配置）
const settingsStore = useSettingsStore()
logCollector.init({
  enabled: settingsStore.settingsData.errorReport?.enabled ?? false,
  serverUrl: settingsStore.settingsData.errorReport?.serverUrl || '',
  appName: 'data-masker',
  version: '1.0.0'
})

// 路由变化时记录页面访问
router.afterEach((to) => {
  logCollector.pageView(to.name || to.path)
})

// 应用主题
function applyTheme(theme) {
  const html = document.documentElement
  
  if (theme === 'dark') {
    html.classList.add('dark')
    html.setAttribute('data-theme', 'dark')
  } else if (theme === 'light') {
    html.classList.remove('dark')
    html.setAttribute('data-theme', 'light')
  } else {
    // 跟随系统
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (isDark) {
      html.classList.add('dark')
      html.setAttribute('data-theme', 'dark')
    } else {
      html.classList.remove('dark')
      html.setAttribute('data-theme', 'light')
    }
  }
}

// 初始应用主题
applyTheme(settingsStore.settingsData.general.theme)

// 监听主题变化
let lastTheme = settingsStore.settingsData.general.theme
settingsStore.$subscribe((mutation, state) => {
  const newTheme = state.general.theme
  if (newTheme !== lastTheme) {
    lastTheme = newTheme
    applyTheme(newTheme)
    logCollector.operation('THEME_CHANGE', { theme: newTheme })
  }
})

// 监听系统主题变化
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  if (settingsStore.settingsData.general.theme === 'auto') {
    applyTheme('auto')
  }
})
