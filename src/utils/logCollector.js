/**
 * 日志收集工具 v4.0 - 安全加固版
 * 收集应用的所有日志：错误、操作、调试、行为分析
 * 
 * 安全特性:
 * - 默认关闭日志收集
 * - 用户需显式同意后才启用
 * - 完善的敏感信息脱敏
 */

class LogCollector {
  constructor() {
    this.enabled = false  // 默认关闭
    this.serverUrl = ''
    this.appName = 'data-masker'
    this.version = '1.0.0'
    this.queue = []
    this.flushInterval = null
    this.maxQueueSize = 10
    this.userConsent = false  // 用户同意标志
  }

  /**
   * 初始化日志收集器
   * 注意: 默认不启用，需要用户显式同意
   */
  init(options = {}) {
    console.log('[LogCollector] init called with options:', options)
    
    // 检查用户是否已同意
    const savedConsent = localStorage.getItem('log-collector-consent')
    this.userConsent = savedConsent === 'true'
    
    // 只有在用户同意且明确启用时才激活
    this.enabled = this.userConsent && (options.enabled ?? false)
    this.serverUrl = options.serverUrl || ''
    this.appName = options.appName || 'data-masker'
    this.version = options.version || '1.0.0'
    
    console.log('[LogCollector] 初始化完成:', { 
      enabled: this.enabled, 
      userConsent: this.userConsent,
      serverUrl: this.serverUrl 
    })
    
    // 定期刷新队列
    if (this.enabled && this.userConsent) {
      this.flushInterval = setInterval(() => this.flush(), 30000) // 30秒刷新一次
      this._setupErrorHandlers()
    }
  }

  /**
   * 请求用户同意
   */
  requestConsent() {
    return new Promise((resolve) => {
      // 这里应该显示一个用户确认对话框
      // 为简化实现,我们返回一个标识,由前端处理
      console.log('[LogCollector] 需要用户同意才能启用日志收集')
      resolve(false)
    })
  }

  /**
   * 设置用户同意
   */
  setConsent(consent) {
    this.userConsent = consent
    localStorage.setItem('log-collector-consent', String(consent))
    
    if (!consent) {
      // 用户拒绝,禁用日志收集
      this.enabled = false
      if (this.flushInterval) {
        clearInterval(this.flushInterval)
        this.flushInterval = null
      }
    }
    
    console.log('[LogCollector] 用户同意状态更新:', consent)
  }

  /**
   * 更新配置
   */
  updateConfig(options = {}) {
    console.log('[LogCollector] updateConfig called with options:', options)
    
    // 检查用户同意
    if (!this.userConsent && options.enabled) {
      console.warn('[LogCollector] 用户未同意,无法启用日志收集')
      return
    }
    
    if (options.enabled !== undefined) {
      this.enabled = options.enabled && this.userConsent
      if (this.enabled && !this.flushInterval) {
        this.flushInterval = setInterval(() => this.flush(), 30000)
        this._setupErrorHandlers()
      }
    }
    if (options.serverUrl !== undefined) this.serverUrl = options.serverUrl
    console.log('[LogCollector] 配置更新完成:', { enabled: this.enabled, serverUrl: this.serverUrl })
  }

  /**
   * 设置错误处理器
   */
  _setupErrorHandlers() {
    // JS 错误
    window.addEventListener('error', (event) => {
      console.log('[LogCollector] 捕获JS错误:', event.message)
      this.error('JS_ERROR', event.message, {
        filename: event.filename,
        lineno: event.lineno,
        colno: event.colno,
        stack: event.error?.stack
      })
    })

    // Promise 拒绝
    window.addEventListener('unhandledrejection', (event) => {
      console.log('[LogCollector] 捕获Promise拒绝:', event.reason)
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
    console.log('[LogCollector] _log called:', { enabled: this.enabled, serverUrl: this.serverUrl, logType, event })
    
    if (!this.enabled) {
      console.log('[LogCollector] 日志收集已禁用，跳过')
      return
    }
    
    if (!this.serverUrl) {
      console.log('[LogCollector] 服务器地址为空，跳过')
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
    console.log('[LogCollector] 日志加入队列，当前队列长度:', this.queue.length)

    // 队列满了或者错误立即发送
    if (this.queue.length >= this.maxQueueSize || logType === 'error') {
      await this.flush()
    }
  }

  /**
   * 刷新队列，发送所有日志
   */
  async flush() {
    console.log('[LogCollector] flush called, enabled:', this.enabled, 'serverUrl:', this.serverUrl, 'queue length:', this.queue.length)
    
    if (!this.enabled) {
      console.log('[LogCollector] flush: 日志收集已禁用')
      return
    }
    
    if (!this.serverUrl) {
      console.log('[LogCollector] flush: 服务器地址为空')
      return
    }
    
    if (this.queue.length === 0) {
      console.log('[LogCollector] flush: 队列为空')
      return
    }

    const logs = [...this.queue]
    this.queue = []

    try {
      console.log('[LogCollector] 开始发送日志，数量:', logs.length)
      // 逐条发送
      for (const logData of logs) {
        console.log('[LogCollector] 发送日志:', logData.event)
        const response = await fetch(this.serverUrl, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            app_name: this.appName,
            ...logData
          })
        })
        console.log('[LogCollector] 日志发送响应:', response.status, response.ok)
      }
      console.log('[LogCollector] 所有日志发送完成')
    } catch (error) {
      console.error('[LogCollector] 发送日志失败:', error)
      // 发送失败，重新加入队列
      this.queue = [...logs, ...this.queue]
    }
  }

