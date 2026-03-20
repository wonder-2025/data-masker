// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 处理结果状态管理 Store
 * 管理脱敏处理的结果和统计数据
 */
export const useResultStore = defineStore('result', () => {
  // 处理结果列表
  const results = ref([])
  
  // 当前处理进度
  const progress = ref({
    current: 0,
    total: 0,
    currentFile: '',
    status: 'idle' // idle, processing, done, error
  })
  
  // 处理日志
  const logs = ref([])
  
  // 映射表（用于可逆脱敏）
  const mappingTable = ref({})

  // 计算属性：处理成功数量
  const successCount = computed(() => {
    return results.value.filter(r => r.status === 'done').length
  })

  // 计算属性：处理失败数量
  const errorCount = computed(() => {
    return results.value.filter(r => r.status === 'error').length
  })

  // 计算属性：敏感信息统计
  const sensitiveStats = computed(() => {
    const stats = {}
    results.value.forEach(result => {
      if (result.sensitiveInfo) {
        result.sensitiveInfo.forEach(info => {
          if (!stats[info.type]) {
            stats[info.type] = 0
          }
          stats[info.type]++
        })
      }
    })
    return stats
  })

  // 计算属性：总敏感信息数量
  const totalSensitive = computed(() => {
    return Object.values(sensitiveStats.value).reduce((sum, count) => sum + count, 0)
  })

  /**
   * 添加处理结果
   * @param {Object} result - 处理结果对象
   */
  function addResult(result) {
    console.log('[DEBUG] ========== resultStore.addResult ==========')
    console.log('[DEBUG] 接收到的 result:', JSON.stringify(result, null, 2))
    console.log('[DEBUG] result.sensitiveInfo:', result.sensitiveInfo)
    console.log('[DEBUG] result.sensitiveInfo 类型:', typeof result.sensitiveInfo)
    console.log('[DEBUG] result.sensitiveInfo 长度:', result.sensitiveInfo?.length)
    
    const newResult = {
      id: Date.now().toString(),
      fileId: result.fileId,
      fileName: result.fileName,
      status: result.status,
      sensitiveInfo: result.sensitiveInfo || [],
      maskedContent: result.maskedContent,
      outputPath: result.outputPath,
      processingTime: result.processingTime,
      createdAt: new Date().toISOString()
    }
    
    console.log('[DEBUG] 构造的 newResult:', JSON.stringify(newResult, null, 2))
    
    results.value.push(newResult)
    
    console.log('[DEBUG] 添加后 results.value 长度:', results.value.length)
    console.log('[DEBUG] 添加后 results.value 最后一条:', results.value[results.value.length - 1])
  }

  /**
   * 更新处理进度
   * @param {Object} progressData - 进度数据
   */
  function updateProgress(progressData) {
    progress.value = {
      ...progress.value,
      ...progressData
    }
  }

  /**
   * 添加日志
   * @param {string} level - 日志级别
   * @param {string} message - 日志消息
   * @param {Object} data - 附加数据
   */
  function addLog(level, message, data = {}) {
    logs.value.push({
      id: Date.now().toString(),
      level, // info, warn, error, success
      message,
      data,
      timestamp: new Date().toISOString()
    })
    
    // 保留最近1000条日志
    if (logs.value.length > 1000) {
      logs.value = logs.value.slice(-1000)
    }
  }

  /**
   * 设置映射表
   * @param {Object} mapping - 映射对象
   */
  function setMappingTable(mapping) {
    mappingTable.value = mapping
  }

  /**
   * 添加映射项
   * @param {string} original - 原始值
   * @param {string} masked - 脱敏值
   */
  function addMapping(original, masked) {
    mappingTable.value[original] = masked
  }

  /**
   * 清空结果
   */
  function clearResults() {
    results.value = []
    progress.value = {
      current: 0,
      total: 0,
      currentFile: '',
      status: 'idle'
    }
  }

  /**
   * 清空日志
   */
  function clearLogs() {
    logs.value = []
  }

  /**
   * 清空映射表
   */
  function clearMapping() {
    mappingTable.value = {}
  }

  /**
   * 导出映射表
   * @returns {string} JSON字符串
   */
  function exportMapping() {
    return JSON.stringify(mappingTable.value, null, 2)
  }

  /**
   * 获取处理报告数据
   * @returns {Object} 报告数据
   */
  function getReportData() {
    return {
      summary: {
        totalFiles: results.value.length,
        successCount: successCount.value,
        errorCount: errorCount.value,
        totalSensitive: totalSensitive.value
      },
      sensitiveStats: sensitiveStats.value,
      results: results.value.map(r => ({
        fileName: r.fileName,
        status: r.status,
        sensitiveCount: r.sensitiveInfo.length,
        processingTime: r.processingTime
      })),
      generatedAt: new Date().toISOString()
    }
  }

  return {
    results,
    progress,
    logs,
    mappingTable,
    successCount,
    errorCount,
    sensitiveStats,
    totalSensitive,
    addResult,
    updateProgress,
    addLog,
    setMappingTable,
    addMapping,
    clearResults,
    clearLogs,
    clearMapping,
    exportMapping,
    getReportData
  }
})
