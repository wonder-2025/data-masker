// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 文件管理 Store
 * 管理待处理的文件列表和文件状态
 */
export const useFilesStore = defineStore('files', () => {
  // 文件列表
  const files = ref([])
  
  // 最近处理的文件
  const recentFiles = ref([])
  
  // 总大小（字节）
  const totalSize = computed(() => {
    return files.value.reduce((sum, file) => sum + file.size, 0)
  })
  
  // 文件数量
  const fileCount = computed(() => files.value.length)
  
  /**
   * 添加文件
   * @param {Object} file - 文件对象
   */
  function addFile(file) {
    // 检查是否已存在
    if (!files.value.some(f => f.path === file.path)) {
      files.value.push({
        id: file.id || Date.now().toString(),
        name: file.name,
        path: file.path,
        size: file.size,
        type: file.type,
        status: 'pending',
        sensitiveCount: 0,
        processingTime: '',
        addedAt: file.addedAt || new Date().toISOString()
      })
    }
  }
  
  /**
   * 批量添加文件
   * @param {Array} fileList - 文件列表
   */
  function addFiles(fileList) {
    fileList.forEach(file => addFile(file))
  }
  
  /**
   * 移除文件
   * @param {string} fileId - 文件ID
   */
  function removeFile(fileId) {
    const index = files.value.findIndex(f => f.id === fileId)
    if (index !== -1) {
      files.value.splice(index, 1)
    }
  }
  
  /**
   * 清空所有文件
   */
  function clearFiles() {
    files.value = []
  }
  
  /**
   * 更新文件状态
   * @param {string} fileId - 文件ID
   * @param {string} status - 新状态
   */
  function updateFileStatus(fileId, status) {
    const file = files.value.find(f => f.id === fileId)
    if (file) {
      file.status = status
    }
  }
  
  /**
   * 更新文件处理结果
   * @param {string} fileId - 文件ID
   * @param {Object} result - 处理结果
   */
  function updateFileResult(fileId, result) {
    const file = files.value.find(f => f.id === fileId)
    if (file) {
      file.status = result.status
      file.sensitiveCount = result.sensitiveCount
      file.processingTime = result.processingTime
      file.outputPath = result.outputPath
    }
  }
  
  /**
   * 添加最近处理文件
   * @param {Object} file - 文件信息
   */
  function addRecentFile(file) {
    // 移除重复项
    recentFiles.value = recentFiles.value.filter(f => f.path !== file.path)
    
    // 添加到开头
    recentFiles.value.unshift({
      name: file.name,
      path: file.path,
      type: file.type,
      processedAt: new Date().toISOString()
    })
    
    // 保留最近20条
    if (recentFiles.value.length > 20) {
      recentFiles.value = recentFiles.value.slice(0, 20)
    }
    
    // 保存到本地存储
    saveRecentToStorage()
  }
  
  /**
   * 清空最近处理文件
   */
  function clearRecentFiles() {
    recentFiles.value = []
    localStorage.removeItem('data-masker-recent-files')
  }
  
  /**
   * 保存最近文件到本地存储
   */
  function saveRecentToStorage() {
    try {
      localStorage.setItem('data-masker-recent-files', JSON.stringify(recentFiles.value))
    } catch (e) {
      console.error('保存最近文件失败:', e)
    }
  }
  
  /**
   * 从本地存储加载最近文件
   */
  function loadRecentFromStorage() {
    try {
      const saved = localStorage.getItem('data-masker-recent-files')
      if (saved) {
        recentFiles.value = JSON.parse(saved)
      }
    } catch (e) {
      console.error('加载最近文件失败:', e)
    }
  }
  
  // 初始化时加载最近文件
  loadRecentFromStorage()
  
  return {
    files,
    recentFiles,
    totalSize,
    fileCount,
    addFile,
    addFiles,
    removeFile,
    clearFiles,
    updateFileStatus,
    updateFileResult,
    addRecentFile,
    clearRecentFiles
  }
})
