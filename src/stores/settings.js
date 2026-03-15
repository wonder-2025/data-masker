// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { secureStorage } from '@/utils/secureStorage'

// 默认设置
const defaultSettings = {
  general: {
    language: 'zh-CN',
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
    password: '',
    autoCleanTemp: true,
    cleanAfter: 30,
    encryptMapping: false
  },
  advanced: {
    maxFileSize: 500,
    concurrentFiles: 3,
    logLevel: 'info'
  },
  errorReport: {
    enabled: false,
    serverUrl: '',
    collectErrors: true,
    collectOperations: false,
    collectAnalytics: false
  }
}

/**
 * 设置状态管理 Store
 */
export const useSettingsStore = defineStore('settings', () => {
  // 设置数据
  const settingsData = ref(JSON.parse(JSON.stringify(defaultSettings)))
  
  /**
   * 加载设置
   */
  async function loadSettings() {
    try {
      const saved = localStorage.getItem('data-masker-settings')
      if (saved) {
        const parsed = JSON.parse(saved)
        // 深度合并，确保新增字段有默认值
        settingsData.value = deepMerge(defaultSettings, parsed)
        
        // 解密密码字段（如果存在）
        if (settingsData.value.security?.password) {
          const decryptedPassword = await secureStorage.getItem('data-masker-password', true)
          if (decryptedPassword) {
            settingsData.value.security.password = decryptedPassword
          } else {
            // 如果解密失败，清除密码
            settingsData.value.security.password = ''
          }
        }
      }
    } catch (e) {
      console.error('加载设置失败:', e)
    }
  }
  
  /**
   * 保存设置
   */
  async function saveSettings() {
    try {
      // 创建设置副本
      const settingsToSave = JSON.parse(JSON.stringify(settingsData.value))
      
      // 加密密码字段后单独存储
      if (settingsToSave.security?.password) {
        const encrypted = await secureStorage.setItem(
          'data-masker-password',
          settingsToSave.security.password,
          true
        )
        // 在保存到 localStorage 的设置中移除密码
        delete settingsToSave.security.password
      }
      
      localStorage.setItem('data-masker-settings', JSON.stringify(settingsToSave))
      ElMessage.success('设置已保存')
    } catch (e) {
      console.error('保存设置失败:', e)
      ElMessage.error('保存设置失败')
    }
  }
  
  /**
   * 重置为默认设置
   */
  function resetToDefault() {
    settingsData.value = JSON.parse(JSON.stringify(defaultSettings))
    localStorage.removeItem('data-masker-settings')
    secureStorage.removeItem('data-masker-password')
    ElMessage.success('设置已重置')
  }
  
  /**
   * 更新单个设置项
   */
  function updateSetting(category, key, value) {
    if (settingsData.value[category]) {
      settingsData.value[category][key] = value
    }
  }
  
  /**
   * 获取输出目录
   */
  function getOutputDir() {
    return settingsData.value.general.outputDir || ''
  }
  
  /**
   * 设置输出目录
   */
  function setOutputDir(dir) {
    settingsData.value.general.outputDir = dir
    saveSettings()
  }
  
  /**
   * 设置密码（带强度检查）
   */
  function setPassword(password) {
    const strengthCheck = secureStorage.checkPasswordStrength(password)
    if (!strengthCheck.isValid) {
      return {
        success: false,
        message: '密码强度不足',
        suggestions: strengthCheck.suggestions,
        level: strengthCheck.level
      }
    }
    
    settingsData.value.security.password = password
    saveSettings()
    
    return {
      success: true,
      message: '密码设置成功',
      level: strengthCheck.level
    }
  }
  
  /**
   * 验证密码强度
   */
  function validatePasswordStrength(password) {
    return secureStorage.checkPasswordStrength(password)
  }
  
  /**
   * 深度合并对象
   */
  function deepMerge(target, source) {
    const result = { ...target }
    for (const key in source) {
      if (source[key] && typeof source[key] === 'object' && !Array.isArray(source[key])) {
        result[key] = deepMerge(target[key] || {}, source[key])
      } else {
        result[key] = source[key]
      }
    }
    return result
  }
  
  // 防抖保存定时器
  let saveTimeout = null
  
  /**
   * 防抖保存设置（自动保存时使用）
   */
  function debouncedSave() {
    if (saveTimeout) {
      clearTimeout(saveTimeout)
    }
    saveTimeout = setTimeout(async () => {
      try {
        // 创建设置副本
        const settingsToSave = JSON.parse(JSON.stringify(settingsData.value))
        
        // 加密密码字段后单独存储
        if (settingsToSave.security?.password) {
          await secureStorage.setItem(
            'data-masker-password',
            settingsToSave.security.password,
            true
          )
          // 在保存到 localStorage 的设置中移除密码
          delete settingsToSave.security.password
        }
        
        localStorage.setItem('data-masker-settings', JSON.stringify(settingsToSave))
      } catch (e) {
        console.error('自动保存设置失败:', e)
      }
    }, 500) // 500ms 防抖延迟
  }
  
  // 监听设置变化，自动保存
  watch(settingsData, () => {
    debouncedSave()
  }, { deep: true })
  
  // 初始化时加载设置
  loadSettings()
  
  return {
    settingsData,
    loadSettings,
    saveSettings,
    resetToDefault,
    updateSetting,
    getOutputDir,
    setOutputDir,
    setPassword,
    validatePasswordStrength
  }
})
