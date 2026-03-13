// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="preview-page">
    <div class="page-header">
      <h1 class="page-title">预览确认</h1>
      <p class="page-desc">查看脱敏效果，确认后开始处理</p>
    </div>
    
    <!-- 文件选择器 -->
    <div class="file-selector" v-if="fileList.length > 1">
      <el-select v-model="selectedFileId" placeholder="选择要预览的文件" style="width: 300px;">
        <el-option
          v-for="file in fileList"
          :key="file.id"
          :label="file.name"
          :value="file.id"
        />
      </el-select>
      <el-button @click="refreshPreview" :loading="previewLoading">
        <el-icon><Refresh /></el-icon>
        刷新预览
      </el-button>
    </div>
    
    <!-- 预览区域 -->
    <div class="preview-container" v-loading="previewLoading">
      <div class="preview-header">
        <div class="file-info">
          <el-icon :size="24"><Document /></el-icon>
          <span>{{ selectedFile?.name || '未选择文件' }}</span>
        </div>
        <div class="sensitive-count" v-if="sensitiveInfo.length > 0">
          <el-tag type="warning">
            发现 {{ sensitiveInfo.length }} 处敏感信息
          </el-tag>
        </div>
      </div>
      
      <!-- 差异对比视图 -->
      <div class="diff-view">
        <div class="diff-pane original">
          <div class="pane-header">
            <span>原始内容</span>
          </div>
          <div class="pane-content" v-html="originalHtml"></div>
        </div>
        
        <div class="diff-pane masked">
          <div class="pane-header">
            <span>脱敏后内容</span>
          </div>
          <div class="pane-content" v-html="maskedHtml"></div>
        </div>
      </div>
      
      <!-- 敏感信息列表 -->
      <div class="sensitive-list" v-if="sensitiveInfo.length > 0">
        <h3>敏感信息详情</h3>
        <el-table :data="sensitiveInfo" style="width: 100%" max-height="300">
          <el-table-column type="index" label="#" width="50" />
          <el-table-column prop="type" label="类型" width="120">
            <template #default="{ row }">
              <el-tag size="small">{{ getTypeLabel(row.type) }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="original" label="原始值" min-width="150">
            <template #default="{ row }">
              <span class="sensitive-value">{{ row.original }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="masked" label="脱敏后" min-width="150">
            <template #default="{ row }">
              <span class="masked-value">{{ row.masked }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="position" label="位置" width="120">
            <template #default="{ row }">
              行 {{ row.line }}, 列 {{ row.column }}
            </template>
          </el-table-column>
          <el-table-column label="操作" width="100" align="center">
            <template #default="{ row }">
              <el-button size="small" text type="primary" @click="editSensitive(row)">
                修改
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
      
      <!-- 空状态 -->
      <el-empty v-if="!previewLoading && sensitiveInfo.length === 0" description="未发现敏感信息">
        <template #image>
          <el-icon :size="64" color="#67C23A"><CircleCheckFilled /></el-icon>
        </template>
      </el-empty>
    </div>
    
    <!-- 统计面板 -->
    <div class="stats-panel" v-if="sensitiveInfo.length > 0">
      <h3>脱敏统计</h3>
      <div class="stats-grid">
        <div class="stat-item" v-for="(count, type) in typeStats" :key="type">
          <div class="stat-label">{{ getTypeLabel(type) }}</div>
          <div class="stat-value">{{ count }} 处</div>
        </div>
      </div>
    </div>
    
    <!-- 底部操作栏 -->
    <div class="action-bar">
      <el-button @click="goBack">
        <el-icon class="el-icon--left"><ArrowLeft /></el-icon>
        上一步
      </el-button>
      <el-button type="primary" @click="startProcessing" :disabled="sensitiveInfo.length === 0">
        <el-icon class="el-icon--right"><VideoPlay /></el-icon>
        开始脱敏
      </el-button>
    </div>
    
    <!-- 编辑敏感信息对话框 -->
    <el-dialog v-model="editDialogVisible" title="修改脱敏结果" width="400px">
      <el-form label-width="80px">
        <el-form-item label="原始值">
          <el-input :value="editingItem?.original" disabled />
        </el-form-item>
        <el-form-item label="脱敏值">
          <el-input v-model="editMaskedValue" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveEdit">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useRulesStore } from '@/stores/rules'
import { useResultStore } from '@/stores/result'
import { ElMessage } from 'element-plus'

const router = useRouter()
const filesStore = useFilesStore()
const rulesStore = useRulesStore()
const resultStore = useResultStore()

// 文件列表
const fileList = computed(() => filesStore.files)

// 当前选中的文件
const selectedFileId = ref('')
const selectedFile = computed(() => 
  fileList.value.find(f => f.id === selectedFileId.value)
)

// 预览状态
const previewLoading = ref(false)
const originalContent = ref('')
const maskedContent = ref('')
const sensitiveInfo = ref([])

// 编辑对话框
const editDialogVisible = ref(false)
const editingItem = ref(null)
const editMaskedValue = ref('')

// 原始内容 HTML（带高亮）
const originalHtml = computed(() => {
  return highlightSensitive(originalContent.value, sensitiveInfo.value)
})

// 脱敏后内容 HTML
const maskedHtml = computed(() => {
  return escapeHtml(maskedContent.value)
})

// 类型统计
const typeStats = computed(() => {
  const stats = {}
  sensitiveInfo.value.forEach(info => {
    if (!stats[info.type]) {
      stats[info.type] = 0
    }
    stats[info.type]++
  })
  return stats
})

// 监听文件选择变化
watch(selectedFileId, () => {
  refreshPreview()
})

// 组件挂载时初始化
onMounted(() => {
  if (fileList.value.length > 0) {
    selectedFileId.value = fileList.value[0].id
    refreshPreview()
  }
})

// 刷新预览
async function refreshPreview() {
  if (!selectedFile.value) return
  
  previewLoading.value = true
  
  try {
    // 调用 Tauri 命令生成预览
    const { invoke } = await import('@tauri-apps/api')
    const result = await invoke('generate_preview', {
      filePath: selectedFile.value.path,
      rules: rulesStore.enabledRules
    })
    
    originalContent.value = result.original
    maskedContent.value = result.masked
    sensitiveInfo.value = result.sensitiveInfo.map((info, index) => ({
      ...info,
      id: index
    }))
    
    // 保存预览结果
    filesStore.setPreviewContent(result)
  } catch (error) {
    console.error('生成预览失败:', error)
    ElMessage.error('生成预览失败: ' + (error.message || error))
  } finally {
    previewLoading.value = false
  }
}

// 高亮敏感信息
function highlightSensitive(content, info) {
  let html = escapeHtml(content)
  
  // 从后往前替换，避免位置偏移
  const sortedInfo = [...info].sort((a, b) => b.start - a.start)
  
  sortedInfo.forEach(item => {
    const before = html.substring(0, item.start)
    const text = html.substring(item.start, item.end)
    const after = html.substring(item.end)
    html = before + `<span class="highlight-sensitive">${text}</span>` + after
  })
  
  return html
}

// HTML 转义
function escapeHtml(text) {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

// 获取类型标签
function getTypeLabel(type) {
  const labels = {
    id_card: '身份证号',
    phone: '手机号',
    bank_card: '银行卡号',
    passport: '护照号',
    credit_code: '信用代码',
    email: '邮箱',
    license_plate: '车牌号',
    ipv4: 'IPv4',
    ipv6: 'IPv6',
    mac: 'MAC地址',
    api_key: 'API密钥',
    name: '姓名',
    company: '公司',
    address: '地址',
    amount: '金额',
    date: '日期',
    url: 'URL',
    telephone: '电话',
    custom: '自定义'
  }
  return labels[type] || type
}

// 编辑敏感信息
function editSensitive(item) {
  editingItem.value = item
  editMaskedValue.value = item.masked
  editDialogVisible.value = true
}

// 保存编辑
function saveEdit() {
  if (editingItem.value) {
    const index = sensitiveInfo.value.findIndex(i => i.id === editingItem.value.id)
    if (index !== -1) {
      sensitiveInfo.value[index].masked = editMaskedValue.value
    }
  }
  editDialogVisible.value = false
  ElMessage.success('已修改脱敏结果')
}

// 返回上一步
function goBack() {
  router.push('/rule-config')
}

// 开始处理
function startProcessing() {
  if (sensitiveInfo.value.length === 0) {
    ElMessage.warning('没有需要脱敏的内容')
    return
  }
  
  // 保存预览结果到 result store
  resultStore.clearResults()
  
  router.push('/processing')
}
</script>

<style lang="scss" scoped>
.preview-page {
  max-width: 1400px;
  margin: 0 auto;
}

.file-selector {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
}

.preview-container {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  overflow: hidden;
  
  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 24px;
    background: #f5f7fa;
    border-bottom: 1px solid #ebeef5;
    
    .file-info {
      display: flex;
      align-items: center;
      gap: 8px;
      font-weight: 500;
      color: #303133;
    }
  }
}

.diff-view {
  display: grid;
  grid-template-columns: 1fr 1fr;
  min-height: 400px;
  
  .diff-pane {
    border-right: 1px solid #ebeef5;
    
    &:last-child {
      border-right: none;
    }
    
    .pane-header {
      padding: 12px 16px;
      background: #fafafa;
      border-bottom: 1px solid #ebeef5;
      font-weight: 500;
      color: #606266;
    }
    
    .pane-content {
      padding: 16px;
      font-family: 'Consolas', 'Monaco', monospace;
      font-size: 13px;
      line-height: 1.6;
      white-space: pre-wrap;
      word-break: break-all;
      max-height: 400px;
      overflow-y: auto;
    }
  }
  
  .original {
    .pane-content {
      background: #fff;
    }
  }
  
  .masked {
    .pane-content {
      background: #f0f9eb;
    }
  }
}

.sensitive-list {
  padding: 24px;
  border-top: 1px solid #ebeef5;
  
  h3 {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 16px;
  }
  
  .sensitive-value {
    color: #E6A23C;
    font-weight: 500;
  }
  
  .masked-value {
    color: #67C23A;
    font-weight: 500;
  }
}

.stats-panel {
  margin-top: 24px;
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  h3 {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 16px;
  }
  
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 16px;
    
    .stat-item {
      background: #f5f7fa;
      border-radius: 8px;
      padding: 16px;
      text-align: center;
      
      .stat-label {
        font-size: 13px;
        color: #909399;
        margin-bottom: 8px;
      }
      
      .stat-value {
        font-size: 20px;
        font-weight: 600;
        color: #409EFF;
      }
    }
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
