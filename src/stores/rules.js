// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'

// 内置规则定义
const builtinRules = [
  {
    id: 'phone',
    name: '手机号码',
    type: 'phone',
    pattern: '(?:(?:\\+|00)86)?1[3-9]\\d{9}',
    description: '检测中国大陆手机号码',
    enabled: true,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 3, keepEnd: 4, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  },
  {
    id: 'id_card',
    name: '身份证号',
    type: 'id_card',
    pattern: '[1-9]\\d{5}(?:18|19|20)\\d{2}(?:0[1-9]|1[0-2])(?:0[1-9]|[12]\\d|3[01])\\d{3}[\\dXx]',
    description: '检测18位身份证号码',
    enabled: true,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 6, keepEnd: 4, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  },
  {
    id: 'bank_card',
    name: '银行卡号',
    type: 'bank_card',
    pattern: '\\d{16,19}',
    description: '检测银行卡号（需通过Luhn校验）',
    enabled: true,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 4, keepEnd: 4, maskChar: '*' },
    needLuhnCheck: true,
    mode: 'regex'
  },
  {
    id: 'email',
    name: '电子邮箱',
    type: 'email',
    pattern: '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}',
    description: '检测电子邮箱地址',
    enabled: true,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 2, keepEnd: 0, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  },
  {
    id: 'ipv4',
    name: 'IPv4地址',
    type: 'ipv4',
    pattern: '(?:(?:25[0-5]|2[0-4]\\d|[01]?\\d\\d?)\\.){3}(?:25[0-5]|2[0-4]\\d|[01]?\\d\\d?)',
    description: '检测IPv4地址',
    enabled: true,
    strategy: 'full_mask',
    strategyConfig: { keepStart: 0, keepEnd: 0, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  },
  {
    id: 'credit_card',
    name: '信用卡号',
    type: 'credit_card',
    pattern: '(?:\\d{4}[-\\s]?){3}\\d{4}',
    description: '检测信用卡号',
    enabled: false,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 4, keepEnd: 4, maskChar: '*' },
    needLuhnCheck: true,
    mode: 'regex'
  },
  {
    id: 'passport',
    name: '护照号码',
    type: 'passport',
    pattern: '[EeGg]\\d{8}',
    description: '检测中国护照号码',
    enabled: false,
    strategy: 'full_mask',
    strategyConfig: { keepStart: 0, keepEnd: 0, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  },
  {
    id: 'license_plate',
    name: '车牌号',
    type: 'license_plate',
    pattern: '[京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领][A-Z][A-HJ-NP-Z0-9]{4,5}[A-HJ-NP-Z0-9挂学警港澳]',
    description: '检测中国车牌号',
    enabled: false,
    strategy: 'partial_mask',
    strategyConfig: { keepStart: 1, keepEnd: 1, maskChar: '*' },
    needLuhnCheck: false,
    mode: 'regex'
  }
]

/**
 * 规则管理 Store
 */
