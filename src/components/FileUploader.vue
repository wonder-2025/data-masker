// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'

/**
 * 文件上传组件
 * 支持拖拽上传、点击选择、多文件上传
 */
export default {
  name: 'FileUploader',
  
  props: {
    // 是否支持多文件
    multiple: {
      type: Boolean,
      default: true
    },
    // 最大文件大小（MB）
    maxSize: {
      type: Number,
      default: 100
    },
    // 支持的文件类型
    accept: {
      type: Array,
      default: () => ['.pdf', '.docx', '.xlsx', '.txt', '.csv', '.json', '.xml']
    },
    // 最大文件数量
    maxCount: {
      type: Number,
      default: 20
    }
  },
  
  emits: ['change', 'success', 'error'],
  
  setup(props, { emit }) {
    const isDragging = ref(false)
    const files = ref([])
    const inputRef = ref(null)
    
    // 文件数量
    const fileCount = computed(() => files.value.length)
    
    // 总大小
    const totalSize = computed(() => {
      return files.value.reduce((sum, file) => sum + file.size, 0)
    })
    
    /**
     * 处理拖拽进入
     */
    function handleDragEnter(e) {
      e.preventDefault()
      isDragging.value = true
    }
    
    /**
     * 处理拖拽离开
     */
    function handleDragLeave(e) {
      e.preventDefault()
      isDragging.value = false
    }
    
    /**
     * 处理拖拽悬停
     */
    function handleDragOver(e) {
      e.preventDefault()
    }
    
    /**
     * 处理文件放下
     */
    function handleDrop(e) {
      e.preventDefault()
      isDragging.value = false
      
      const droppedFiles = Array.from(e.dataTransfer.files)
      handleFiles(droppedFiles)
    }
    
    /**
     * 处理点击选择文件
     */
    function handleInputChange(e) {
      const selectedFiles = Array.from(e.target.files)
      handleFiles(selectedFiles)
      
      // 清空input，允许重复选择相同文件
      e.target.value = ''
    }
    
    /**
     * 处理文件
     */
    function handleFiles(fileList) {
      const validFiles = []
      const maxSizeBytes = props.maxSize * 1024 * 1024
      
      for (const file of fileList) {
        // 检查文件数量限制
        if (files.value.length + validFiles.length >= props.maxCount) {
          ElMessage.warning(`最多只能上传 ${props.maxCount} 个文件`)
          break
        }
        
        // 检查文件类型
        const ext = '.' + file.name.split('.').pop().toLowerCase()
        if (!props.accept.includes(ext)) {
          ElMessage.warning(`不支持的文件类型: ${ext}`)
          continue
        }
        
        // 检查文件大小
        if (file.size > maxSizeBytes) {
          ElMessage.warning(`文件 ${file.name} 超过 ${props.maxSize}MB 限制`)
          continue
        }
        
        // 检查是否已存在
        if (files.value.some(f => f.name === file.name && f.size === file.size)) {
          ElMessage.warning(`文件 ${file.name} 已存在`)
          continue
        }
        
        validFiles.push({
          id: Date.now() + Math.random().toString(36).substr(2, 9),
          name: file.name,
          path: file.path || file.name,
          size: file.size,
          type: ext.slice(1),
          file: file,
          status: 'pending'
        })
      }
      
      if (validFiles.length > 0) {
        files.value.push(...validFiles)
        emit('change', files.value)
        emit('success', validFiles)
      }
    }
    
    /**
     * 移除文件
     */
    function removeFile(fileId) {
      const index = files.value.findIndex(f => f.id === fileId)
      if (index !== -1) {
        files.value.splice(index, 1)
        emit('change', files.value)
      }
    }
    
    /**
     * 清空所有文件
     */
    function clearFiles() {
      files.value = []
      emit('change', files.value)
    }
    
    /**
     * 触发文件选择
     */
    function triggerSelect() {
      inputRef.value?.click()
    }
    
    /**
     * 格式化文件大小
     */
    function formatSize(bytes) {
      if (bytes === 0) return '0 B'
      const k = 1024
      const sizes = ['B', 'KB', 'MB', 'GB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    }
    
    return {
      isDragging,
      files,
      fileCount,
      totalSize,
      inputRef,
      handleDragEnter,
      handleDragLeave,
      handleDragOver,
      handleDrop,
      handleInputChange,
      removeFile,
      clearFiles,
      triggerSelect,
      formatSize
    }
  }
}
