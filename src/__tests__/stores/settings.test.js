/**
 * 设置管理 Store 测试
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useSettingsStore } from '@/stores/settings'

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

// Mock secureStorage
vi.mock('@/utils/secureStorage', () => ({
  secureStorage: {
    getItem: vi.fn(async () => null),
    setItem: vi.fn(async () => true),
    removeItem: vi.fn(),
    checkPasswordStrength: vi.fn((password) => ({
      score: password.length >= 8 ? 4 : 1,
      level: password.length >= 12 ? 'strong' : password.length >= 8 ? 'medium' : 'weak',
      suggestions: password.length < 8 ? ['密码长度至少8位'] : [],
      isValid: password.length >= 8
    }))
  }
}))

describe('Settings Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    localStorageMock.clear()
  })

  describe('初始设置', () => {
    it('应有默认设置值', () => {
      const store = useSettingsStore()
      
      expect(store.settingsData).toHaveProperty('general')
      expect(store.settingsData).toHaveProperty('masking')
      expect(store.settingsData).toHaveProperty('security')
      expect(store.settingsData).toHaveProperty('advanced')
    })

    it('默认语言应为中文', () => {
      const store = useSettingsStore()
      expect(store.settingsData.general.language).toBe('zh-CN')
    })

    it('默认脱敏字符应为 *', () => {
      const store = useSettingsStore()
      expect(store.settingsData.masking.maskChar).toBe('*')
    })
  })

  describe('设置操作', () => {
    it('updateSetting 应更新单个设置项', () => {
      const store = useSettingsStore()
      
      store.updateSetting('general', 'language', 'en-US')
      expect(store.settingsData.general.language).toBe('en-US')
    })

    it('setOutputDir 应设置输出目录', () => {
      const store = useSettingsStore()
      
      store.setOutputDir('/tmp/output')
      expect(store.settingsData.general.outputDir).toBe('/tmp/output')
    })

    it('getOutputDir 应返回输出目录', () => {
      const store = useSettingsStore()
      store.settingsData.general.outputDir = '/test/path'
      
      expect(store.getOutputDir()).toBe('/test/path')
    })
  })

  describe('密码操作', () => {
    it('setPassword 应拒绝弱密码', async () => {
      const store = useSettingsStore()
      
      const result = await store.setPassword('123')
      expect(result.success).toBe(false)
      expect(result.message).toBe('密码强度不足')
    })

    it('setPassword 应接受强密码', async () => {
      const store = useSettingsStore()
      
      const result = await store.setPassword('StrongP@ss123!')
      expect(result.success).toBe(true)
    })

    it('validatePasswordStrength 应返回密码强度', () => {
      const store = useSettingsStore()
      
      const weak = store.validatePasswordStrength('123')
      expect(weak.isValid).toBe(false)
      
      const strong = store.validatePasswordStrength('StrongP@ss123!')
      expect(strong.isValid).toBe(true)
    })
  })

  describe('重置设置', () => {
    it('resetToDefault 应恢复默认设置', () => {
      const store = useSettingsStore()
      
      // 修改设置
      store.updateSetting('general', 'language', 'en-US')
      store.updateSetting('masking', 'maskChar', '#')
      
      // 重置
      store.resetToDefault()
      
      expect(store.settingsData.general.language).toBe('zh-CN')
      expect(store.settingsData.masking.maskChar).toBe('*')
    })
  })
})
