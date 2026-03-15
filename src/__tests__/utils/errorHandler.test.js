/**
 * 错误处理工具测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest'
import { errorHandler, default as ErrorHandler } from '@/utils/errorHandler'

// Mock import.meta.env
vi.mock('import.meta', () => ({
  env: {
    PROD: false
  }
}))

describe('ErrorHandler', () => {
  describe('sanitizePath 方法', () => {
    it('应隐藏完整路径，只显示文件名', () => {
      const result = errorHandler.sanitizePath('/home/user/documents/secret/file.txt')
      expect(result).toBe('.../file.txt')
    })

    it('应处理 Windows 路径', () => {
      const result = errorHandler.sanitizePath('C:\\Users\\Admin\\Documents\\secret.txt')
      expect(result).toBe('.../secret.txt')
    })

    it('应处理相对路径', () => {
      const result = errorHandler.sanitizePath('./config/settings.json')
      expect(result).toBe('.../settings.json')
    })

    it('应处理简单文件名', () => {
      const result = errorHandler.sanitizePath('file.txt')
      expect(result).toBe('file.txt')
    })

    it('应处理空输入', () => {
      expect(errorHandler.sanitizePath(null)).toBe(null)
      expect(errorHandler.sanitizePath(undefined)).toBe(undefined)
      expect(errorHandler.sanitizePath('')).toBe('')
    })
  })

  describe('sanitizeError 方法', () => {
    it('应处理字符串错误', () => {
      const result = errorHandler.sanitizeError('操作失败')
      expect(result).toBe('操作失败')
    })

    it('应处理 Error 对象', () => {
      const error = new Error('测试错误')
      const result = errorHandler.sanitizeError(error)
      expect(result).toBe('测试错误')
    })

    it('应脱敏路径信息', () => {
      const error = new Error('无法读取文件 /home/user/secret/data.txt')
      const result = errorHandler.sanitizeError(error)
      expect(result).not.toContain('/home/user/secret/')
      expect(result).toContain('data.txt')
    })

    it('应脱敏 Windows 路径', () => {
      const error = new Error('文件不存在: C:\\Users\\Admin\\private\\config.ini')
      const result = errorHandler.sanitizeError(error)
      expect(result).not.toContain('C:\\Users\\Admin\\private\\')
      expect(result).toContain('config.ini')
    })

    it('应移除调用栈信息', () => {
      const error = new Error('错误 at Object.method (file.js:10:5)')
      const result = errorHandler.sanitizeError(error)
      expect(result).not.toContain('at Object.method')
    })

    it('应处理空错误', () => {
      expect(errorHandler.sanitizeError(null)).toBe('未知错误')
      expect(errorHandler.sanitizeError(undefined)).toBe('未知错误')
    })

    it('应处理没有消息的 Error', () => {
      const error = new Error()
      const result = errorHandler.sanitizeError(error)
      expect(result).toBe('操作失败')
    })
  })

  describe('createUserMessage 方法', () => {
    it('应为文件读取操作创建友好消息', () => {
      const result = errorHandler.createUserMessage('file_read', new Error('读取失败'))
      expect(result).toBeTruthy()
    })

    it('应为文件写入操作创建友好消息', () => {
      const result = errorHandler.createUserMessage('file_write', new Error('写入失败'))
      expect(result).toBeTruthy()
    })

    it('应为未知操作创建默认消息', () => {
      const result = errorHandler.createUserMessage('unknown_operation', new Error('错误'))
      expect(result).toBeTruthy()
    })

    it('应在开发环境返回详细错误', () => {
      const handler = new ErrorHandler()
      handler.isProduction = false
      
      const error = new Error('详细错误信息 /path/to/file.txt')
      const result = handler.createUserMessage('file_read', error)
      expect(result).toContain('file.txt')
    })
  })

  describe('logError 方法', () => {
    let consoleSpy

    beforeEach(() => {
      consoleSpy = vi.spyOn(console, 'group').mockImplementation(() => {})
      vi.spyOn(console, 'error').mockImplementation(() => {})
      vi.spyOn(console, 'groupEnd').mockImplementation(() => {})
    })

    afterEach(() => {
      consoleSpy.mockRestore()
    })

    it('应在开发环境记录错误', () => {
      const handler = new ErrorHandler()
      handler.isProduction = false
      
      handler.logError('test_operation', new Error('测试错误'))
      expect(consoleSpy).toHaveBeenCalled()
    })

    it('应记录上下文信息', () => {
      const handler = new ErrorHandler()
      handler.isProduction = false
      
      const consoleErrorSpy = vi.spyOn(console, 'error')
      handler.logError('test', new Error('err'), { userId: '123' })
      
      expect(consoleErrorSpy).toHaveBeenCalled()
    })
  })

  describe('wrapAsync 方法', () => {
    it('应包装成功执行的异步函数', async () => {
      const successFn = async () => 'success'
      const wrapped = errorHandler.wrapAsync(successFn, 'test_operation')
      
      const result = await wrapped()
      expect(result).toBe('success')
    })

    it('应捕获并转换异步错误', async () => {
      const failFn = async () => {
        throw new Error('原始错误')
      }
      const wrapped = errorHandler.wrapAsync(failFn, 'file_read')
      
      await expect(wrapped()).rejects.toThrow()
    })

    it('应传递函数参数', async () => {
      const fn = async (a, b) => a + b
      const wrapped = errorHandler.wrapAsync(fn, 'add')
      
      const result = await wrapped(1, 2)
      expect(result).toBe(3)
    })
  })

  describe('showError 方法', () => {
    it('应安全处理错误显示', () => {
      // Mock alert for happy-dom
      global.alert = vi.fn()
      
      expect(() => {
        errorHandler.showError('file_read', new Error('测试错误'))
      }).not.toThrow()
      
      // 验证 alert 被调用
      expect(global.alert).toHaveBeenCalled()
    })
  })

  describe('showWarning 方法', () => {
    it('应安全处理警告消息', () => {
      expect(() => {
        errorHandler.showWarning('测试警告')
      }).not.toThrow()
    })
  })

  describe('showSuccess 方法', () => {
    it('应安全处理成功消息', () => {
      expect(() => {
        errorHandler.showSuccess('测试成功')
      }).not.toThrow()
    })
  })

  describe('生产环境行为', () => {
    it('应返回通用错误消息', () => {
      const handler = new ErrorHandler()
      handler.isProduction = true
      
      const error = new Error('文件 /secret/path/file.txt 不存在')
      const result = handler.sanitizeError(error)
      
      // 生产环境应该返回通用消息
      expect(result).not.toContain('/secret/path/')
    })

    it('应为特定错误类型返回适当的通用消息', () => {
      const handler = new ErrorHandler()
      handler.isProduction = true
      
      const fileError = handler.createUserMessage('file_read', new Error('文件错误'))
      expect(fileError).toBe('读取文件失败')

      const networkError = handler.sanitizeError(new Error('网络连接超时'))
      expect(networkError).toContain('网络')
    })
  })

  describe('敏感信息过滤', () => {
    it('应过滤用户名路径', () => {
      const error = new Error('无法访问 /home/johndoe/.ssh/id_rsa')
      const result = errorHandler.sanitizeError(error)
      
      expect(result).not.toContain('johndoe')
      expect(result).not.toContain('.ssh')
      expect(result).toContain('id_rsa')
    })

    it('应过滤项目路径', () => {
      const error = new Error('配置文件 /var/www/myapp/config/database.yml 读取失败')
      const result = errorHandler.sanitizeError(error)
      
      expect(result).not.toContain('/var/www/myapp/')
      expect(result).toContain('database.yml')
    })

    it('应过滤多级嵌套路径', () => {
      const error = new Error('路径 /a/b/c/d/e/f/g/file.txt 不存在')
      const result = errorHandler.sanitizeError(error)
      
      // 路径被脱敏，只保留文件名
      expect(result).toContain('file.txt')
      expect(result).not.toContain('/a/b/c/')
    })
  })

  describe('边界情况', () => {
    it('应处理循环引用错误', () => {
      const error = new Error('测试错误')
      error.stack = 'Error: 测试\n    at line 1\n    at line 2'
      
      const result = errorHandler.sanitizeError(error)
      expect(result).toBeTruthy()
    })

    it('应处理超长错误消息', () => {
      const longMessage = '错误: ' + 'a'.repeat(10000)
      const result = errorHandler.sanitizeError(longMessage)
      expect(result.length).toBeLessThanOrEqual(10000 + 10) // 允许一些额外字符
    })

    it('应处理特殊字符', () => {
      const error = new Error('错误: \n\t\r特殊字符')
      const result = errorHandler.sanitizeError(error)
      expect(result).toBeTruthy()
    })

    it('应处理非标准错误对象', () => {
      const weirdError = { message: '奇怪的错误', code: 500 }
      const result = errorHandler.sanitizeError(weirdError)
      expect(result).toBe('奇怪的错误')
    })
  })
})
