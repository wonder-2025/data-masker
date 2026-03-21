// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="result-page">
    <div class="page-header">
      <h1 class="page-title">处理结果</h1>
      <p class="page-desc">文件脱敏处理已完成</p>
    </div>
    
    <!-- 结果概览 -->
    <div class="result-overview">
      <el-result
        :icon="hasError ? 'error' : 'success'"
        :title="hasError ? '部分文件处理失败' : '处理完成'"
        :sub-title="overviewText"
      >
        <template #extra>
          <el-button type="primary" @click="exportAll">
            <el-icon><Download /></el-icon>
            导出全部文件
          </el-button>
          <el-button @click="openOutputDir">
            <el-icon><FolderOpened /></el-icon>
            打开输出目录
          </el-button>
        </template>
      </el-result>
    </div>
    
    <!-- 统计卡片 -->
    <div class="stats-cards">
      <div class="stat-card success">
        <div class="stat-icon">
          <el-icon :size="32"><CircleCheckFilled /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ successCount }}</div>
          <div class="stat-label">成功处理</div>
        </div>
      </div>
      
      <div class="stat-card error" v-if="errorCount > 0">
        <div class="stat-icon">
          <el-icon :size="32"><CircleCloseFilled /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ errorCount }}</div>
          <div class="stat-label">处理失败</div>
        </div>
      </div>
      
      <div class="stat-card info">
        <div class="stat-icon">
          <el-icon :size="32"><Search /></el-icon>
        </div>
        <div class="stat-content">
          <div class="stat-value">{{ totalSensitive }}</div>
          <div class="stat-label">敏感信息</div>
        </div>
      </div>
    </div>
    
    <!-- 敏感信息类型分布 -->
    <div class="chart-section" v-if="Object.keys(sensitiveStats).length > 0">
      <h3>敏感信息类型分布</h3>
      <div class="chart-container" ref="chartRef"></div>
    </div>
    
    <!-- 处理结果列表 -->
    <div class="result-list-section">
      <div class="section-header">
        <h3>处理详情</h3>
        <el-button text type="primary" @click="exportReport">
          <el-icon><Document /></el-icon>
          导出报告
        </el-button>
      </div>
      
      <el-table :data="results" style="width: 100%">
        <el-table-column type="expand">
          <template #default="{ row }">
            <div class="expand-content" v-if="row.sensitiveInfo && row.sensitiveInfo.length > 0">
              <h4>敏感信息列表</h4>
              <el-table :data="row.sensitiveInfo" size="small">
                <el-table-column type="index" label="#" width="50" />
                <el-table-column prop="type" label="类型" width="120">
                  <template #default="{ row }">
                    <el-tag size="small">{{ getTypeLabel(row.type) }}</el-tag>
                  </template>
                </el-table-column>
                <el-table-column prop="original" label="原始值" min-width="150" />
                <el-table-column prop="masked" label="脱敏后" min-width="150" />
              </el-table>
            </div>
            <el-empty v-else description="无敏感信息" :image-size="60" />
          </template>
        </el-table-column>
        
        <el-table-column prop="fileName" label="文件名" min-width="200">
          <template #default="{ row }">
            <div class="file-name-cell">
              <el-icon><Document /></el-icon>
              <span>{{ row.fileName }}</span>
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="row.status === 'done' ? 'success' : 'danger'" size="small">
              {{ row.status === 'done' ? '成功' : '失败' }}
            </el-tag>
          </template>
        </el-table-column>
        
        <el-table-column label="敏感信息" width="100" align="center">
          <template #default="{ row }">
            {{ row.sensitiveInfo?.length || 0 }} 处
          </template>
        </el-table-column>
        
        <el-table-column prop="processingTime" label="耗时" width="100" align="center" />
        
        <el-table-column label="操作" width="150" align="center">
          <template #default="{ row }">
            <el-button size="small" text type="primary" @click="previewResult(row)">
              预览
            </el-button>
            <el-button size="small" text type="primary" @click="downloadFile(row)" :disabled="row.status !== 'done'">
              下载
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    
    <!-- 映射表导出 -->
    <div class="mapping-section" v-if="hasReversibleMasking">
      <el-alert
        title="检测到可逆脱敏"
        type="info"
        description="已启用可逆脱敏策略，您可以导出映射表以支持数据恢复。"
        show-icon
        :closable="false"
      />
      <el-button type="primary" @click="exportMapping">
        <el-icon><Key /></el-icon>
        导出映射表
      </el-button>
    </div>
    
    <!-- 底部操作栏 -->
    <div class="action-bar">
      <el-button type="danger" @click="clearTempFiles">
        <el-icon><Delete /></el-icon>
        清除临时文件
      </el-button>
      <el-button type="primary" @click="startNew">
        <el-icon><RefreshRight /></el-icon>
        开始新任务
      </el-button>
    </div>
    
    <!-- 预览对话框 -->
    <el-dialog
      v-model="previewVisible"
      :title="previewFileName"
      width="80%"
      top="5vh"
      destroy-on-close
    >
      <div class="preview-content">
        <pre>{{ previewContent }}</pre>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useResultStore } from '@/stores/result'
