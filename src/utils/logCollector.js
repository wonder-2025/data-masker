/**
 * 日志收集工具 v5.0 - 完善版
 * 收集应用的所有日志：错误、操作、调试、行为分析
 * 
 * 安全特性:
 * - 默认关闭日志收集
 * - 用户需显式同意后才启用
 * - 完善的敏感信息脱敏
 * 
 * 功能特性:
 * - 支持多种日志级别
 * - 自动收集关键操作
 * - 丰富的上下文信息
 * - 批量日志发送
 * - 本地日志缓存
 */

class LogCollector {
  constructor() {
    this.enabled = false
    this.serverUrl = ''
    this.appName = 'data-masker'
    this.version = '1.0.0'
    this.queue = []
    this.flushInterval = null
    this.maxQueueSize = 20
    this.userConsent = false
    
    // 收集配置
    this.config = {
      collectErrors: true,
      collectOperations: true,
      collectAnalytics: true
    }
    
    // 日志级别枚举
    this.levels = {
      DEBUG: 0,
      INFO: 1,
      WARN: 2,
      ERROR: 3
    }
    this.currentLevel = this.levels.INFO
  }

  /**
   * 初始化日志收集器
   */
  init(options = {}) {
    console.log('[LogCollector] 初始化...', options)
    
    // 检查用户同意
    const savedConsent = localStorage.getItem('log-collector-consent')
    this.userConsent = savedConsent === 'true'
    
    // 加载配置
    this.enabled = this.userConsent && (options.enabled ?? false)
    this.serverUrl = options.serverUrl || ''
    this.appName = options.appName || 'data-masker'
    this.version = options.version || '1.0.0'
    
    // 加载收集配置
    const savedConfig = localStorage.getItem('log-collector-config')
    if (savedConfig) {
      try {
        this.config = { ...this.config, ...JSON.parse(savedConfig) }
      } catch (e) {
        console.warn('[LogCollector] 加载配置失败:', e)
      }
    }
    
    console.log('[LogCollector] 初始化完成:', { 
      enabled: this.enabled, 
      userConsent: this.userConsent,
      serverUrl: this.serverUrl,
      config: this.config
    })
    
    // 启动定时刷新
    if (this.enabled && this.userConsent) {
      this._startFlushing()
      this._setupErrorHandlers()
      
      // 记录应用启动
      this.info('APP_START', {
        version: this.version,
        userAgent: navigator.userAgent,
        language: navigator.language,
        platform: navigator.platform,
        screenWidth: window.screen.width,
        screenHeight: window.screen.height,
        viewportWidth: window.innerWidth,
        viewportHeight: window.innerHeight
      })
    }
  }

  /**
   * 启动定时刷新
   */
  _startFlushing() {
    if (this.flushInterval) {
      clearInterval(this.flushInterval)
    }
    this.flushInterval = setInterval(() => this.flush(), 30000)
  }

  /**
   * 设置错误处理器
   */
  _setupErrorHandlers() {
    // JS 错误
    window.addEventListener('error', (event) => {
      if (!this.config.collectErrors) return
      this.error('JS_ERROR', event.message, {
        filename: this._maskPath(event.filename),
        lineno: event.lineno,
        colno: event.colno,
        stack: this._maskStack(event.error?.stack)
      })
    })

    // Promise 拒绝
    window.addEventListener('unhandledrejection', (event) => {
      if (!this.config.collectErrors) return
      this.error('PROMISE_REJECTION', event.reason?.message || String(event.reason), {
        stack: this._maskStack(event.reason?.stack),
        reason: String(event.reason)
      })
    })

    // Vue 错误
    if (window.app && window.app.config) {
      window.app.config.errorHandler = (err, vm, info) => {
        if (!this.config.collectErrors) return
        this.error('VUE_ERROR', err.message || String(err), {
          component: vm?.$options?.name || 'unknown',
          info,
          stack: this._maskStack(err.stack)
        })
      }
    }
  }

