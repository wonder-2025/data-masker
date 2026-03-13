// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 脱敏规则类型枚举
 */
export const RuleType = {
  ID_CARD: 'id_card',           // 身份证号
  PHONE: 'phone',               // 手机号
  BANK_CARD: 'bank_card',       // 银行卡号
  PASSPORT: 'passport',         // 护照号
  CREDIT_CODE: 'credit_code',   // 统一社会信用代码
  EMAIL: 'email',               // 邮箱
  LICENSE_PLATE: 'license_plate', // 车牌号
  IPV4: 'ipv4',                 // IPv4地址
  IPV6: 'ipv6',                 // IPv6地址
  MAC: 'mac',                   // MAC地址
  API_KEY: 'api_key',           // JSON密钥
  NAME: 'name',                 // 姓名
  COMPANY: 'company',           // 公司全称
  ADDRESS: 'address',           // 地址
  AMOUNT: 'amount',             // 金额
  DATE: 'date',                 // 日期
  URL: 'url',                   // URL
  TELEPHONE: 'telephone',       // 电话号码
  CUSTOM: 'custom'              // 自定义规则
}

/**
 * 脱敏策略类型枚举
 */
export const StrategyType = {
  FULL_MASK: 'full_mask',       // 完全隐藏
  PARTIAL_MASK: 'partial_mask', // 部分掩码
  FAKE_DATA: 'fake_data',       // 假数据替换
  REVERSIBLE: 'reversible',     // 可逆加密
  HASH: 'hash',                 // 哈希脱敏
  CUSTOM: 'custom'              // 自定义替换
}

/**
 * 内置规则定义
 */
const builtinRules = [
  {
    id: 'rule_id_card',
    name: '身份证号',
    type: RuleType.ID_CARD,
    pattern: '\\d{17}[\\dXx]',
    description: '18位身份证号码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 6, keepEnd: 4 }
  },
  {
    id: 'rule_phone',
    name: '手机号',
    type: RuleType.PHONE,
    pattern: '1[3-9]\\d{9}',
    description: '中国大陆手机号码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 3, keepEnd: 4 }
  },
  {
    id: 'rule_bank_card',
    name: '银行卡号',
    type: RuleType.BANK_CARD,
    pattern: '\\d{16,19}',
    description: '银行卡号（带Luhn校验）',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 4, keepEnd: 4 },
    needLuhnCheck: true
  },
  {
    id: 'rule_passport',
    name: '护照号',
    type: RuleType.PASSPORT,
    pattern: '[A-Z]\\d{8}',
    description: '中国护照号码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 1, keepEnd: 3 }
  },
  {
    id: 'rule_credit_code',
    name: '统一社会信用代码',
    type: RuleType.CREDIT_CODE,
    pattern: '[0-9A-Z]{18}',
    description: '18位统一社会信用代码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 6, keepEnd: 4 }
  },
  {
    id: 'rule_email',
    name: '邮箱',
    type: RuleType.EMAIL,
    pattern: '[\\w.-]+@[\\w.-]+\\.\\w+',
    description: '电子邮箱地址',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 2, keepEnd: 0 }
  },
  {
    id: 'rule_license_plate',
    name: '车牌号',
    type: RuleType.LICENSE_PLATE,
    pattern: '[京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼][A-Z][A-Z0-9]{5,6}',
    description: '中国车牌号码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 1, keepEnd: 2 }
  },
  {
    id: 'rule_ipv4',
    name: 'IPv4地址',
    type: RuleType.IPV4,
    pattern: '\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}',
    description: 'IPv4网络地址',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 0, keepEnd: 0 }
  },
  {
    id: 'rule_ipv6',
    name: 'IPv6地址',
    type: RuleType.IPV6,
    pattern: '([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}',
    description: 'IPv6网络地址',
    enabled: true,
    strategy: StrategyType.FULL_MASK
  },
  {
    id: 'rule_mac',
    name: 'MAC地址',
    type: RuleType.MAC,
    pattern: '([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}',
    description: '设备MAC地址',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 0, keepEnd: 2 }
  },
  {
    id: 'rule_api_key',
    name: 'JSON密钥',
    type: RuleType.API_KEY,
    pattern: '"(api[_-]?key|token|secret|password)":\\s*"[^"]+"',
    description: 'API密钥、Token等敏感配置',
    enabled: true,
    strategy: StrategyType.FULL_MASK
  },
  {
    id: 'rule_name',
    name: '姓名',
    type: RuleType.NAME,
    pattern: '',
    description: '中文姓名（需NER支持）',
    enabled: false, // 默认关闭，需要NER模型
    strategy: StrategyType.FAKE_DATA
  },
  {
    id: 'rule_company',
    name: '公司全称',
    type: RuleType.COMPANY,
    pattern: '',
    description: '公司名称（需NER支持）',
    enabled: false,
    strategy: StrategyType.FAKE_DATA
  },
  {
    id: 'rule_address',
    name: '地址',
    type: RuleType.ADDRESS,
    pattern: '',
    description: '详细地址信息',
    enabled: false,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 6, keepEnd: 0 }
  },
  {
    id: 'rule_amount',
    name: '金额',
    type: RuleType.AMOUNT,
    pattern: '[\\d,]+\\.?\\d*\\s*(元|万元|亿元|¥|\\$)',
    description: '金额数值',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 0, keepEnd: 1 }
  },
  {
    id: 'rule_date',
    name: '日期',
    type: RuleType.DATE,
    pattern: '\\d{4}[-/年]\\d{1,2}[-/月]\\d{1,2}日?',
    description: '日期格式',
    enabled: false, // 日期通常不需要脱敏
    strategy: StrategyType.PARTIAL_MASK
  },
  {
    id: 'rule_url',
    name: 'URL',
    type: RuleType.URL,
    pattern: 'https?://[^\\s]+',
    description: '网页链接',
    enabled: false,
    strategy: StrategyType.PARTIAL_MASK
  },
  {
    id: 'rule_telephone',
    name: '电话号码',
    type: RuleType.TELEPHONE,
    pattern: '0\\d{2,3}-?\\d{7,8}',
    description: '固定电话号码',
    enabled: true,
    strategy: StrategyType.PARTIAL_MASK,
    strategyConfig: { keepStart: 3, keepEnd: 3 }
  }
]

