// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="processing-page">
    <div class="page-header">
      <h1 class="page-title">正在处理</h1>
      <p class="page-desc">正在进行文件脱敏处理，请稍候...</p>
    </div>
    
    <!-- 总体进度 -->
    <div class="overall-progress">
      <div class="progress-header">
        <span class="progress-label">总体进度</span>
        <span class="progress-text">{{ progress.current }} / {{ progress.total }}</span>
      </div>
      <el-progress
        :percentage="overallPercentage"
        :stroke-width="20"
        :format="progressFormat"
      />
    </div>
    
    <!-- 当前处理文件 -->
    <div class="current-file" v-if="progress.currentFile">
      <el-icon class="file-icon" :size="24"><Document /></el-icon>
      <span class="file-name">{{ progress.currentFile }}</span>
      <el-tag type="warning" size="small">处理中</el-tag>
    </div>
    
    <!-- 实时统计 -->
    <div class="realtime-stats">
      <div class="stat-card">
        <div class="stat-icon blue">
          <el-icon :size="24"><Search /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ detectedCount }}</div>
          <div class="stat-label">已检测敏感信息</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon green">
          <el-icon :size="24"><Finished /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ processedCount }}</div>
          <div class="stat-label">已处理文件</div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-icon orange">
          <el-icon :size="24"><Timer /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ elapsedTime }}</div>
          <div class="stat-label">已用时间</div>
        </div>
      </div>
    </div>
    
    <!-- 处理日志 -->
    <div class="log-section">
      <div class="log-header">
        <h3>处理日志</h3>
        <el-switch v-model="showLogs" active-text="显示日志" />
      </div>
      
      <div class="log-container" v-show="showLogs">
        <div
          v-for="log in logs"
          :key="log.id"
          :class="['log-item', log.level]"
        >
          <span class="log-time">{{ formatTime(log.timestamp) }}</span>
          <el-icon class="log-icon">
            <component :is="getLogIcon(log.level)" />
          </el-icon>
          <span class="log-message">{{ log.message }}</span>
        </div>
      </div>
    </div>
    
    <!-- 文件处理列表 -->
    <div class="file-list-section">
      <h3>文件处理状态</h3>
      <el-table :data="fileList" style="width: 100%">
        <el-table-column prop="name" label="文件名" min-width="200">
          <template #default="{ row }">
            <div class="file-name-cell">
              <el-icon><Document /></el-icon>
              <span>{{ row.name }}</span>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="status" label="状态" width="120" align="center">
          <template #default="{ row }">
            <div class="status-cell">
              <el-icon v-if="row.status === 'processing'" class="is-loading" color="#E6A23C">
                <Loading />
              </el-icon>
              <el-icon v-else-if="row.status === 'done'" color="#67C23A">
                <CircleCheckFilled />
              </el-icon>
              <el-icon v-else-if="row.status === 'error'" color="#F56C6C">
                <CircleCloseFilled />
              </el-icon>
              <el-icon v-else color="#909399">
                <Clock />
              </el-icon>
              <span>{{ getStatusLabel(row.status) }}</span>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="sensitiveCount" label="敏感信息" width="100" align="center">
          <template #default="{ row }">
            <span v-if="row.sensitiveCount">{{ row.sensitiveCount }} 处</span>
            <span v-else>-</span>
          </template>
        </el-table-column>
        
        <el-table-column prop="processingTime" label="耗时" width="100" align="center">
          <template #default="{ row }">
            {{ row.processingTime || '-' }}
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useRulesStore } from '@/stores/rules'
import { useResultStore } from '@/stores/result'
import { useSettingsStore } from '@/stores/settings'
import { ElMessage } from 'element-plus'

const router = useRouter()
const filesStore = useFilesStore()
const rulesStore = useRulesStore()
const resultStore = useResultStore()
const settingsStore = useSettingsStore()

// 进度状态
const progress = computed(() => resultStore.progress)
const logs = computed(() => resultStore.logs)

// 统计数据
const detectedCount = ref(0)
const processedCount = ref(0)
const elapsedTime = ref('00:00')

// 显示日志开关
const showLogs = ref(true)

// 文件列表（带状态）
const fileList = computed(() => filesStore.files)

// 总体百分比
const overallPercentage = computed(() => {
  if (progress.value.total === 0) return 0
  return Math.round((progress.value.current / progress.value.total) * 100)
})

// 计时器
let timer = null
let startTime = null