  /**
   * 脱敏路径
   */
  _maskPath(path) {
    if (!path) return path
    return path
      .replace(/\/Users\/[^/]+/g, '/Users/***')
      .replace(/\/home\/[^/]+/g, '/home/***')
      .replace(/C:\\Users\\[^\\]+/g, 'C:\\Users\\***')
      .replace(/\/root\/[^/]*/g, '/root/***')
      .replace(/\/var\/www\/[^/]*/g, '/var/www/***')
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
   * 脱敏邮箱
   */
  _maskEmail(email) {
    if (!email) return email
    return email.replace(/(.{2}).*(@.*)/g, '$1***$2')
  }

  /**
   * 脱敏银行卡号
   */
  _maskBankCard(card) {
    if (!card) return card
    return card.replace(/(\d{4})\d+(\d{4})/g, '$1****$2')
  }

  /**
   * 脱敏IP地址
   */
  _maskIP(ip) {
    if (!ip) return ip
    // IPv4
    let masked = ip.replace(/(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/g, '$1.$2.***.***')
    // IPv6 (简化处理)
    masked = masked.replace(/([0-9a-fA-F]{1,4}):([0-9a-fA-F]{1,4}):.*/g, '$1:$2:****')
    return masked
  }

  /**
   * 脱敏URL参数中的敏感信息
   */
  _maskUrlParams(url) {
    if (!url) return url
    // 脱敏 URL 中的 token、key、password 等参数
    return url
      .replace(/([?&])(token|key|password|pwd|secret|api_key|apikey)=[^&]*/gi, '$1$2=***')
      .replace(/([?&])(access_token|auth)=[^&]*/gi, '$1$2=***')
  }

  /**
   * 脱敏JSON中的敏感字段
   */
  _maskSensitiveFields(obj) {
    if (typeof obj !== 'object' || obj === null) return obj
    
    const sensitiveKeys = [
      'password', 'pwd', 'pass', 'secret', 'token', 'key', 'apikey', 'api_key',
      'access_token', 'auth', 'credential', 'private_key', 'secret_key',
      'ssn', 'social_security', 'credit_card', 'card_number'
    ]
    
    const masked = Array.isArray(obj) ? [] : {}
    
    for (const key in obj) {
      const lowerKey = key.toLowerCase()
      
      // 检查是否是敏感字段
      if (sensitiveKeys.some(sk => lowerKey.includes(sk))) {
        masked[key] = '***'
      } else if (typeof obj[key] === 'string') {
        masked[key] = this._sanitize(obj[key])
      } else if (typeof obj[key] === 'object') {
        masked[key] = this._maskSensitiveFields(obj[key])
      } else {
        masked[key] = obj[key]
      }
    }
    
    return masked
  }

  /**
   * 脱敏字符串（综合脱敏）
   */
  _sanitize(str) {
    if (typeof str !== 'string') return str
    
    let sanitized = str
    // 按顺序应用各种脱敏规则
    sanitized = this._maskPath(sanitized)
    sanitized = this._maskPhone(sanitized)
    sanitized = this._maskIdCard(sanitized)
    sanitized = this._maskEmail(sanitized)
    sanitized = this._maskBankCard(sanitized)
    sanitized = this._maskIP(sanitized)
    sanitized = this._maskUrlParams(sanitized)
    
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