import { useRulesStore } from '@/stores/rules'
import { ElMessage, ElMessageBox } from 'element-plus'
import * as echarts from 'echarts'

const router = useRouter()
const filesStore = useFilesStore()
const resultStore = useResultStore()
const rulesStore = useRulesStore()

// 图表引用
const chartRef = ref(null)
let chartInstance = null

// 预览相关
const previewVisible = ref(false)
const previewFileName = ref('')
const previewContent = ref('')

// 计算属性
const results = computed(() => resultStore.results)
const successCount = computed(() => resultStore.successCount)
const errorCount = computed(() => resultStore.errorCount)
const totalSensitive = computed(() => resultStore.totalSensitive)
const sensitiveStats = computed(() => resultStore.sensitiveStats)

const hasError = computed(() => errorCount.value > 0)

const overviewText = computed(() => {
  return `共处理 ${results.value.length} 个文件，发现 ${totalSensitive.value} 处敏感信息`
})

// 检查是否有可逆脱敏
const hasReversibleMasking = computed(() => {
  return rulesStore.enabledRules.some(r => r.strategy === 'reversible')
})

// 初始化图表
async function initChart() {
  if (!chartRef.value || Object.keys(sensitiveStats.value).length === 0) return
  
  await nextTick()
  
  if (chartInstance) {
    chartInstance.dispose()
  }
  
  chartInstance = echarts.init(chartRef.value)
  
  const data = Object.entries(sensitiveStats.value).map(([type, count]) => ({
    name: getTypeLabel(type),
    value: count
  }))
  
  const option = {
    tooltip: {
      trigger: 'item',
      formatter: '{b}: {c} ({d}%)'
    },
    legend: {
      orient: 'vertical',
      right: 20,
      top: 'center'
    },
    series: [
      {
        name: '敏感信息类型',
        type: 'pie',
        radius: ['40%', '70%'],
        avoidLabelOverlap: false,
        itemStyle: {
          borderRadius: 10,
          borderColor: '#fff',
          borderWidth: 2
        },
        label: {
          show: false,
          position: 'center'
        },
        emphasis: {
          label: {
            show: true,
            fontSize: 18,
            fontWeight: 'bold'
          }
        },
        labelLine: {
          show: false
        },
        data: data
      }
    ]
  }
  
  chartInstance.setOption(option)
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

// 导出全部文件
async function exportAll() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const outputPath = await invoke('export_all_results', {
      results: results.value
    })
    ElMessage.success(`文件已导出到: ${outputPath}`)
  } catch (error) {
    ElMessage.error('导出失败: ' + (error.message || error))
  }
}

// 打开输出目录
async function openOutputDir() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('open_output_directory')
  } catch (error) {
    ElMessage.error('打开目录失败: ' + (error.message || error))
  }
}

// 导出报告
async function exportReport() {
  try {
    console.log('[DEBUG] ========== 导出报告 ==========')
    console.log('[DEBUG] resultStore.results 数量:', resultStore.results.length)
    console.log('[DEBUG] 报告数据:', JSON.stringify(resultStore.getReportData(), null, 2))
    
    const { invoke } = await import('@tauri-apps/api/core')
    const reportPath = await invoke('export_report', {
      reportData: resultStore.getReportData()
    })
    ElMessage.success(`报告已导出: ${reportPath}`)
  } catch (error) {
    console.error('[DEBUG] 导出报告失败:', error)
    ElMessage.error('导出报告失败: ' + (error.message || error))
  }
}

// 导出映射表
function exportMapping() {
  const json = resultStore.exportMapping()
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `data-masker-mapping-${Date.now()}.json`
  a.click()
  URL.revokeObjectURL(url)
  ElMessage.success('映射表已导出')
}

// 预览结果
async function previewResult(result) {
  previewFileName.value = result.fileName
  previewVisible.value = true
  previewContent.value = result.maskedContent || '无法预览'
}

