/**
 * 日志收集工具 v3.0
 * 收集应用的所有日志：错误、操作、调试、行为分析
 */

class LogCollector {
  constructor() {
    this.enabled = false
    this.serverUrl = ''
    this.appName = 'data-masker'
    this.version = '1.0.0'
    this.queue = []
    this.flushInterval = null
    this.maxQueueSize = 10
  }

  /**
   * 初始化日志收集器
   */
  init(options = {}) {
    this.enabled = options.enabled ?? false
    this.serverUrl = options.serverUrl || ''
    this.appName = options.appName || 'data-masker'
    this.version = options.version || '1.0.0'
    
    // 定期刷新队列
    if (this.enabled) {
      this.flushInterval = setInterval(() => this.flush(), 30000) // 30秒刷新一次
      this._setupErrorHandlers()
    }
  }

  /**
   * 更新配置
   */
  updateConfig(options = {}) {
    if (options.enabled !== undefined) this.enabled = options.enabled
    if (options.serverUrl !== undefined) this.serverUrl = options.serverUrl
  }

  /**
   * 设置错误处理器
   */
  _setupErrorHandlers() {
    // JS 错误
    window.addEventListener('error', (event) => {
      this.error('JS_ERROR', event.message, {
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
        stack: event.error?.stack
      })
    })

    // Promise 拒绝
    window.addEventListener('unhandledrejection', (event) => {
      this.error('PROMISE_REJECTION', event.reason?.message || String(event.reason), {
        stack: event.reason?.stack
      })
    })
  }

  /**
   * 发送错误日志
   */
  error(event, message, data = {}) {
    this._log('error', event, message, data)
  }

  /**
   * 发送操作日志
   */
  operation(event, detail = {}) {
    this._log('operation', event, detail.message || '', detail)
  }

  /**
   * 发送调试日志
   */
  debug(event, detail = {}) {
    this._log('debug', event, detail.message || '', detail)
  }

  /**
   * 发送行为分析日志
   */
  analytics(event, detail = {}) {
    this._log('analytics', event, detail.message || '', detail)
  }

  /**
   * 记录页面访问
   */
  pageView(pageName) {
    this.analytics('PAGE_VIEW', { page: pageName, url: window.location.href })
  }

  /**
   * 记录功能使用
   */
  featureUse(featureName, detail = {}) {
    this.analytics('FEATURE_USE', { feature: featureName, ...detail })
  }

  /**
   * 记录文件操作
   */
  fileOperation(action, fileInfo = {}) {
    this.operation('FILE_OPERATION', {
      action,
      fileType: fileInfo.type,
      fileSize: fileInfo.size,
      fileName: this._maskPath(fileInfo.name)
    })
  }

  /**
   * 记录处理耗时
   */
  timing(operation, durationMs, detail = {}) {
    this.debug('TIMING', { operation, duration: durationMs, ...detail })
  }

  /**
   * 内部日志方法
   */
  async _log(logType, event, message, data = {}) {
    if (!this.enabled || !this.serverUrl) {
      return
    }

    const logData = {
      log_type: logType,
      event,
      message: this._sanitize(message),
      version: this.version,
      os: navigator.platform,
      timestamp: new Date().toISOString(),
      ...this._sanitizeObject(data)
    }

    // 加入队列
    this.queue.push(logData)

    // 队列满了或者错误立即发送
    if (this.queue.length >= this.maxQueueSize || logType === 'error') {
      await this.flush()
    }
  }

  /**
   * 刷新队列，发送所有日志
   */
  async flush() {
    if (!this.enabled || !this.serverUrl || this.queue.length === 0) {
      return
    }

    const logs = [...this.queue]
    this.queue = []

    try {
      // 逐条发送
      for (const logData of logs) {
        await fetch(this.serverUrl, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            app_name: this.appName,
            ...logData
          })
        })
      }
    } catch (error) {
      console.error('发送日志失败:', error)
      // 发送失败，重新加入队列
      this.queue = [...logs, ...this.queue]
    }
  }

  /**
   * 脱敏路径
   */
  _maskPath(path) {
    if (!path) return path
    return path.replace(/\/Users\/[^/]+/g, '/Users/***')
              .replace(/\/home\/[^/]+/g, '/home/***')
              .replace(/C:\\Users\\[^\\]+/g, 'C:\\Users\\***')
  }

  /**
   * 脱敏手机号
   */
  _maskPhone(phone) {
    if (!phone) return phone
    return phone.replace(/(\d{3})\d{4}(\d{4})/g, '$1****$2')
  }

  /**
   * 脱敏身份证
   */
  _maskIdCard(idCard) {
    if (!idCard) return idCard
    return idCard.replace(/(\d{6})\d{8}(\d{4})/g, '$1********$2')
  }

  /**
   * 脱敏字符串
   */
  _sanitize(str) {
    if (typeof str !== 'string') return str
    return this._maskPhone(this._maskIdCard(this._maskPath(str)))
  }

  /**
   * 脱敏对象
   */
  _sanitizeObject(obj) {
    if (typeof obj !== 'object' || obj === null) return obj
    
    const sanitized = Array.isArray(obj) ? [] : {}
    for (const key in obj) {
      if (typeof obj[key] === 'string') {
        sanitized[key] = this._sanitize(obj[key])
      } else if (typeof obj[key] === 'object') {
        sanitized[key] = this._sanitizeObject(obj[key])
      } else {
        sanitized[key] = obj[key]
      }
    }
    return sanitized
  }

  /**
   * 销毁
   */
  destroy() {
    if (this.flushInterval) {
      clearInterval(this.flushInterval)
    }
    this.flush()
  }
}

// 导出单例
export const logCollector = new LogCollector()

// 导出类
export default LogCollector
