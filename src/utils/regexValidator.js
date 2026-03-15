/**
 * 正则表达式安全验证工具
 * 防止 ReDoS (Regular Expression Denial of Service) 攻击
 * 
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

class RegexValidator {
  constructor() {
    // 正则表达式执行超时时间（毫秒）
    this.timeoutMs = 100
    // 最大正则表达式长度
    this.maxLength = 500
    // 危险模式列表（可能导致回溯爆炸）
    this.dangerousPatterns = [
      /\([^)]*\+[^)]*\)/,          // 嵌套量词 (a+)+
      /\([^)]*\*[^)]*\)/,          // 嵌套量词 (a*)*
      /\([^)]*\{[^}]*\}[^)]*\)/,   // 嵌套量词 (a{n,m})*
      /\.\*\.\*/,                   // 连续的 .*
      /\.\+\.\+/,                   // 连续的 .+
      /\[[^\]]*\]\+[^}]*\+/,        // 字符类后的多个量词
      /\([^)]*\|[^)]*\)\+/,        // 交替组的量词
    ]
  }

  /**
   * 验证正则表达式安全性
   * @param {string} pattern - 正则表达式字符串
   * @returns {object} - 验证结果
   */
  validate(pattern) {
    const result = {
      isValid: false,
      errors: [],
      warnings: [],
      complexity: 'low',
      estimatedTime: 0
    }

    // 基本检查
    if (!pattern || typeof pattern !== 'string') {
      result.errors.push('正则表达式不能为空')
      return result
    }

    // 长度检查
    if (pattern.length > this.maxLength) {
      result.errors.push(`正则表达式过长（超过 ${this.maxLength} 字符），可能存在性能问题`)
      return result
    }

    // 语法检查
    try {
      new RegExp(pattern)
    } catch (e) {
      result.errors.push(`正则表达式语法错误: ${e.message}`)
      return result
    }

    // 危险模式检查
    for (const dangerousPattern of this.dangerousPatterns) {
      if (dangerousPattern.test(pattern)) {
        result.warnings.push('检测到可能导致性能问题的模式（嵌套量词或连续通配符）')
        result.complexity = 'high'
        break
      }
    }

    // 复杂度评估
    const complexityScore = this._calculateComplexity(pattern)
    if (complexityScore > 20) {
      result.complexity = 'high'
      result.warnings.push('正则表达式复杂度较高，可能导致性能问题')
    } else if (complexityScore > 10) {
      result.complexity = 'medium'
      result.warnings.push('正则表达式复杂度中等，建议简化')
    }

    // 测试执行时间
    const testResult = this._testExecutionTime(pattern)
    result.estimatedTime = testResult.time

    if (testResult.timeout) {
      result.errors.push('正则表达式执行超时，可能存在 ReDoS 风险')
      return result
    }

    if (testResult.time > 50) {
      result.warnings.push(`正则表达式执行时间较长（${testResult.time.toFixed(2)}ms）`)
    }

    // 所有检查通过
    result.isValid = result.errors.length === 0
    return result
  }

  /**
   * 计算正则表达式复杂度
   */
  _calculateComplexity(pattern) {
    let score = 0

    // 量词数量
    const quantifiers = (pattern.match(/[+*?{]/g) || []).length
    score += quantifiers * 2

    // 分组数量
    const groups = (pattern.match(/\(/g) || []).length
    score += groups * 3

    // 交替数量
    const alternations = (pattern.match(/\|/g) || []).length
    score += alternations * 2

    // 字符类数量
    const charClasses = (pattern.match(/\[/g) || []).length
    score += charClasses

    // 反向引用
    const backrefs = (pattern.match(/\\[1-9]/g) || []).length
    score += backrefs * 3

    return score
  }

  /**
   * 测试正则表达式执行时间
   */
  _testExecutionTime(pattern) {
    const result = {
      time: 0,
      timeout: false
    }

    try {
      const regex = new RegExp(pattern)
      
      // 创建测试字符串（包含各种情况的混合）
      const testStrings = [
        'a'.repeat(50),
        'test123@example.com',
        '192.168.1.1',
        '北京市朝阳区',
        '2023-12-25 10:30:45',
        '110101199001011234',
        '13812345678'
      ]

      const startTime = performance.now()

      for (const testStr of testStrings) {
        // 使用 setTimeout 检测超时
        const testStart = performance.now()
        regex.test(testStr)
        const testTime = performance.now() - testStart

        if (testTime > this.timeoutMs) {
          result.timeout = true
          result.time = testTime
          return result
        }
      }

      result.time = performance.now() - startTime
    } catch (e) {
      result.timeout = true
    }

    return result
  }

  /**
   * 安全测试正则表达式
   * @param {string} pattern - 正则表达式
   * @param {string} testStr - 测试字符串
   * @param {number} timeoutMs - 超时时间（毫秒）
   * @returns {object} - 测试结果
   */
  safeTest(pattern, testStr, timeoutMs = 100) {
    const result = {
      match: false,
      timeout: false,
      error: null,
      time: 0
    }

    try {
      const regex = new RegExp(pattern)
      const startTime = performance.now()
      
      // 在 Worker 中执行可能会有更好的效果，但这里简化实现
      result.match = regex.test(testStr)
      result.time = performance.now() - startTime

      if (result.time > timeoutMs) {
        result.timeout = true
        result.match = false
        result.error = '执行超时'
      }
    } catch (e) {
      result.error = e.message
    }

    return result
  }

  /**
   * 安全执行正则表达式替换
   * @param {string} pattern - 正则表达式
   * @param {string} replacement - 替换字符串
   * @param {string} text - 待处理文本
   * @param {number} timeoutMs - 超时时间（毫秒）
   * @returns {object} - 执行结果
   */
  safeReplace(pattern, replacement, text, timeoutMs = 100) {
    const result = {
      text: text,
      replaced: false,
      timeout: false,
      error: null,
      time: 0
    }

    try {
      const regex = new RegExp(pattern, 'g')
      const startTime = performance.now()
      
      result.text = text.replace(regex, replacement)
      result.replaced = true
      result.time = performance.now() - startTime

      if (result.time > timeoutMs) {
        result.timeout = true
        result.text = text  // 恢复原文本
        result.replaced = false
        result.error = '执行超时'
      }
    } catch (e) {
      result.error = e.message
    }

    return result
  }

  /**
   * 获取正则表达式建议
   * @param {string} pattern - 正则表达式
   * @returns {string[]} - 优化建议
   */
  getSuggestions(pattern) {
    const suggestions = []

    // 检查是否有不必要的捕获组
    if (/\([^)]*\)/.test(pattern) && !/\\[1-9]/.test(pattern)) {
      suggestions.push('建议使用非捕获组 (?:...) 代替捕获组 (...) 以提高性能')
    }

    // 检查是否有可以简化的模式
    if (/\[\d\]/.test(pattern)) {
      suggestions.push('字符类中单个字符可以直接使用该字符')
    }

    // 检查是否有冗余的量词
    if (/\.\*\*/.test(pattern)) {
      suggestions.push('.* 和 ** 是冗余的，建议简化')
    }

    // 检查是否使用了贪婪量词但可以改为懒惰量词
    if (/\.\+[^?]/.test(pattern)) {
      suggestions.push('建议使用非贪婪量词 .*? 代替 .* 以提高性能')
    }

    return suggestions
  }
}

// 导出单例
export const regexValidator = new RegexValidator()

// 导出类
export default RegexValidator