// 下载文件
async function downloadFile(result) {
  if (!result.outputPath) {
    ElMessage.warning('文件路径不存在')
    return
  }
  
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    
    // 调用后端读取文件并返回Base64
    const response = await invoke('read_file_base64', { path: result.outputPath })
    const fileData = JSON.parse(response)
    
    // 将Base64转换为Blob并下载
    const byteCharacters = atob(fileData.data)
    const byteNumbers = new Array(byteCharacters.length)
    for (let i = 0; i < byteCharacters.length; i++) {
      byteNumbers[i] = byteCharacters.charCodeAt(i)
    }
    const byteArray = new Uint8Array(byteNumbers)
    const blob = new Blob([byteArray], { type: fileData.mimeType })
    
    // 创建下载链接并触发下载
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = fileData.fileName
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
    
    ElMessage.success('文件下载成功')
  } catch (error) {
    console.error('下载失败:', error)
    // 如果下载失败，回退到打开文件位置
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('open_file_location', { path: result.outputPath })
      ElMessage.info('已打开文件所在目录，请手动复制文件')
    } catch (e) {
      ElMessage.error('下载文件失败: ' + (error.message || error))
    }
  }
}

// 清除临时文件
async function clearTempFiles() {
  try {
    await ElMessageBox.confirm(
      '确定要清除所有临时文件吗？这将无法恢复原始文件。',
      '确认清除',
      {
        confirmButtonText: '清除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('clear_temp_files')
    
    ElMessage.success('临时文件已清除')
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('清除失败: ' + (error.message || error))
    }
  }
}

// 开始新任务
function startNew() {
  filesStore.clearFiles()
  resultStore.clearResults()
  resultStore.clearLogs()
  router.push('/')
}

// 组件挂载时初始化图表
onMounted(() => {
  console.log('[DEBUG] ========== 结果页面加载 ==========')
  console.log('[DEBUG] resultStore.results 数量:', resultStore.results.length)
  console.log('[DEBUG] results 数量:', results.value.length)
  
  if (resultStore.results.length === 0) {
    console.error('[DEBUG] 错误: resultStore.results 为空!')
    ElMessage.warning('没有处理结果，请重新处理文件')
  }
  
  initChart()
  
  // 添加到最近文件
  results.value.forEach(result => {
    if (result.status === 'done') {
      filesStore.addRecentFile({
        name: result.fileName,
        path: result.outputPath,
        type: result.fileName.split('.').pop()
      })
    }
  })
})

// 组件卸载时清理ECharts实例
onUnmounted(() => {
  if (chartInstance) {
    chartInstance.dispose()
    chartInstance = null
  }
})
</script>

<style lang="scss" scoped>
.result-page {
  max-width: 1200px;
  margin: 0 auto;
}

.result-overview {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
  
  .stat-card {
    background: #fff;
    border-radius: 12px;
    padding: 24px;
    display: flex;
    align-items: center;
    gap: 16px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
    
    .stat-icon {
      width: 64px;
      height: 64px;
      border-radius: 16px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
    }
    
    &.success .stat-icon {
      background: linear-gradient(135deg, #67C23A 0%, #4CAF50 100%);
    }
    
    &.error .stat-icon {
      background: linear-gradient(135deg, #F56C6C 0%, #E53935 100%);
    }
    
    &.info .stat-icon {
      background: linear-gradient(135deg, #409EFF 0%, #36D1DC 100%);
    }
    
    .stat-content {
      .stat-value {
        font-size: 32px;
        font-weight: 700;
        color: #303133;
      }
      
      .stat-label {
        font-size: 14px;
        color: #909399;
        margin-top: 4px;
      }
    }
  }
}

.chart-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  h3 {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 16px;
  }
  
  .chart-container {
    height: 300px;
  }
}

.result-list-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .section-header {
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
  
  .file-name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .expand-content {
    padding: 16px 24px;
    
    h4 {
      font-size: 14px;
      font-weight: 600;
      color: #606266;
      margin-bottom: 12px;
    }
  }
}

.mapping-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .el-alert {
    margin-bottom: 16px;
  }
}

.action-bar {
  display: flex;
  justify-content: space-between;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #ebeef5;
}

.preview-content {
  background: #f5f7fa;
  padding: 16px;
  border-radius: 8px;
  max-height: 60vh;
  overflow-y: auto;
  
  pre {
    margin: 0;
    font-size: 13px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-all;
  }
}
</style>
