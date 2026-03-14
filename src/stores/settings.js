import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  // 密码 hash（不存储明文密码）
  let passwordHash = null

  // 简单的密码 hash 函数
  async function hashPassword(password) {
    const encoder = new TextEncoder()
    const data = encoder.encode(password + 'data-masker-salt')
    const hashBuffer = await crypto.subtle.digest('SHA-256', data)
    const hashArray = Array.from(new Uint8Array(hashBuffer))
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
  }

  // 验证密码
  async function verifyPassword(password) {
    if (!passwordHash) return false
    const hash = await hashPassword(password)
    return hash === passwordHash
  }

  // 设置密码
  async function setPassword(password) {
    passwordHash = await hashPassword(password)
    localStorage.setItem('data-masker-password-hash', passwordHash)
  }

  // 检查是否设置了密码
  function hasPassword() {
    if (!passwordHash) {
      passwordHash = localStorage.getItem('data-masker-password-hash')
    }
    return !!passwordHash
  }

  // 完整的设置对象
  const settingsData = ref({
    // 通用设置
    general: {
      language: 'zh-CN',
      theme: 'light',
      outputDir: '',
      autoOpenOutput: true
    },
    // 脱敏设置
    masking: {
      defaultStrategy: 'partial_mask',
      keepStartDigits: 3,
      keepEndDigits: 4,
      maskChar: '*',
      fakeDataLocale: 'zh-CN'
    },
    // 安全设置
    security: {
      passwordProtect: false,
      // 密码不再明文存储，改为使用 hash
      autoCleanTemp: true,
      cleanAfter: 60,
      encryptMapping: true
    },
    // 高级设置
    advanced: {
      maxFileSize: 100,
      concurrentFiles: 3,
      logLevel: 'info',
      enableOCR: false,
      enableNER: false
    },
    // 错误报告（移除硬编码服务器地址）
    errorReport: {
      enabled: false,
      serverUrl: '', // 用户自定义服务器地址
      collectErrors: false,
      collectOperations: false,
      collectAnalytics: false
    }
  })

  // 计算属性
  const settings = computed(() => settingsData.value)

  // 更新设置
  function updateSetting(category, key, value) {
    if (settingsData.value[category]) {
      settingsData.value[category][key] = value
    }
  }

  // 批量更新设置
  function updateSettings(newSettings) {
    Object.assign(settingsData.value, newSettings)
  }

  // 重置为默认值
  function resetToDefault() {
    settingsData.value = {
      general: {
        language: 'zh-CN',
        theme: 'light',
        outputDir: '',
        autoOpenOutput: true
      },
      masking: {
        defaultStrategy: 'partial_mask',
        keepStartDigits: 3,
        keepEndDigits: 4,
        maskChar: '*',
        fakeDataLocale: 'zh-CN'
      },
      security: {
        passwordProtect: false,
        autoCleanTemp: true,
        cleanAfter: 60,
        encryptMapping: true
      },
      advanced: {
        maxFileSize: 100,
        concurrentFiles: 3,
        logLevel: 'info',
        enableOCR: false,
        enableNER: false
      },
      errorReport: {
        enabled: false,
        serverUrl: '',
        collectErrors: false,
        collectOperations: false,
        collectAnalytics: false
      }
    }
  }

  // 保存到本地存储
  function saveSettings() {
    localStorage.setItem('data-masker-settings', JSON.stringify(settingsData.value))
  }

  // 从本地存储加载
  function loadSettings() {
    const saved = localStorage.getItem('data-masker-settings')
    if (saved) {
      try {
        const parsed = JSON.parse(saved)
        Object.assign(settingsData.value, parsed)
      } catch (e) {
        console.error('加载设置失败:', e)
      }
    }
  }

  // 监听变化自动保存
  watch(settingsData, () => {
    localStorage.setItem('data-masker-settings', JSON.stringify(settingsData.value))
  }, { deep: true })

  // 初始化时加载
  loadSettings()

  return {
    settings,
    settingsData,
    updateSetting,
    updateSettings,
    resetToDefault,
    saveSettings,
    loadSettings,
    // 密码相关函数
    setPassword,
    verifyPassword,
    hasPassword
  }
})
