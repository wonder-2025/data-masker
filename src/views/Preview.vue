// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="preview-page">
    <div class="page-header">
      <h1 class="page-title">预览确认</h1>
      <p class="page-desc">预览敏感信息检测结果，确认后开始处理</p>
    </div>
    
    <!-- 加载状态 -->
    <div class="loading-section" v-if="loading">
      <el-icon class="is-loading" :size="48" color="#409EFF"><Loading /></el-icon>
      <p>正在分析文件内容...</p>
    </div>
    
    <!-- 无文件状态 -->
    <el-empty v-else-if="fileList.length === 0" description="暂无文件">
      <el-button type="primary" @click="goBack">选择文件</el-button>
    </el-empty>
    
    <!-- 预览内容 -->
    <template v-else>
      <!-- 统计概览 -->
      <div class="stats-overview">
        <div class="stat-card">
          <el-icon :size="24" color="#409EFF"><Files /></el-icon>
          <div class="stat-info">
            <div class="stat-value">{{ fileList.length }}</div>
            <div class="stat-label">待处理文件</div>
          </div>
        </div>
        
        <div class="stat-card">
          <el-icon :size="24" color="#67C23A"><Search /></el-icon>
          <div class="stat-info">
            <div class="stat-value">{{ totalSensitive }}</div>
            <div class="stat-label">敏感信息</div>
          </div>
        </div>
        
        <div class="stat-card">
          <el-icon :size="24" color="#E6A23C"><Setting /></el-icon>
          <div class="stat-info">
            <div class="stat-value">{{ enabledRulesCount }}</div>
            <div class="stat-label">启用规则</div>
          </div>
        </div>
      </div>
      
      <!-- 文件预览列表 -->
      <div class="preview-list">
        <el-tabs v-model="activeFile" type="card">
          <el-tab-pane
            v-for="(preview, index) in previewResults"
            :key="index"
            :label="preview.fileName"
            :name="String(index)"
          >
            <!-- 敏感信息列表 -->
            <div class="sensitive-info" v-if="preview.sensitiveInfo && preview.sensitiveInfo.length > 0">
              <div class="info-header">
                <h4>检测到 {{ preview.sensitiveInfo.length }} 处敏感信息</h4>
                <el-tag type="warning" size="small">点击可查看详情</el-tag>
              </div>
              
              <el-table :data="preview.sensitiveInfo" style="width: 100%" max-height="300">
                <el-table-column type="index" label="#" width="50" />
                <el-table-column prop="type" label="类型" width="120">
                  <template #default="{ row }">
                    <el-tag size="small">{{ getTypeLabel(row.type) }}</el-tag>
                  </template>
                </el-table-column>
                <el-table-column prop="original" label="原始值" min-width="150" show-overflow-tooltip />
                <el-table-column prop="masked" label="脱敏后" min-width="150" show-overflow-tooltip />
                <el-table-column label="位置" width="100">
                  <template #default="{ row }">
                    行 {{ row.line }}
                  </template>
                </el-table-column>
              </el-table>
            </div>
            
            <el-empty v-else description="未检测到敏感信息" :image-size="80">
              <template #description>
                <p>该文件未检测到敏感信息</p>
                <p class="sub-desc">可能是文件内容不包含敏感信息，或规则配置不完整</p>
              </template>
            </el-empty>
            
            <!-- 内容预览 -->
            <div class="content-preview">
              <div class="preview-header">
                <h4>内容预览</h4>
                <el-button-group>
                  <el-button size="small" :type="showOriginal ? 'primary' : 'default'" @click="showOriginal = true">
                    原始内容
                  </el-button>
                  <el-button size="small" :type="!showOriginal ? 'primary' : 'default'" @click="showOriginal = false">
                    脱敏后
                  </el-button>
                </el-button-group>
              </div>
              
              <div class="preview-content">
                <pre v-if="showOriginal">{{ preview.original || '无法预览原始内容' }}</pre>
                <pre v-else>{{ preview.masked || '无法预览脱敏后内容' }}</pre>
              </div>
            </div>
          </el-tab-pane>
        </el-tabs>
      </div>
      
      <!-- 底部操作栏 -->
      <div class="action-bar">
        <el-button @click="goBack">
          <el-icon class="el-icon--left"><ArrowLeft /></el-icon>
          上一步
        </el-button>
        
        <div class="action-right">
          <el-checkbox v-model="confirmChecked">我已确认预览内容</el-checkbox>
          <el-button type="primary" :disabled="!confirmChecked" @click="startProcessing">
            开始处理
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useRulesStore } from '@/stores/rules'
import { useResultStore } from '@/stores/result'
import { ElMessage, ElMessageBox } from 'element-plus'

