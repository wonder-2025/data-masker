// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'

import App from './App.vue'
import router from './router'
import './styles/main.css'
import errorReporter from './utils/errorReporter'

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
  errorReporter.reportError({
    type: 'VueError',
    message: err.message || String(err),
    stack: err.stack || '',
    info: info || ''
  })
}

app.mount('#app')

// 初始化错误报告器
errorReporter.init()