export const useRulesStore = defineStore('rules', () => {
  // 内置规则列表
  const builtinRulesList = ref(JSON.parse(JSON.stringify(builtinRules)))
  
  // 自定义规则列表
  const customRules = ref([])
  
  // 所有规则
  const allRules = computed(() => [
    ...builtinRulesList.value,
    ...customRules.value
  ])
  
  // 已启用的规则
  const enabledRules = computed(() => 
    allRules.value.filter(r => r.enabled)
  )
  
  // 规则统计
  const ruleStats = computed(() => ({
    builtinTotal: builtinRulesList.value.length,
    builtinEnabled: builtinRulesList.value.filter(r => r.enabled).length,
    customTotal: customRules.value.length,
    customEnabled: customRules.value.filter(r => r.enabled).length
  }))
  
  /**
   * 切换规则启用状态
   */
  function toggleRule(ruleId) {
    const rule = allRules.value.find(r => r.id === ruleId)
    if (rule) {
      rule.enabled = !rule.enabled
      saveToStorage()
    }
  }
  
  /**
   * 更新规则
   */
  function updateRule(ruleId, updates) {
    // 先在内置规则中查找
    let rule = builtinRulesList.value.find(r => r.id === ruleId)
    if (rule) {
      Object.assign(rule, updates)
      saveToStorage()
      return
    }
    
    // 再在自定义规则中查找
    rule = customRules.value.find(r => r.id === ruleId)
    if (rule) {
      Object.assign(rule, updates)
      saveToStorage()
    }
  }
  
  /**
   * 添加自定义规则
   */
  function addCustomRule(rule) {
    const newRule = {
      id: `custom_${Date.now()}`,
      ...rule,
      enabled: true
    }
    customRules.value.push(newRule)
    saveToStorage()
    return newRule
  }
  
  /**
   * 删除自定义规则
   */
  function deleteCustomRule(ruleId) {
    const index = customRules.value.findIndex(r => r.id === ruleId)
    if (index !== -1) {
      customRules.value.splice(index, 1)
      saveToStorage()
    }
  }
  
  /**
   * 重置所有规则到默认状态
   */
  function resetRules() {
    // 深拷贝原始规则，确保完全重置
    builtinRulesList.value = JSON.parse(JSON.stringify(builtinRules))
    customRules.value = []
    saveToStorage()
    ElMessage.success('规则已重置为默认状态')
  }
  
  /**
   * 重置内置规则到默认状态
   */
  function resetBuiltinRules() {
    builtinRulesList.value = JSON.parse(JSON.stringify(builtinRules))
    saveToStorage()
    ElMessage.success('内置规则已重置')
  }
  
  /**
   * 导出规则
   */
  function exportRules() {
    return {
      builtin: builtinRulesList.value,
      custom: customRules.value,
      exportedAt: new Date().toISOString()
    }
  }
  
  /**
   * 导入规则
   */
  function importRules(data) {
    try {
      // 验证数据格式
      if (!data || typeof data !== 'object') {
        throw new Error('无效的规则数据格式')
      }
      
      // 检查是否是有效的规则文件
      if (!data.builtin && !data.custom) {
        throw new Error('规则文件格式不正确，缺少 builtin 或 custom 字段')
      }
      
      if (data.builtin && Array.isArray(data.builtin)) {
        builtinRulesList.value = data.builtin
      }
      
      if (data.custom && Array.isArray(data.custom)) {
        customRules.value = data.custom
      }
      
      saveToStorage()
      ElMessage.success('规则导入成功')
      return true
    } catch (error) {
      ElMessage.error('规则导入失败: ' + error.message)
      return false
    }
  }
  
  /**
   * 保存到本地存储
   */
  function saveToStorage() {
    try {
      localStorage.setItem('data-masker-rules', JSON.stringify({
        builtin: builtinRulesList.value,
        custom: customRules.value
      }))
    } catch (e) {
      console.error('保存规则失败:', e)
      if (e.name === 'QuotaExceededError') {
        ElMessage.error('存储空间不足，无法保存规则。请清理浏览器缓存。')
      } else if (e.name === 'SecurityError') {
        ElMessage.warning('本地存储被禁用，规则将不会持久化保存')
      }
    }
  }
  
  /**
   * 从本地存储加载
   */
  function loadFromStorage() {
    try {
      const saved = localStorage.getItem('data-masker-rules')
      if (saved) {
        const data = JSON.parse(saved)
        if (data.builtin && Array.isArray(data.builtin)) {
          builtinRulesList.value = data.builtin
        }
        if (data.custom && Array.isArray(data.custom)) {
          customRules.value = data.custom
        }
      }
    } catch (e) {
      console.error('加载规则失败:', e)
    }
  }
  
  // 初始化时加载
  loadFromStorage()
  
  return {
    builtinRulesList,
    customRules,
    allRules,
    enabledRules,
    ruleStats,
    toggleRule,
    updateRule,
    addCustomRule,
    deleteCustomRule,
    resetRules,
    resetBuiltinRules,
    exportRules,
    importRules
  }
})