const router = useRouter()
const filesStore = useFilesStore()
const rulesStore = useRulesStore()
const resultStore = useResultStore()

// 状态
const loading = ref(true)
const activeFile = ref('0')
const showOriginal = ref(true)
const confirmChecked = ref(false)
const previewResults = ref([])

// 计算属性
const fileList = computed(() => filesStore.files)
const enabledRulesCount = computed(() => rulesStore.enabledRules.length)

const totalSensitive = computed(() => {
  return previewResults.value.reduce((sum, p) => sum + (p.sensitiveInfo?.length || 0), 0)
})

// 加载预览数据
async function loadPreview() {
  loading.value = true
  previewResults.value = []
  
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const rules = rulesStore.enabledRules
    
    for (const file of fileList.value) {
      try {
        const preview = await invoke('generate_preview', {
          filePath: file.path,
          rules: rules
        })
        
        previewResults.value.push({
          fileName: file.name,
          ...preview
        })
      } catch (error) {
        console.error('预览失败:', file.name, error)
        
        // 添加错误预览结果
        previewResults.value.push({
          fileName: file.name,
          original: '',
          masked: '',
          sensitiveInfo: [],
          stats: { totalSensitive: 0, byType: {} },
          error: error.message || String(error)
        })
        
        // 显示错误消息
        ElMessage.warning(`${file.name}: ${error.message || error}`)
      }
    }
  } catch (error) {
    console.error('加载预览失败:', error)
    ElMessage.error('加载预览失败: ' + (error.message || error))
  } finally {
    loading.value = false
  }
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
    keyword: '关键字',
    custom: '自定义'
  }
  return labels[type] || type
}

// 返回上一步
function goBack() {
  router.push('/rule-config')
}

// 开始处理
async function startProcessing() {
  if (!confirmChecked.value) {
    ElMessage.warning('请确认预览内容后再开始处理')
    return
  }
  
  // 检查是否有敏感信息
  const hasSensitive = previewResults.value.some(p => p.sensitiveInfo && p.sensitiveInfo.length > 0)
  
  if (!hasSensitive) {
    try {
      await ElMessageBox.confirm(
        '未检测到敏感信息，是否继续处理？',
        '提示',
        {
          confirmButtonText: '继续处理',
          cancelButtonText: '返回检查',
          type: 'warning'
        }
      )
    } catch {
      return
    }
  }
  
  // 跳转到处理页面
  router.push('/processing')
}

// 组件挂载时加载预览
onMounted(() => {
  if (fileList.value.length === 0) {
    ElMessage.warning('请先选择文件')
    router.push('/file-select')
    return
  }
  
  if (enabledRulesCount.value === 0) {
    ElMessage.warning('请至少启用一条脱敏规则')
    router.push('/rule-config')
    return
  }
  
  loadPreview()
})
</script>

<style lang="scss" scoped>
.preview-page {
  max-width: 1200px;
  margin: 0 auto;
}

.loading-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 400px;
  
  p {
    margin-top: 16px;
    color: #909399;
  }
}

.stats-overview {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  margin-bottom: 24px;
  
  .stat-card {
    background: #fff;
    border-radius: 12px;
    padding: 20px;
    display: flex;
    align-items: center;
    gap: 16px;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
    
    .stat-info {
      .stat-value {
        font-size: 28px;
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

.preview-list {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.sensitive-info {
  margin-bottom: 24px;
  
  .info-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    
    h4 {
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;
    }
  }
}

.content-preview {
  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    
    h4 {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      margin: 0;
    }
  }
  
  .preview-content {
    background: #1a1a2e;
    border-radius: 8px;
    padding: 16px;
    max-height: 400px;
    overflow-y: auto;
    
    pre {
      margin: 0;
      font-family: 'Consolas', 'Monaco', monospace;
      font-size: 13px;
      line-height: 1.6;
      color: #e5eaf3;
      white-space: pre-wrap;
      word-break: break-all;
    }
  }
}

.action-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #ebeef5;
  
  .action-right {
    display: flex;
    align-items: center;
    gap: 16px;
  }
}

.sub-desc {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}
</style>