// 开始处理
async function startProcessing() {
  const files = filesStore.files
  const rules = rulesStore.enabledRules
  
  if (files.length === 0) {
    ElMessage.warning('没有要处理的文件')
    router.push('/file-select')
    return
  }
  
  if (rules.length === 0) {
    ElMessage.warning('没有启用的脱敏规则')
    router.push('/rule-config')
    return
  }
  
  // 初始化进度
  resultStore.updateProgress({
    current: 0,
    total: files.length,
    currentFile: '',
    status: 'processing'
  })
  
  // 开始计时
  startTime = Date.now()
  timer = setInterval(updateElapsedTime, 1000)
  
  // 添加开始日志
  resultStore.addLog('info', `开始处理 ${files.length} 个文件`)
  
  try {
    // 调用 Tauri 命令处理文件
    const { invoke } = await import('@tauri-apps/api/core')
    
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      
      // 更新当前文件
      resultStore.updateProgress({
        current: i,
        currentFile: file.name
      })
      
      filesStore.updateFileStatus(file.id, 'processing')
      resultStore.addLog('info', `正在处理: ${file.name}`)
      
      try {
        // 获取用户设置的输出目录 - 确保从最新设置中读取
        const outputDir = settingsStore.settingsData?.general?.outputDir || ''
        console.log('[DEBUG] Output directory from settings:', outputDir)
        console.log('[DEBUG] Full settings:', JSON.stringify(settingsStore.settingsData, null, 2))
        
        // 规则检查 - 确保规则正确传递
        console.log('[DEBUG] Rules to process:', rules.length, 'rules')
        console.log('[DEBUG] First rule sample:', rules[0] ? JSON.stringify(rules[0], null, 2) : 'No rules')
        
        const result = await invoke('process_file', {
          filePath: file.path,
          rules: rules,
          outputDir: outputDir || null
        })
        
        // 更新文件状态
        filesStore.updateFileStatus(file.id, 'done')
        files[i].sensitiveCount = result.sensitiveCount
        files[i].processingTime = result.processingTime
        
        // 添加结果
        resultStore.addResult({
          fileId: file.id,
          fileName: file.name,
          status: 'done',
          sensitiveInfo: result.sensitiveInfo,
          maskedContent: result.masked_content,
          outputPath: result.outputPath,
          processingTime: result.processingTime
        })
        
        detectedCount.value += result.sensitiveCount
        processedCount.value++
        
        resultStore.addLog('success', `处理完成: ${file.name}，发现 ${result.sensitiveCount} 处敏感信息`)
      } catch (error) {
        filesStore.updateFileStatus(file.id, 'error')
        resultStore.addLog('error', `处理失败: ${file.name} - ${error.message || error}`)
      }
    }
    
    // 完成处理
    resultStore.updateProgress({
      current: files.length,
      currentFile: '',
      status: 'done'
    })
    
    resultStore.addLog('success', '所有文件处理完成')
    
    // 跳转到结果页
    setTimeout(() => {
      router.push('/result')
    }, 1000)
    
  } catch (error) {
    console.error('处理文件失败:', error)
    resultStore.addLog('error', `处理失败: ${error.message || error}`)
    resultStore.updateProgress({ status: 'error' })
  } finally {
    if (timer) {
      clearInterval(timer)
    }
  }
}

// 更新已用时间
function updateElapsedTime() {
  if (!startTime) return
  const elapsed = Math.floor((Date.now() - startTime) / 1000)
  const minutes = Math.floor(elapsed / 60).toString().padStart(2, '0')
  const seconds = (elapsed % 60).toString().padStart(2, '0')
  elapsedTime.value = `${minutes}:${seconds}`
}

// 格式化进度
function progressFormat(percentage) {
  return `${percentage}%`
}

// 格式化时间
function formatTime(timestamp) {
  if (!timestamp) return ''
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

// 获取状态标签
function getStatusLabel(status) {
  const labels = {
    pending: '等待中',
    processing: '处理中',
    done: '已完成',
    error: '失败'
  }
  return labels[status] || status
}

// 获取日志图标
function getLogIcon(level) {
  const icons = {
    info: 'InfoFilled',
    success: 'CircleCheckFilled',
    warning: 'WarningFilled',
    error: 'CircleCloseFilled'
  }
  return icons[level] || 'InfoFilled'
}

// 组件挂载时开始处理
onMounted(async () => {
  // 等待设置加载完成
  await settingsStore.loadSettings()
  console.log('[DEBUG] Settings loaded:', settingsStore.settingsData.general.outputDir)
  startProcessing()
})

// 组件卸载时清理
onUnmounted(() => {
  if (timer) {
    clearInterval(timer)
  }
})
</script>

<style lang="scss" scoped>
.processing-page {
  max-width: 1000px;
  margin: 0 auto;
}

.overall-progress {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    
    .progress-label {
      font-size: 16px;
      font-weight: 600;
      color: #303133;
    }
    
    .progress-text {
      font-size: 14px;
      color: #909399;
    }
  }
}

.current-file {
  display: flex;
  align-items: center;
  gap: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 24px;
  
  .file-icon {
    color: #409EFF;
  }
  
  .file-name {
    flex: 1;
    font-weight: 500;
    color: #303133;
  }
}

.realtime-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 24px;
}

.log-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .log-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    
    h3 {
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;
    }
  }
  
  .log-container {
    background: #1a1a2e;
    border-radius: 8px;
    padding: 16px;
    max-height: 300px;
    overflow-y: auto;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 13px;
    
    .log-item {
      display: flex;
      align-items: flex-start;
      gap: 8px;
      padding: 4px 0;
      color: #a0a0a0;
      
      &.info { color: #909399; }
      &.success { color: #67C23A; }
      &.warning { color: #E6A23C; }
      &.error { color: #F56C6C; }
      
      .log-time {
        color: #666;
        font-size: 12px;
      }
      
      .log-icon {
        flex-shrink: 0;
      }
      
      .log-message {
        flex: 1;
      }
    }
  }
}

.file-list-section {
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
  
  .file-name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .status-cell {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
  }
}
</style>
