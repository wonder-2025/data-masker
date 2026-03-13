// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/Home.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/file-select',
    name: 'FileSelect',
    component: () => import('@/views/FileSelect.vue'),
    meta: { title: '文件选择' }
  },
  {
    path: '/rule-config',
    name: 'RuleConfig',
    component: () => import('@/views/RuleConfig.vue'),
    meta: { title: '规则配置' }
  },
  {
    path: '/preview',
    name: 'Preview',
    component: () => import('@/views/Preview.vue'),
    meta: { title: '预览确认' }
  },
  {
    path: '/processing',
    name: 'Processing',
    component: () => import('@/views/Processing.vue'),
    meta: { title: '处理中' }
  },
  {
    path: '/result',
    name: 'Result',
    component: () => import('@/views/Result.vue'),
    meta: { title: '处理结果' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/Settings.vue'),
    meta: { title: '设置' }
  },
  {
    path: '/ip-mapping',
    name: 'IPMapping',
    component: () => import('@/views/IPMapping.vue'),
    meta: { title: 'IP映射' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫 - 更新页面标题
router.beforeEach((to, from, next) => {
  document.title = `${to.meta.title} - Data Masker` || 'Data Masker'
  next()
})

export default router
