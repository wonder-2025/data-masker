// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

/**
 * 文件状态管理 Store
 * 管理待处理的文件列表和文件相关信息
 */
export const useFilesStore = defineStore('files', () => {
  // 待处理文件列表
  const files = ref([])
  
  // 当前选中的文件
  const selectedFile = ref(null)
  
  // 文件预览内容
  const previewContent = ref(null)
  
  // 最近处理的文件
  const recentFiles = ref([])

  // 计算属性：总文件大小
  const totalSize = computed(() => {
    return files.value.reduce((sum, file) => sum + (file.size || 0), 0)
  })

  // 计算属性：文件数量
  const fileCount = computed(() => files.value.length)

  /**
   * 添加文件到列表
   * @param {Object} file - 文件信息对象
   */
  function addFile(file) {
    const exists = files.value.some(f => f.path === file.path)
    if (!exists) {
      files.value.push({
        id: Date.now().toString(),
        name: file.name,
        path: file.path,
        size: file.size,
        type: file.type,
        status: 'pending', // pending, processing, done, error
        addedAt: new Date().toISOString()
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
    selectedFile.value = null
    previewContent.value = null
    // 清除文件标记（路由守卫需要）
    sessionStorage.removeItem('hasFiles')
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
   * 设置文件预览内容
   * @param {Object} content - 预览内容
   */
  function setPreviewContent(content) {
    previewContent.value = content
  }

  /**
   * 添加到最近文件列表
   * @param {Object} file - 文件信息
   */
  function addRecentFile(file) {
    // 移除重复
    recentFiles.value = recentFiles.value.filter(f => f.path !== file.path)
    // 添加到开头
    recentFiles.value.unshift({
      ...file,
      processedAt: new Date().toISOString()
    })
    // 保留最近10个
    if (recentFiles.value.length > 10) {
      recentFiles.value = recentFiles.value.slice(0, 10)
    }
  }

  return {
    files,
    selectedFile,
    previewContent,
    recentFiles,
    totalSize,
    fileCount,
    addFile,
    addFiles,
    removeFile,
    clearFiles,
    updateFileStatus,
    setPreviewContent,
    addRecentFile
  }
})
