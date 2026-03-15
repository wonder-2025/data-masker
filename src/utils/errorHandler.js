/**
 * 错误处理工具
 * 防止错误信息泄露敏感路径和系统信息
 * 
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

class ErrorHandler {
  constructor() {
    this.isProduction = import.meta.env.PROD
  }

  /**
   * 脱敏路径信息
   * @param {string} path - 原始路径
   * @returns {string} - 脱敏后的路径
   */
  sanitizePath(path) {
    if (!path || typeof path !== 'string') return path

    // 只显示文件名，隐藏完整路径
    const parts = path.split(/[/\\]/)
    if (parts.length > 1) {
      return `.../${parts[parts.length - 1]}`
    }
    return path
  }

  /**
   * 脱敏错误信息
   * @param {Error|string} error - 错误对象或消息
   * @returns {string} - 脱敏后的错误消息
   */
  sanitizeError(error) {
    if (!error) return '未知错误'

    let message = typeof error === 'string' ? error : (error.message || '操作失败')

    // 脱敏路径
    message = message.replace(
      /(?:\/[\w\-\.]+)+\/[\w\-\.]+/g,
      match => this.sanitizePath(match)
    )

    // 脱敏 Windows 路径
    message = message.replace(
      /[A-Z]:\\[\w\-\.\\]+/g,
      match => this.sanitizePath(match)
    )

    // 移除内部实现细节
    message = message.replace(/at\s+.+/g, '')  // 移除调用栈
    message = message.replace(/\s+/g, ' ').trim()

    // 生产环境使用通用错误提示
    if (this.isProduction) {
      // 如果是文件相关错误，返回通用提示
      if (message.includes('文件') || message.includes('file')) {
        return '文件操作失败，请检查文件是否存在且有访问权限'
      }
      if (message.includes('权限') || message.includes('permission')) {
        return '权限不足，无法执行该操作'
      }
      if (message.includes('网络') || message.includes('network')) {
        return '网络连接失败，请检查网络设置'
      }
    }

    return message || '操作失败'
  }

  /**
   * 创建用户友好的错误消息
   * @param {string} operation - 操作类型
   * @param {Error|string} error - 原始错误
   * @returns {string} - 用户友好的错误消息
   */
  createUserMessage(operation, error) {
    // 开发环境显示详细错误
    if (!this.isProduction) {
      return this.sanitizeError(error)
    }

    // 生产环境返回通用消息
    const operationMessages = {
      'file_read': '读取文件失败',
      'file_write': '保存文件失败',
      'file_delete': '删除文件失败',
      'file_select': '选择文件失败',
      'file_scan': '扫描文件夹失败',
      'mask_process': '脱敏处理失败',
      'rule_validate': '规则验证失败',
      'export': '导出失败',
      'import': '导入失败',
      'config_save': '保存配置失败',
      'config_load': '加载配置失败'
    }

    return operationMessages[operation] || '操作失败，请重试'
  }

  /**
   * 记录错误日志（内部使用）
   * @param {string} operation - 操作类型
   * @param {Error|string} error - 错误详情
   * @param {object} context - 上下文信息
   */
  logError(operation, error, context = {}) {
    // 详细日志记录到控制台（仅开发环境）
    if (!this.isProduction) {
      console.group(`[Error] ${operation}`)
      console.error('Error:', error)
      console.error('Context:', context)
      console.groupEnd()
    }

    // 生产环境可以发送到错误收集服务
    // 这里只是示例，实际应该调用日志收集器
    if (this.isProduction && typeof window !== 'undefined') {
      // 可以集成到现有的日志收集系统
      const logData = {
        operation,
        message: this.sanitizeError(error),
        timestamp: new Date().toISOString(),
        ...context
      }
      
      // 发送到日志服务（如果已配置）
      // logCollector.error(operation, logData)
    }
  }

  /**
   * 包装异步函数，自动处理错误
   * @param {Function} fn - 异步函数
   * @param {string} operation - 操作名称
   * @returns {Function} - 包装后的函数
   */
  wrapAsync(fn, operation) {
    return async (...args) => {
      try {
        return await fn(...args)
      } catch (error) {
        // 记录详细错误
        this.logError(operation, error, { args: args.length })

        // 返回用户友好的错误消息
        const userMessage = this.createUserMessage(operation, error)
        throw new Error(userMessage)
      }
    }
  }

  /**
   * 显示错误提示
   * @param {string} operation - 操作类型
   * @param {Error|string} error - 错误信息
   * @param {object} options - 选项
   */
  showError(operation, error, options = {}) {
    const message = this.createUserMessage(operation, error)
    
    // 使用 Element Plus 的消息提示（如果可用）
    if (typeof ElMessage !== 'undefined') {
      ElMessage.error({
        message,
        duration: options.duration || 3000,
        ...options
      })
    } else {
      // 回退到 alert
      alert(message)
    }

    // 记录错误
    this.logError(operation, error)
  }

  /**
   * 显示警告提示
   * @param {string} message - 警告消息
   */
  showWarning(message) {
    if (typeof ElMessage !== 'undefined') {
      ElMessage.warning(message)
    } else {
      console.warn(message)
    }
  }

  /**
   * 显示成功提示
   * @param {string} message - 成功消息
   */
  showSuccess(message) {
    if (typeof ElMessage !== 'undefined') {
      ElMessage.success(message)
    }
  }
}

// 导出单例
export const errorHandler = new ErrorHandler()

// 导出类
export default ErrorHandler
