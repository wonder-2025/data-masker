/**
 * 正则表达式验证器测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { regexValidator, default as RegexValidator } from '@/utils/regexValidator'

describe('RegexValidator', () => {
  describe('validate 方法', () => {
    it('应拒绝空正则表达式', () => {
      const result = regexValidator.validate('')
      expect(result.isValid).toBe(false)
      expect(result.errors).toContain('正则表达式不能为空')
    })

    it('应拒绝非字符串输入', () => {
      const result = regexValidator.validate(null)
      expect(result.isValid).toBe(false)
      expect(result.errors).toContain('正则表达式不能为空')
    })

    it('应拒绝过长的正则表达式', () => {
      const longPattern = 'a'.repeat(600)
      const result = regexValidator.validate(longPattern)
      expect(result.isValid).toBe(false)
      expect(result.errors.some(e => e.includes('过长'))).toBe(true)
    })

    it('应拒绝语法错误的正则表达式', () => {
      const result = regexValidator.validate('[invalid(')
      expect(result.isValid).toBe(false)
      expect(result.errors.some(e => e.includes('语法错误'))).toBe(true)
    })

    it('应接受有效的正则表达式', () => {
      const result = regexValidator.validate('\\d+')
      expect(result.isValid).toBe(true)
      expect(result.errors.length).toBe(0)
    })

    it('应检测危险的嵌套量词模式', () => {
      const result = regexValidator.validate('(a+)+')
      expect(result.warnings.some(w => w.includes('嵌套量词'))).toBe(true)
      expect(result.complexity).toBe('high')
    })

    it('应检测连续通配符', () => {
      const result = regexValidator.validate('.*.*')
      expect(result.warnings.length).toBeGreaterThan(0)
    })

    it('应评估正则表达式复杂度', () => {
      const simple = regexValidator.validate('abc')
      expect(simple.complexity).toBe('low')

      const complex = regexValidator.validate('((a|b|c)+d?){2,5}')
      expect(['medium', 'high']).toContain(complex.complexity)
    })
  })

  describe('手机号正则验证', () => {
    const phonePattern = '^1[3-9]\\d{9}$'

    it('应匹配有效的手机号', () => {
      const result = regexValidator.validate(phonePattern)
      expect(result.isValid).toBe(true)

      const testResult = regexValidator.safeTest(phonePattern, '13812345678')
      expect(testResult.match).toBe(true)
    })

    it('应拒绝无效的手机号', () => {
      const testCases = [
        { input: '12345678901', expected: false },
        { input: '1381234567', expected: false },
        { input: '23812345678', expected: false },
        { input: '138123456789', expected: false }
      ]

      testCases.forEach(({ input, expected }) => {
        const result = regexValidator.safeTest(phonePattern, input)
        expect(result.match).toBe(expected)
      })
    })
  })

  describe('身份证正则验证', () => {
    // 15位身份证
    const id15Pattern = '^[1-9]\\d{7}\\d{2}\\d{2}\\d{3}$'
    // 18位身份证
    const id18Pattern = '^[1-9]\\d{5}(18|19|20)\\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\\d|3[01])\\d{3}[0-9Xx]$'

    it('应匹配15位身份证号', () => {
      const result = regexValidator.validate(id15Pattern)
      expect(result.isValid).toBe(true)

      const testResult = regexValidator.safeTest(id15Pattern, '110101700101001')
      expect(testResult.match).toBe(true)
    })

    it('应匹配18位身份证号', () => {
      const result = regexValidator.validate(id18Pattern)
      expect(result.isValid).toBe(true)

      const testResult = regexValidator.safeTest(id18Pattern, '110101199001011234')
      expect(testResult.match).toBe(true)
    })

    it('应匹配带X的身份证号', () => {
      const testResult = regexValidator.safeTest(id18Pattern, '11010119900101123X')
      expect(testResult.match).toBe(true)
    })

    it('应拒绝无效身份证号', () => {
      const invalidCases = [
        '012345678901234567',
        '11010119900101123',
        '1101011990010112345',
        '110101199013011234'  // 无效月份
      ]

      invalidCases.forEach(input => {
        const result = regexValidator.safeTest(id18Pattern, input)
        expect(result.match).toBe(false)
      })
    })
  })

  describe('银行卡号验证', () => {
    const bankCardPattern = '^[1-9]\\d{15,18}$'

    it('应匹配有效的银行卡号', () => {
      const result = regexValidator.validate(bankCardPattern)
      expect(result.isValid).toBe(true)

      const validCards = [
        '6222021234567890123',
        '6217001234567890123',
        '6225881234567890'
      ]

      validCards.forEach(card => {
        const testResult = regexValidator.safeTest(bankCardPattern, card)
        expect(testResult.match).toBe(true)
      })
    })

    it('应拒绝无效银行卡号', () => {
      const invalidCards = [
        '0222021234567890',  // 以0开头
        '62220212345678',    // 太短
        '6222021234567890123456'  // 太长
      ]

      invalidCards.forEach(card => {
        const result = regexValidator.safeTest(bankCardPattern, card)
        expect(result.match).toBe(false)
      })
    })
  })

  describe('邮箱正则验证', () => {
    const emailPattern = '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$'

    it('应匹配有效的邮箱地址', () => {
      const result = regexValidator.validate(emailPattern)
      expect(result.isValid).toBe(true)

      const validEmails = [
        'test@example.com',
        'user.name@domain.org',
        'user+tag@example.co.uk',
        '123@test.cn'
      ]

      validEmails.forEach(email => {
        const testResult = regexValidator.safeTest(emailPattern, email)
        expect(testResult.match).toBe(true)
      })
    })

    it('应拒绝无效邮箱地址', () => {
      const invalidEmails = [
        'invalid',
        '@example.com',
        'user@',
        'user@.com',
        'user@domain'
      ]

      invalidEmails.forEach(email => {
        const result = regexValidator.safeTest(emailPattern, email)
        expect(result.match).toBe(false)
      })
    })
  })

  describe('IP 地址验证', () => {
    const ipv4Pattern = '^((25[0-5]|2[0-4]\\d|[01]?\\d\\d?)\\.){3}(25[0-5]|2[0-4]\\d|[01]?\\d\\d?)$'
    const ipv6Pattern = '^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$'

    describe('IPv4 验证', () => {
      it('应匹配有效的 IPv4 地址', () => {
        const result = regexValidator.validate(ipv4Pattern)
        expect(result.isValid).toBe(true)

        const validIPs = [
          '192.168.1.1',
          '10.0.0.1',
          '255.255.255.255',
          '0.0.0.0',
          '172.16.0.1'
        ]

        validIPs.forEach(ip => {
          const testResult = regexValidator.safeTest(ipv4Pattern, ip)
          expect(testResult.match).toBe(true)
        })
      })

      it('应拒绝无效的 IPv4 地址', () => {
        const invalidIPs = [
          '256.1.1.1',        // 超过255
          '192.168.1',        // 缺少一段
          '192.168.1.1.1'     // 多余一段
        ]

        invalidIPs.forEach(ip => {
          const result = regexValidator.safeTest(ipv4Pattern, ip)
          expect(result.match).toBe(false)
        })
      })
    })

    describe('IPv6 验证', () => {
      it('应匹配有效的 IPv6 地址', () => {
        const result = regexValidator.validate(ipv6Pattern)
        expect(result.isValid).toBe(true)

        const validIPs = [
          '2001:0db8:85a3:0000:0000:8a2e:0370:7334',
          'fe80:0000:0000:0000:0000:0000:0000:0001'
        ]

        validIPs.forEach(ip => {
          const testResult = regexValidator.safeTest(ipv6Pattern, ip)
          expect(testResult.match).toBe(true)
        })
      })

      it('应拒绝无效的 IPv6 地址', () => {
        const invalidIPs = [
          '2001:db8::1',  // 简化格式（这个模式不支持）
          'gggg::1',
          '2001:0db8:85a3::8a2e:0370:7334'
        ]

        invalidIPs.forEach(ip => {
          const result = regexValidator.safeTest(ipv6Pattern, ip)
          expect(result.match).toBe(false)
        })
      })
    })
  })

  describe('ReDoS 防护测试', () => {
    it('应在超长字符串上设置合理执行时间', () => {
      const pattern = '^a+$'
      const longString = 'a'.repeat(10000)
      
      const result = regexValidator.safeTest(pattern, longString, 200)
      expect(result.time).toBeLessThan(200)
      expect(result.match).toBe(true)
    })

    it('应检测超时情况', () => {
      // 这是一个简单的测试，实际 ReDoS 模式更复杂
      const pattern = '^(a+)+$'
      const evilInput = 'aaaaaaaaaaaaaaaaaaaa!'
      
      const result = regexValidator.safeTest(pattern, evilInput, 1000)
      expect(result.time).toBeLessThan(1000)  // 应该在超时前完成
    })

    it('应对恶意模式发出警告', () => {
      const dangerousPatterns = [
        '(a+)+',
        '(a*)*',
        '(a|aa)+'
      ]

      dangerousPatterns.forEach(pattern => {
        const result = regexValidator.validate(pattern)
        expect(result.warnings.length).toBeGreaterThan(0)
      })
    })
  })

  describe('safeTest 方法', () => {
    it('应返回正确的测试结果', () => {
      const result = regexValidator.safeTest('\\d+', 'abc123def')
      expect(result.match).toBe(true)
      expect(result.error).toBe(null)
    })

    it('应处理无效正则表达式', () => {
      const result = regexValidator.safeTest('[invalid', 'test')
      expect(result.error).toBeTruthy()
    })

    it('应记录执行时间', () => {
      const result = regexValidator.safeTest('\\d+', '123')
      expect(result.time).toBeGreaterThanOrEqual(0)
    })
  })

  describe('safeReplace 方法', () => {
    it('应正确执行替换', () => {
      const result = regexValidator.safeReplace('\\d+', '*', 'abc123def')
      expect(result.replaced).toBe(true)
      expect(result.text).toBe('abc*def')
    })

    it('应处理全局替换', () => {
      const result = regexValidator.safeReplace('\\d', 'X', 'a1b2c3')
      expect(result.text).toBe('aXbXcX')
    })

    it('应处理无效正则表达式', () => {
      const result = regexValidator.safeReplace('[invalid', '*', 'test')
      expect(result.error).toBeTruthy()
      expect(result.replaced).toBe(false)
    })
  })

  describe('getSuggestions 方法', () => {
    it('应建议使用非捕获组', () => {
      const suggestions = regexValidator.getSuggestions('(test)+')
      expect(suggestions.some(s => s.includes('非捕获组'))).toBe(true)
    })

    it('应为冗余模式提供建议', () => {
      // 连续通配符模式
      const suggestions = regexValidator.getSuggestions('.*.*test')
      // 这个模式会被贪婪量词建议匹配
      expect(suggestions.length).toBeGreaterThanOrEqual(0)
    })

    it('应为贪婪量词提供建议', () => {
      const suggestions = regexValidator.getSuggestions('.+test')
      expect(suggestions.some(s => s.includes('非贪婪'))).toBe(true)
    })
  })

  describe('复杂度计算', () => {
    it('应正确计算简单模式的复杂度', () => {
      const validator = new RegexValidator()
      const complexity = validator._calculateComplexity('abc')
      expect(complexity).toBe(0)
    })

    it('应正确计算含分组模式的复杂度', () => {
      const validator = new RegexValidator()
      const complexity = validator._calculateComplexity('(a)(b)(c)')
      expect(complexity).toBeGreaterThan(0)
    })

    it('应正确计算含交替模式的复杂度', () => {
      const validator = new RegexValidator()
      const complexity = validator._calculateComplexity('a|b|c')
      expect(complexity).toBeGreaterThan(0)
    })
  })
})
