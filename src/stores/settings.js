import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useSettingsStore = defineStore('settings', () => {
  // 文件保存路径
  const savePath = ref('')
  
  // 脱敏级别：low, medium, high
  const maskingLevel = ref('medium')
  
  // 默认脱敏规则
  const defaultRules = ref([
    { id: 'id_card', name: '身份证号', enabled: true },
    { id: 'phone', name: '手机号', enabled: true },
    { id: 'bank_card', name: '银行卡号', enabled: true },
    { id: 'email', name: '邮箱', enabled: true },
    { id: 'name', name: '姓名', enabled: true },
    { id: 'address', name: '地址', enabled: true }
  ])
  
  // 错误日志提交
  const errorReport = ref({
    enabled: true,
    serverUrl: 'http://106.12.190.227:30051/api/error-log'
  })
  
  // 测试服务器连接
  const testConnection = async () => {
    try {
      const response = await fetch(errorReport.value.serverUrl.replace('/api/error-log', '/api/ping'), {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' }
      })
      if (response.ok) {
        const data = await response.json()
        return { success: true, message: data.message || '连接成功' }
      }
      return { success: false, message: `HTTP ${response.status}` }
    } catch (error) {
      return { success: false, message: error.message }
    }
  }
  
  // 保存设置到本地存储
  watch([savePath, maskingLevel, defaultRules, errorReport], () => {
    localStorage.setItem('data-masker-settings', JSON.stringify({
      savePath: savePath.value,
      maskingLevel: maskingLevel.value,
      defaultRules: defaultRules.value,
      errorReport: errorReport.value
    }))
  }, { deep: true })
  
  // 从本地存储加载设置
  const loadSettings = () => {
    const saved = localStorage.getItem('data-masker-settings')
    if (saved) {
      const settings = JSON.parse(saved)
      savePath.value = settings.savePath || ''
      maskingLevel.value = settings.maskingLevel || 'medium'
      defaultRules.value = settings.defaultRules || defaultRules.value
      errorReport.value = settings.errorReport || errorReport.value
    }
  }
  
  return {
    savePath,
    maskingLevel,
    defaultRules,
    errorReport,
    testConnection,
    loadSettings
  }
})
