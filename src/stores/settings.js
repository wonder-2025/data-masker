// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

/**
 * 设置状态管理 Store
 * 管理应用的各项配置设置
 */

// 默认设置
const defaultSettings = {
  // 通用设置
  general: {
    language: 'zh-CN',          // 语言
    theme: 'light',             // 主题 (light/dark)
    outputDir: '',              // 默认输出目录
    autoOpenOutput: true        // 处理完成后自动打开输出目录
  },
  
  // 脱敏设置
  masking: {
    defaultStrategy: 'partial_mask', // 默认脱敏策略
    keepStartDigits: 3,        // 默认保留前几位
    keepEndDigits: 4,          // 默认保留后几位
    maskChar: '*',             // 脱敏字符
    fakeDataLocale: 'zh-CN'    // 假数据语言区域
  },
  
  // 安全设置
  security: {
    passwordProtect: false,    // 密码保护
    password: '',              // 密码（加密存储）
    autoCleanTemp: true,       // 自动清理临时文件
    cleanAfter: 30,            // 多少分钟后清理
    encryptMapping: true       // 加密映射表
  },
  
  // 高级设置
  advanced: {
    logLevel: 'info',          // 日志级别 (debug/info/warn/error)
    maxFileSize: 100,          // 最大文件大小(MB)
    concurrentFiles: 3,        // 并发处理文件数
    enableOCR: false,          // 启用OCR
    enableNER: false           // 启用NER实体识别
  },

  // 错误日志提交
  errorReport: {
    enabled: true,             // 是否启用错误日志提交
    serverUrl: 'http://106.12.190.227:30050/api/error-log'  // 日志服务器地址
  }
}

export const useSettingsStore = defineStore('settings', () => {
  // 当前设置
  const settings = ref({ ...defaultSettings })
  
  // 设置是否已加载
  const isLoaded = ref(false)

  /**
   * 从本地存储加载设置
   */
  async function loadSettings() {
    try {
      const saved = localStorage.getItem('data-masker-settings')
      if (saved) {
        const parsed = JSON.parse(saved)
        // 深度合并，保留默认值
        settings.value = deepMerge({ ...defaultSettings }, parsed)
      }
      isLoaded.value = true
    } catch (error) {
      console.error('加载设置失败:', error)
      settings.value = { ...defaultSettings }
    }
  }

  /**
   * 保存设置到本地存储
   */
  function saveSettings() {
    try {
      localStorage.setItem('data-masker-settings', JSON.stringify(settings.value))
    } catch (error) {
      console.error('保存设置失败:', error)
    }
  }

  /**
   * 更新设置
   * @param {string} category - 设置分类
   * @param {string} key - 设置项
   * @param {any} value - 设置值
   */
  function updateSetting(category, key, value) {
    if (settings.value[category]) {
      settings.value[category][key] = value
      saveSettings()
    }
  }

  /**
   * 批量更新设置
   * @param {Object} newSettings - 新设置对象
   */
  function updateSettings(newSettings) {
    settings.value = deepMerge(settings.value, newSettings)
    saveSettings()
  }

  /**
   * 重置为默认设置
   */
  function resetToDefault() {
    settings.value = { ...defaultSettings }
    saveSettings()
  }

  /**
   * 重置指定分类的设置
   * @param {string} category - 设置分类
   */
  function resetCategory(category) {
    if (defaultSettings[category]) {
      settings.value[category] = { ...defaultSettings[category] }
      saveSettings()
    }
  }

  /**
   * 获取设置值
   * @param {string} category - 设置分类
   * @param {string} key - 设置项
   * @returns {any} 设置值
   */
  function getSetting(category, key) {
    return settings.value[category]?.[key]
  }

  /**
   * 深度合并对象
   * @param {Object} target - 目标对象
   * @param {Object} source - 源对象
   * @returns {Object} 合并后的对象
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

  // 监听设置变化，自动保存
  watch(settings, () => {
    if (isLoaded.value) {
      saveSettings()
    }
  }, { deep: true })

  // 初始化加载设置
  loadSettings()

  return {
    settings,
    isLoaded,
    loadSettings,
    saveSettings,
    updateSetting,
    updateSettings,
    resetToDefault,
    resetCategory,
    getSetting
  }
})
