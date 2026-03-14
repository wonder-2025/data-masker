// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="file-select-page">
    <div class="page-header">
      <h1 class="page-title">选择文件</h1>
      <p class="page-desc">选择需要脱敏的文件，支持 PDF、Word、Excel、TXT 等多种格式</p>
    </div>
    
    <!-- 上传区域 -->
    <div class="upload-section">
      <el-upload
        ref="uploadRef"
        class="upload-dragger"
        drag
        multiple
        :auto-upload="false"
        :show-file-list="false"
        @change="handleFileChange"
      >
        <div class="upload-content">
          <el-icon class="upload-icon" :size="64"><UploadFilled /></el-icon>
          <div class="upload-text">
            <p class="upload-title">拖拽文件到此处，或<em>点击上传</em></p>
            <p class="upload-hint">支持 PDF、Word、Excel、TXT、CSV、JSON、XML 格式</p>
          </div>
        </div>
      </el-upload>
      
      <!-- 支持的格式图标 -->
      <div class="format-icons">
        <div class="format-icon" v-for="format in supportedFormats" :key="format.type">
          <el-icon :size="24" :color="format.color"><component :is="format.icon" /></el-icon>
          <span>{{ format.label }}</span>
        </div>
      </div>
    </div>
    
    <!-- 文件列表 -->
    <div class="file-list-section" v-if="fileList.length > 0">
      <div class="section-header">
        <h2 class="section-title">
          待处理文件
          <el-tag type="info" size="small" style="margin-left: 8px;">{{ fileList.length }} 个文件</el-tag>
        </h2>
        <el-button text type="danger" @click="clearAll">清空全部</el-button>
      </div>
      
      <el-table :data="fileList" style="width: 100%">
        <el-table-column prop="name" label="文件名" min-width="250">
          <template #default="{ row }">
            <div class="file-name-cell">
              <el-icon :size="24" :color="getFileIcon(row.type).color">
                <component :is="getFileIcon(row.type).icon" />
              </el-icon>
              <div class="file-info">
                <span class="file-name">{{ row.name }}</span>
                <span class="file-path">{{ row.path }}</span>
              </div>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="size" label="大小" width="100" align="center">
          <template #default="{ row }">
            {{ formatFileSize(row.size) }}
          </template>
        </el-table-column>
        
        <el-table-column prop="type" label="类型" width="100" align="center">
          <template #default="{ row }">
            <el-tag size="small">{{ getFileTypeLabel(row.type) }}</el-tag>
          </template>
        </el-table-column>
        
        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)" size="small">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        
        <el-table-column label="操作" width="120" align="center">
          <template #default="{ row }">
            <el-button size="small" text type="primary" @click="previewFile(row)">
              <el-icon><View /></el-icon>
              预览
            </el-button>
            <el-button size="small" text type="danger" @click="removeFile(row.id)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
      
      <!-- 文件统计 -->
      <div class="file-stats">
        <div class="stat-item">
          <span class="stat-label">总文件数:</span>
          <span class="stat-value">{{ fileList.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">总大小:</span>
          <span class="stat-value">{{ formatFileSize(totalSize) }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">预估时间:</span>
          <span class="stat-value">{{ estimateTime }}</span>
        </div>
      </div>
    </div>
    
    <!-- 文件预览对话框 -->
    <el-dialog
      v-model="previewVisible"
      :title="previewFileName"
      width="80%"
      top="5vh"
      destroy-on-close
    >
      <div class="preview-container">
        <div class="preview-loading" v-if="previewLoading">
          <el-icon class="is-loading" :size="32"><Loading /></el-icon>
          <p>正在加载文件内容...</p>
        </div>
        <pre class="preview-content" v-else-if="previewContent">{{ previewContent }}</pre>
        <el-empty v-else description="无法预览此文件类型" />
      </div>
    </el-dialog>
    
    <!-- 底部操作栏 -->
    <div class="action-bar" v-if="fileList.length > 0">
      <el-button @click="goBack">返回首页</el-button>
      <el-button type="primary" @click="goNext">
        下一步：配置规则
        <el-icon class="el-icon--right"><ArrowRight /></el-icon>
      </el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { ElMessage } from 'element-plus'

const router = useRouter()
const filesStore = useFilesStore()

// 文件列表
const fileList = computed(() => filesStore.files)
const totalSize = computed(() => filesStore.totalSize)

// 上传组件引用
const uploadRef = ref(null)

// 预览相关
const previewVisible = ref(false)
const previewLoading = ref(false)
const previewContent = ref('')
const previewFileName = ref('')

// 支持的格式
const supportedFormats = [
  { type: 'pdf', label: 'PDF', icon: 'Document', color: '#F56C6C' },
  { type: 'docx', label: 'Word', icon: 'Document', color: '#409EFF' },
  { type: 'xlsx', label: 'Excel', icon: 'Document', color: '#67C23A' },
  { type: 'txt', label: 'TXT', icon: 'Tickets', color: '#909399' },
  { type: 'csv', label: 'CSV', icon: 'Tickets', color: '#E6A23C' },
  { type: 'json', label: 'JSON', icon: 'Tickets', color: '#9C27B0' }
]

// 计算预估处理时间
const estimateTime = computed(() => {
  const mb = totalSize.value / (1024 * 1024)
  const minutes = Math.ceil(mb * 0.5) // 假设每MB需要0.5分钟
  if (minutes < 1) return '小于1分钟'
  if (minutes >= 60) return `${Math.floor(minutes / 60)}小时${minutes % 60}分钟`
  return `约${minutes}分钟`
})

// 处理文件选择
function handleFileChange(file) {
  if (!file || !file.raw) return
  
  const rawFile = file.raw
  const ext = getFileExtension(rawFile.name)
  
  // 检查文件格式
  if (!isSupportedFormat(ext)) {
    ElMessage.warning(getUnsupportedFormatTip(ext))
    return
  }
  
  // 检查文件大小（默认最大100MB）
  const maxSize = 100 * 1024 * 1024
  if (rawFile.size > maxSize) {
    ElMessage.warning('文件大小超过100MB限制')
    return
  }
  
  // 添加到文件列表
  filesStore.addFile({
    name: rawFile.name,
    path: rawFile.path || rawFile.name,
    size: rawFile.size,
    type: ext
  })
}

// 获取文件扩展名
function getFileExtension(filename) {
  const ext = filename.split('.').pop()?.toLowerCase()
  return ext || 'unknown'
}

// 检查是否支持的格式
function isSupportedFormat(ext) {
  const supported = ['pdf', 'docx', 'xlsx', 'xls', 'txt', 'md', 'csv', 'json', 'xml', 'pptx']
  return supported.includes(ext)
}

// 获取不支持的格式的提示信息
function getUnsupportedFormatTip(ext) {
  const tips = {
    'doc': '旧版 Word 文档 (.doc) 暂不支持，请转换为 .docx 格式后再上传',
    'ppt': '旧版 PPT 文档 (.ppt) 暂不支持，请转换为 .pptx 格式后再上传',
    'docm': '带宏的 Word 文档 (.docm) 暂不支持',
    'xlsm': '带宏的 Excel 文档 (.xlsm) 暂不支持'
  }
  return tips[ext] || `不支持的文件格式: .${ext}`
}

// 获取文件图标
function getFileIcon(type) {
  const icons = {
    pdf: { icon: 'Document', color: '#F56C6C' },
    docx: { icon: 'Document', color: '#409EFF' },
    xlsx: { icon: 'Document', color: '#67C23A' },
    xls: { icon: 'Document', color: '#67C23A' },
    txt: { icon: 'Tickets', color: '#909399' },
    csv: { icon: 'Tickets', color: '#E6A23C' },
    json: { icon: 'Tickets', color: '#9C27B0' },
    xml: { icon: 'Tickets', color: '#9C27B0' }
  }
  return icons[type] || { icon: 'Document', color: '#909399' }
}

// 获取文件类型标签
function getFileTypeLabel(type) {
  const labels = {
    pdf: 'PDF',
    docx: 'Word',
    xlsx: 'Excel',
    xls: 'Excel',
    txt: '文本',
    csv: 'CSV',
    json: 'JSON',
    xml: 'XML',
    pptx: 'PPT'
  }
  return labels[type] || type?.toUpperCase() || '未知'
}

// 获取状态类型
function getStatusType(status) {
  const types = {
    pending: 'info',
    processing: 'warning',
    done: 'success',
    error: 'danger'
  }
  return types[status] || 'info'
}

// 获取状态标签
function getStatusLabel(status) {
  const labels = {
    pending: '待处理',
    processing: '处理中',
    done: '已完成',
    error: '错误'
  }
  return labels[status] || status
}

// 格式化文件大小
function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 移除文件
function removeFile(fileId) {
  filesStore.removeFile(fileId)
}

// 清空全部
function clearAll() {
  filesStore.clearFiles()
}

// 预览文件
async function previewFile(file) {
  previewVisible.value = true
  previewFileName.value = file.name
  previewLoading.value = true
  previewContent.value = ''
  
  try {
    // 调用 Tauri 命令读取文件内容
    const { invoke } = await import('@tauri-apps/api/core')
    const content = await invoke('read_file_preview', { path: file.path })
    previewContent.value = content
  } catch (error) {
    console.error('预览文件失败:', error)
    previewContent.value = `无法读取文件: ${error.message || error}`
  } finally {
    previewLoading.value = false
  }
}

// 返回首页
function goBack() {
  router.push('/')
}

// 下一步
function goNext() {
  if (fileList.value.length === 0) {
    ElMessage.warning('请先选择要处理的文件')
    return
  }
  router.push('/rule-config')
}
</script>

<style lang="scss" scoped>
.file-select-page {
  max-width: 1000px;
  margin: 0 auto;
}

.upload-section {
  margin-bottom: 32px;
  
  .upload-dragger {
    width: 100%;
    
    :deep(.el-upload-dragger) {
      border: 2px dashed #dcdfe6;
      border-radius: 12px;
      background: #fafafa;
      transition: all 0.3s;
      
      &:hover {
        border-color: #409EFF;
        background: #f0f7ff;
      }
    }
  }
  
  .upload-content {
    padding: 48px;
    text-align: center;
    
    .upload-icon {
      color: #c0c4cc;
      margin-bottom: 16px;
    }
    
    .upload-text {
      .upload-title {
        font-size: 16px;
        color: #606266;
        margin: 0 0 8px;
        
        em {
          color: #409EFF;
          font-style: normal;
        }
      }
      
      .upload-hint {
        font-size: 14px;
        color: #909399;
        margin: 0;
      }
    }
  }
  
  .format-icons {
    display: flex;
    justify-content: center;
    gap: 32px;
    margin-top: 24px;
    
    .format-icon {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 4px;
      
      span {
        font-size: 12px;
        color: #909399;
      }
    }
  }
}

.file-list-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  
  .section-title {
    font-size: 18px;
    font-weight: 600;
    color: #303133;
  }
  
  .file-name-cell {
    display: flex;
    align-items: center;
    gap: 12px;
    
    .file-info {
      display: flex;
      flex-direction: column;
      
      .file-name {
        font-weight: 500;
        color: #303133;
      }
      
      .file-path {
        font-size: 12px;
        color: #909399;
        max-width: 300px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }
  }
  
  .file-stats {
    display: flex;
    justify-content: flex-end;
    gap: 24px;
    margin-top: 16px;
    padding-top: 16px;
    border-top: 1px solid #ebeef5;
    
    .stat-item {
      display: flex;
      align-items: center;
      gap: 8px;
      
      .stat-label {
        color: #909399;
        font-size: 14px;
      }
      
      .stat-value {
        font-weight: 600;
        color: #303133;
      }
    }
  }
}

.preview-container {
  min-height: 300px;
  max-height: 60vh;
  overflow-y: auto;
  
  .preview-loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 300px;
    color: #909399;
  }
  
  .preview-content {
    background: #f5f7fa;
    padding: 16px;
    border-radius: 8px;
    font-size: 13px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-all;
    margin: 0;
  }
}

.action-bar {
  display: flex;
  justify-content: space-between;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #ebeef5;
}
</style>
