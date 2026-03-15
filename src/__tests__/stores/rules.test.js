/**
 * 规则管理 Store 测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useRulesStore } from '@/stores/rules'

// Mock localStorage
const localStorageMock = (() => {
  let store = {}
  return {
    getItem: vi.fn((key) => store[key] || null),
    setItem: vi.fn((key, value) => {
      store[key] = value
    }),
    removeItem: vi.fn((key) => {
      delete store[key]
    }),
    clear: vi.fn(() => {
      store = {}
    })
  }
})()

Object.defineProperty(global, 'localStorage', {
  value: localStorageMock
})

// Mock ElMessage
vi.mock('element-plus', () => ({
  ElMessage: {
    success: vi.fn(),
    error: vi.fn()
  }
}))

describe('Rules Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
  })

  describe('初始状态', () => {
    it('应该包含内置规则', () => {
      const store = useRulesStore()
      expect(store.builtinRulesList.length).toBeGreaterThan(0)
    })

    it('初始自定义规则应为空', () => {
      const store = useRulesStore()
      expect(store.customRules).toEqual([])
    })

    it('allRules 应包含所有规则', () => {
      const store = useRulesStore()
      expect(store.allRules.length).toBe(store.builtinRulesList.length + store.customRules.length)
    })
  })

  describe('规则操作', () => {
    it('toggleRule 应切换规则启用状态', () => {
      const store = useRulesStore()
      const firstRule = store.builtinRulesList[0]
      const originalState = firstRule.enabled
      
      store.toggleRule(firstRule.id)
      expect(firstRule.enabled).toBe(!originalState)
      
      // 恢复
      store.toggleRule(firstRule.id)
      expect(firstRule.enabled).toBe(originalState)
    })

    it('updateRule 应更新规则属性', () => {
      const store = useRulesStore()
      const firstRule = store.builtinRulesList[0]
      
      store.updateRule(firstRule.id, { name: '测试规则' })
      expect(firstRule.name).toBe('测试规则')
    })

    it('addCustomRule 应添加新规则', () => {
      const store = useRulesStore()
      const initialCount = store.customRules.length
      
      const newRule = {
        name: '自定义规则',
        type: 'custom',
        pattern: 'test',
        description: '测试规则',
        strategy: 'full_mask'
      }
      
      store.addCustomRule(newRule)
      expect(store.customRules.length).toBe(initialCount + 1)
      expect(store.customRules[store.customRules.length - 1].name).toBe('自定义规则')
      expect(store.customRules[store.customRules.length - 1].enabled).toBe(true)
    })

    it('deleteCustomRule 应删除指定规则', () => {
      const store = useRulesStore()
      
      const newRule = store.addCustomRule({
        name: '待删除规则',
        type: 'custom',
        pattern: 'test',
        strategy: 'full_mask'
      })
      
      const countAfterAdd = store.customRules.length
      store.deleteCustomRule(newRule.id)
      expect(store.customRules.length).toBe(countAfterAdd - 1)
    })

    it('deleteCustomRule 不应删除内置规则', () => {
      const store = useRulesStore()
      const builtinId = store.builtinRulesList[0].id
      const builtinCount = store.builtinRulesList.length
      
      store.deleteCustomRule(builtinId)
      expect(store.builtinRulesList.length).toBe(builtinCount)
    })
  })

  describe('规则重置', () => {
    it('resetRules 应重置所有规则到默认状态', () => {
      const store = useRulesStore()
      
      // 修改规则
      store.toggleRule(store.builtinRulesList[0].id)
      store.addCustomRule({
        name: '临时规则',
        type: 'custom',
        pattern: 'test',
        strategy: 'full_mask'
      })
      
      store.resetRules()
      
      expect(store.customRules).toEqual([])
      // 验证内置规则已恢复
      expect(store.builtinRulesList.every(r => r.enabled === true || r.enabled === false)).toBe(true)
    })
  })

  describe('规则导入导出', () => {
    it('exportRules 应返回规则数据', () => {
      const store = useRulesStore()
      const exported = store.exportRules()
      
      expect(exported).toHaveProperty('builtin')
      expect(exported).toHaveProperty('custom')
      expect(exported).toHaveProperty('exportedAt')
      expect(Array.isArray(exported.builtin)).toBe(true)
      expect(Array.isArray(exported.custom)).toBe(true)
    })

    it('importRules 应导入有效数据', () => {
      const store = useRulesStore()
      
      const data = {
        builtin: [{ id: 'test', name: '测试', enabled: true }],
        custom: [{ id: 'custom_test', name: '自定义测试', enabled: true }]
      }
      
      const result = store.importRules(data)
      expect(result).toBe(true)
    })

    it('importRules 应拒绝无效数据', () => {
      const store = useRulesStore()
      
      const result = store.importRules(null)
      expect(result).toBe(false)
      
      const result2 = store.importRules({ invalid: 'data' })
      expect(result2).toBe(false)
    })
  })

  describe('计算属性', () => {
    it('enabledRules 应只返回启用的规则', () => {
      const store = useRulesStore()
      
      // 禁用第一个规则
      const firstRule = store.builtinRulesList[0]
      store.toggleRule(firstRule.id)
      
      // 验证 enabledRules 不包含被禁用的规则
      const hasDisabled = store.enabledRules.some(r => r.id === firstRule.id && !r.enabled)
      expect(hasDisabled).toBe(false)
    })

    it('ruleStats 应返回正确的统计信息', () => {
      const store = useRulesStore()
      const stats = store.ruleStats
      
      expect(stats).toHaveProperty('builtinTotal')
      expect(stats).toHaveProperty('builtinEnabled')
      expect(stats).toHaveProperty('customTotal')
      expect(stats).toHaveProperty('customEnabled')
    })
  })
})
