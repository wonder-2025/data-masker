// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { useSettingsStore } from '@/stores/settings'

/**
 * 错误日志处理器
 * 自动捕获应用错误并发送到服务器
 */

class ErrorReporter {
  constructor() {
    this.settingsStore = null
    this.isInitialized = false
  }

  /**
   * 初始化错误报告器
   */
  init() {
    if (this.isInitialized) return

    // 延迟获取 store，确保 Pinia 已初始化
    setTimeout(() => {
      this.settingsStore = useSettingsStore()
      this.setupErrorHandlers()
      this.isInitialized = true
    }, 100)
  }

  /**
   * 设置错误处理器
   */
  setupErrorHandlers() {
    // 捕获全局 JavaScript 错误
    window.addEventListener('error', (event) => {
      this.reportError({
        type: 'JavaScriptError',
        message: event.message,
        stack: event.error?.stack || '',
        filename: this.maskPath(event.filename),
        lineno: event.lineno,
        colno: event.colno
      })
    })

    // 捕获 Promise 未处理拒绝
    window.addEventListener('unhandledrejection', (event) => {
      this.reportError({
        type: 'UnhandledPromiseRejection',
        message: event.reason?.message || String(event.reason),
        stack: event.reason?.stack || ''
      })
    })

    // 捕获 Vue 错误（需要在 main.js 中配置）
    console.log('[ErrorReporter] 错误处理器已初始化')
  }

  /**
   * 报告错误
   * @param {Object} error - 错误信息
   */
  async reportError(error) {
    if (!this.settingsStore?.settings?.errorReport?.enabled) {
      return
    }

    try {
      const response = await fetch(this.settingsStore.settings.errorReport.serverUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          app_name: 'data-masker',
          version: '1.0.0',
          os: navigator.platform,
          error_type: error.type || 'UnknownError',
          error_message: this.maskSensitiveData(error.message || ''),
          stack_trace: this.maskSensitiveData(error.stack || ''),
          user_description: error.userDescription || '',
          timestamp: new Date().toISOString()
        })
      })

      if (response.ok) {
        console.log('[ErrorReporter] 错误日志已发送')
      }
    } catch (sendError) {
      // 发送失败时静默处理，避免循环
      console.error('[ErrorReporter] 发送错误日志失败:', sendError)
    }
  }

  /**
   * 脱敏路径
   * @param {string} path - 原始路径
   * @returns {string} 脱敏后的路径
   */
  maskPath(path) {
    if (!path) return path

    // 替换用户主目录
    path = path.replace(/\/Users\/[^/]+/g, '<USER_HOME>')
    path = path.replace(/\/home\/[^/]+/g, '<USER_HOME>')
    path = path.replace(/C:\\Users\\[^\\]+/g, '<USER_HOME>')

    // 替换具体文件名
    path = path.replace(/\/[^/]+\.(txt|pdf|doc|docx|xls|xlsx|ppt|pptx)/gi, '/<FILE>')

    return path
  }

  /**
   * 脱敏敏感数据
   * @param {string} text - 原始文本
   * @returns {string} 脱敏后的文本
   */
  maskSensitiveData(text) {
    if (!text) return text

    // 脱敏文件路径
    text = this.maskPath(text)

    // 脱敏手机号
    text = text.replace(/1[3-9]\d{9}/g, '1**********')

    // 脱敏身份证号
    text = text.replace(/\d{17}[\dXx]/g, '******************')

    // 脱敏邮箱
    text = text.replace(/[\w.-]+@[\w.-]+\.\w+/g, '***@***.***')

    return text
  }

  /**
   * 手动报告错误
   * @param {string} type - 错误类型
   * @param {string} message - 错误消息
   * @param {string} userDescription - 用户描述（可选）
   */
  manualReport(type, message, userDescription = '') {
    this.reportError({
      type,
      message,
      userDescription
    })
  }
}

// 创建单例
const errorReporter = new ErrorReporter()

export default errorReporter