  /**
   * 设置日志级别
   */
  setLevel(level) {
    if (typeof level === 'string') {
      this.currentLevel = this.levels[level.toUpperCase()] || this.levels.INFO
    } else {
      this.currentLevel = level
    }
    console.log('[LogCollector] 日志级别设置为:', this.currentLevel)
  }

  /**
   * 调试日志
   */
  debug(event, detail = {}) {
    if (!this.config.collectAnalytics) return
    this._log(this.levels.DEBUG, 'debug', event, detail.message || '', detail)
  }

  /**
   * 信息日志
   */
  info(event, detail = {}) {
    this._log(this.levels.INFO, 'info', event, detail.message || '', detail)
  }

  /**
   * 警告日志
   */
  warn(event, detail = {}) {
    this._log(this.levels.WARN, 'warning', event, detail.message || '', detail)
  }

  /**
   * 错误日志
   */
  error(event, message, data = {}) {
    if (!this.config.collectErrors) return
    this._log(this.levels.ERROR, 'error', event, message, data)
  }

  /**
   * 操作日志
   */
  operation(event, detail = {}) {
    if (!this.config.collectOperations) return
    this._log(this.levels.INFO, 'operation', event, detail.message || '', {
      ...detail,
      timestamp: new Date().toISOString()
    })
  }

  /**
   * 行为分析日志
   */
  analytics(event, detail = {}) {
    if (!this.config.collectAnalytics) return
    this._log(this.levels.INFO, 'analytics', event, detail.message || '', detail)
  }