/**
 * 规则状态管理 Store
 * 管理内置规则和自定义规则
 */
export const useRulesStore = defineStore('rules', () => {
  // 内置规则列表
  const builtinRulesList = ref([...builtinRules])
  
  // 自定义规则列表
  const customRules = ref([])
  
  // 规则优先级顺序
  const rulePriority = ref([])

  // 计算属性：所有启用的规则
  const enabledRules = computed(() => {
    return [
      ...builtinRulesList.value.filter(r => r.enabled),
      ...customRules.value.filter(r => r.enabled)
    ].sort((a, b) => {
      const aIndex = rulePriority.value.indexOf(a.id)
      const bIndex = rulePriority.value.indexOf(b.id)
      if (aIndex === -1 && bIndex === -1) return 0
      if (aIndex === -1) return 1
      if (bIndex === -1) return -1
      return aIndex - bIndex
    })
  })

  // 计算属性：规则统计
  const ruleStats = computed(() => {
    const builtin = builtinRulesList.value.filter(r => r.enabled).length
    const custom = customRules.value.filter(r => r.enabled).length
    return {
      builtinEnabled: builtin,
      builtinTotal: builtinRulesList.value.length,
      customEnabled: custom,
      customTotal: customRules.value.length,
      totalEnabled: builtin + custom
    }
  })

  /**
   * 切换规则启用状态
   * @param {string} ruleId - 规则ID
   */
  function toggleRule(ruleId) {
    const builtinRule = builtinRulesList.value.find(r => r.id === ruleId)
    if (builtinRule) {
      builtinRule.enabled = !builtinRule.enabled
      return
    }
    
    const customRule = customRules.value.find(r => r.id === ruleId)
    if (customRule) {
      customRule.enabled = !customRule.enabled
    }
  }

  /**
   * 更新规则策略
   * @param {string} ruleId - 规则ID
   * @param {string} strategy - 策略类型
   * @param {Object} config - 策略配置
   */
  function updateRuleStrategy(ruleId, strategy, config = {}) {
    const rule = builtinRulesList.value.find(r => r.id === ruleId) ||
                 customRules.value.find(r => r.id === ruleId)
    if (rule) {
      rule.strategy = strategy
      rule.strategyConfig = config
    }
  }

  /**
   * 添加自定义规则
   * @param {Object} rule - 规则对象
   */
  function addCustomRule(rule) {
    const newRule = {
      id: `custom_${Date.now()}`,
      name: rule.name,
      type: RuleType.CUSTOM,
      pattern: rule.pattern,
      description: rule.description || '',
      enabled: true,
      strategy: rule.strategy || StrategyType.FULL_MASK,
      strategyConfig: rule.strategyConfig || {},
      createdAt: new Date().toISOString()
    }
    customRules.value.push(newRule)
    return newRule
  }

  /**
   * 更新自定义规则
   * @param {string} ruleId - 规则ID
   * @param {Object} updates - 更新内容
   */
  function updateCustomRule(ruleId, updates) {
    const index = customRules.value.findIndex(r => r.id === ruleId)
    if (index !== -1) {
      customRules.value[index] = {
        ...customRules.value[index],
        ...updates,
        updatedAt: new Date().toISOString()
      }
    }
  }

  /**
   * 删除自定义规则
   * @param {string} ruleId - 规则ID
   */
  function deleteCustomRule(ruleId) {
    const index = customRules.value.findIndex(r => r.id === ruleId)
    if (index !== -1) {
      customRules.value.splice(index, 1)
    }
  }

  /**
   * 导入规则
   * @param {Array} rules - 规则列表
   */
  function importRules(rules) {
    rules.forEach(rule => {
      if (rule.id && rule.name && rule.pattern) {
        // 检查是否已存在
        const exists = customRules.value.some(r => 
          r.name === rule.name || r.pattern === rule.pattern
        )
        if (!exists) {
          addCustomRule(rule)
        }
      }
    })
  }

  /**
   * 导出规则
   * @returns {string} JSON字符串
   */
  function exportRules() {
    return JSON.stringify(customRules.value, null, 2)
  }

  /**
   * 重置为默认规则
   */
  function resetToDefault() {
    builtinRulesList.value = [...builtinRules]
    customRules.value = []
    rulePriority.value = []
  }

  return {
    builtinRulesList,
    customRules,
    rulePriority,
    enabledRules,
    ruleStats,
    toggleRule,
    updateRuleStrategy,
    addCustomRule,
    updateCustomRule,
    deleteCustomRule,
    importRules,
    exportRules,
    resetToDefault
  }
})
