// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { createRouter, createWebHistory } from 'vue-router'
import { useSettingsStore } from '@/stores/settings'

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

// 应用解锁状态
let isUnlocked = false

// 需要文件检查的路由
const FILE_REQUIRED_ROUTES = ['/processing', '/result']

// 路由守卫 - 密码保护和文件检查
router.beforeEach((to, from, next) => {
  // 更新页面标题
  document.title = `${to.meta.title} - Data Masker` || 'Data Masker'
  
  // 检查密码保护
  const settingsStore = useSettingsStore()
  const { passwordProtect, password } = settingsStore.settingsData.security
  
  // 如果启用了密码保护且未解锁
  if (passwordProtect && password && !isUnlocked) {
    // 显示密码输入对话框
    showPasswordDialog(password, () => {
      isUnlocked = true
      // 密码验证成功后继续路由检查
      proceedWithRouteCheck(to, next)
    })
    return
  }
  
  proceedWithRouteCheck(to, next)
})

// 路由检查（文件状态）
function proceedWithRouteCheck(to, next) {
  // 检查是否需要文件
  if (FILE_REQUIRED_ROUTES.includes(to.path)) {
    const hasFiles = sessionStorage.getItem('hasFiles') === 'true'
    if (!hasFiles) {
      // 未选择文件，重定向到文件选择页
      next({ name: 'FileSelect', replace: true })
      return
    }
  }
  next()
}

// 密码输入对话框
function showPasswordDialog(correctPassword, onSuccess) {
  // 创建对话框元素
  const dialog = document.createElement('div')
  dialog.innerHTML = `
    <div style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; 
                background: rgba(0,0,0,0.5); display: flex; 
                align-items: center; justify-content: center; z-index: 9999;">
      <div style="background: #fff; border-radius: 12px; padding: 24px; 
                  width: 320px; box-shadow: 0 4px 20px rgba(0,0,0,0.15);">
        <h3 style="margin: 0 0 16px; font-size: 18px; color: #303133; 
                   display: flex; align-items: center; gap: 8px;">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="#409EFF">
            <path d="M12 1C8.676 1 6 3.676 6 7v2H4v14h16V9h-2V7c0-3.324-2.676-6-6-6zm0 2c2.276 0 4 1.724 4 4v2H8V7c0-2.276 1.724-4 4-4zm0 10c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2z"/>
          </svg>
          请输入密码
        </h3>
        <input type="password" id="password-input" 
               style="width: 100%; padding: 10px 12px; border: 1px solid #dcdfe6; 
                      border-radius: 6px; font-size: 14px; outline: none;
                      transition: border-color 0.2s;"
               placeholder="请输入访问密码" autofocus>
        <div id="error-msg" style="color: #f56c6c; font-size: 12px; margin-top: 8px; display: none;">
          密码错误，请重试
        </div>
        <button id="submit-btn" 
                style="width: 100%; margin-top: 16px; padding: 10px; 
                       background: linear-gradient(135deg, #409EFF, #36D1DC);
                       color: #fff; border: none; border-radius: 6px; 
                       font-size: 14px; cursor: pointer;">
          解锁
        </button>
      </div>
    </div>
  `
  document.body.appendChild(dialog)
  
  const input = dialog.querySelector('#password-input')
  const errorMsg = dialog.querySelector('#error-msg')
  const submitBtn = dialog.querySelector('#submit-btn')
  
  // 验证密码
  function verify() {
    const enteredPassword = input.value
    if (enteredPassword === correctPassword) {
      document.body.removeChild(dialog)
      onSuccess()
    } else {
      errorMsg.style.display = 'block'
      input.value = ''
      input.focus()
      input.style.borderColor = '#f56c6c'
      setTimeout(() => {
        input.style.borderColor = '#dcdfe6'
      }, 1000)
    }
  }
  
  // 点击按钮
  submitBtn.addEventListener('click', verify)
  
  // 回车提交
  input.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
      verify()
    }
  })
  
  // 聚焦输入框
  setTimeout(() => input.focus(), 100)
}

// 重置解锁状态（用于退出登录或锁定应用）
export function lockApp() {
  isUnlocked = false
}

export default router