  /**
   * 记录页面访问
   */
  pageView(pageName, meta = {}) {
    this.analytics('PAGE_VIEW', { page: pageName, url: window.location.href, ...meta })
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
      fileName: this._maskPath(fileInfo.name),
      fileCount: fileInfo.count
    })
  }

  /**
   * 记录处理结果
   */
  processingResult(fileName, result) {
    this.operation('PROCESSING_RESULT', {
      fileName: this._maskPath(fileName),
      success: result.status === 'done',
      sensitiveCount: result.sensitiveCount,
      processingTime: result.processingTime,
      error: result.error
    })
  }

  /**
   * 记录规则变更
   */
  ruleChange(action, ruleInfo) {
    this.operation('RULE_CHANGE', {
      action,
      ruleId: ruleInfo.id,
      ruleName: ruleInfo.name,
      enabled: ruleInfo.enabled,
      strategy: ruleInfo.strategy
    })
  }

  /**
   * 记录设置变更
   */
  settingChange(key, oldValue, newValue) {
    this.operation('SETTING_CHANGE', {
      key,
      // 敏感值不记录具体内容
      changed: oldValue !== newValue
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
  _log(level, logType, event, message, data = {}) {
    // 检查日志级别
    if (level < this.currentLevel) return
    
    // 检查是否启用
    if (!this.enabled || !this.serverUrl) return
    
    const logData = {
      level,
      log_type: logType,
      event,
      message: this._sanitize(String(message)),
      version: this.version,
      timestamp: new Date().toISOString(),
      // 浏览器信息
      os: navigator.platform,
      browser: this._getBrowserInfo(),
      // 上下文信息
      url: window.location.href,
      referrer: document.referrer,
      viewport: `${window.innerWidth}x${window.innerHeight}`,
      // 数据
      ...this._sanitizeObject(data)
    }

    this.queue.push(logData)

    // 队列满了或者错误立即发送
    if (this.queue.length >= this.maxQueueSize || level >= this.levels.ERROR) {
      this.flush()
    }
  }

  /**
   * 获取浏览器信息
   */
  _getBrowserInfo() {
    const ua = navigator.userAgent
    let browser = 'unknown'
    
    if (ua.indexOf('Firefox') > -1) browser = 'Firefox'
    else if (ua.indexOf('Chrome') > -1) browser = 'Chrome'
    else if (ua.indexOf('Safari') > -1) browser = 'Safari'
    else if (ua.indexOf('Edge') > -1) browser = 'Edge'
    else if (ua.indexOf('MSIE') > -1 || ua.indexOf('Trident') > -1) browser = 'IE'
    
    return browser
  }

  /**
   * 刷新队列
   */
  async flush() {
    if (!this.enabled || !this.serverUrl || this.queue.length === 0) return

    const logs = [...this.queue]
    this.queue = []

    try {
      // 批量发送
      const response = await fetch(this.serverUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          app_name: this.appName,
          logs: logs
        })
      })
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`)
      }
      
      console.log('[LogCollector] 发送成功:', logs.length, '条日志')
    } catch (error) {
      console.error('[LogCollector] 发送失败:', error)
      // 发送失败，重新加入队列（限制重试次数）
      if (this.queue.length < this.maxQueueSize) {
        this.queue = [...logs, ...this.queue]
      } else {
        // 队列已满，丢弃旧日志
        console.warn('[LogCollector] 队列已满，丢弃日志')
      }
    }
  }

  /**
   * 脱敏路径
   */
  _maskPath(path) {
    if (!path) return ''
    return path
      .replace(/\/Users\/[^/]+/g, '/Users/***')
      .replace(/\/home\/[^/]+/g, '/home/***')
      .replace(/C:\\Users\\[^\\]+/g, 'C:\\Users\\***')
      .replace(/\/root\/[^/]*/g, '/root/***')
      .replace(/\/var\/www\/[^/]*/g, '/var/www/***')
  }

  /**
   * 脱敏堆栈信息
   */
  _maskStack(stack) {
    if (!stack) return ''
    return stack
      .replace(/\/Users\/[^/]+/g, '/Users/***')
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
    return ip.replace(/(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/g, '$1.$2.***.***')
  }

  /**
   * 脱敏URL参数
   */
  _maskUrlParams(url) {
    if (!url) return url
    return url
      .replace(/([?&])(token|key|password|pwd|secret|api_key|apikey)=[^&]*/gi, '$1$2=***')
      .replace(/([?&])(access_token|auth)=[^&]*/gi, '$1$2=***')
  }

  /**
   * 脱敏对象中的敏感字段
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
   * 综合脱敏
   */
  _sanitize(str) {
    if (typeof str !== 'string') return str
    let sanitized = str
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
   * 脱敏整个对象
   */
  _sanitizeObject(obj) {
    if (typeof obj !== 'object' || obj === null) return obj
    return this._maskSensitiveFields(obj)
  }

  /**
   * 设置用户同意
   */
  setConsent(consent) {
    this.userConsent = consent
    localStorage.setItem('log-collector-consent', String(consent))
    
    if (!consent) {
      this.enabled = false
      if (this.flushInterval) {
        clearInterval(this.flushInterval)
        this.flushInterval = null
      }
    }
  }

  /**
   * 更新配置
   */
  updateConfig(options = {}) {
    if (options.enabled !== undefined) {
      this.enabled = options.enabled && this.userConsent
      if (this.enabled && !this.flushInterval) {
        this._startFlushing()
        this._setupErrorHandlers()
      }
    }
    if (options.serverUrl !== undefined) this.serverUrl = options.serverUrl
    if (options.collectErrors !== undefined) this.config.collectErrors = options.collectErrors
    if (options.collectOperations !== undefined) this.config.collectOperations = options.collectOperations
    if (options.collectAnalytics !== undefined) this.config.collectAnalytics = options.collectAnalytics
    
    // 保存配置
    localStorage.setItem('log-collector-config', JSON.stringify(this.config))
    
    console.log('[LogCollector] 配置更新:', { 
      enabled: this.enabled, 
      serverUrl: this.serverUrl,
      config: this.config 
    })
  }

  /**
   * 获取队列状态
   */
  getStatus() {
    return {
      enabled: this.enabled,
      queueLength: this.queue.length,
      userConsent: this.userConsent,
      config: this.config,
      version: this.version
    }
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
