// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="home-page">
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="welcome-content">
        <h1 class="welcome-title">
          <el-icon :size="48" color="#409EFF"><Shield /></el-icon>
          Data Masker
        </h1>
        <p class="welcome-desc">
          安全、高效的本地文件脱敏工具，敏感数据不上传云端
        </p>
        <el-button type="primary" size="large" class="start-btn" @click="startMasking">
          <el-icon><Right /></el-icon>
          开始脱敏
        </el-button>
      </div>
      
      <div class="feature-cards">
        <div class="feature-card">
          <el-icon :size="32" color="#409EFF"><Lock /></el-icon>
          <h3>本地处理</h3>
          <p>所有文件在本地处理，数据不出本机</p>
        </div>
        <div class="feature-card">
          <el-icon :size="32" color="#67C23A"><DocumentChecked /></el-icon>
          <h3>智能识别</h3>
          <p>内置18种敏感信息识别规则</p>
        </div>
        <div class="feature-card">
          <el-icon :size="32" color="#E6A23C"><SetUp /></el-icon>
          <h3>灵活配置</h3>
          <p>支持自定义规则和多种脱敏策略</p>
        </div>
        <div class="feature-card" @click="goToIPMapping" style="cursor: pointer;">
          <el-icon :size="32" color="#9C27B0"><Connection /></el-icon>
          <h3>IP映射</h3>
          <p>智能IP映射，保持网络拓扑关系</p>
        </div>
      </div>
    </div>
    
    <!-- 统计概览 -->
    <div class="stats-section">
      <h2 class="section-title">规则统计</h2>
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon blue">
            <el-icon :size="24"><Collection /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ ruleStats.builtinTotal }}</div>
            <div class="stat-label">内置规则</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon green">
            <el-icon :size="24"><CircleCheck /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ ruleStats.builtinEnabled }}</div>
            <div class="stat-label">已启用规则</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon orange">
            <el-icon :size="24"><EditPen /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ ruleStats.customTotal }}</div>
            <div class="stat-label">自定义规则</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon" style="background: linear-gradient(135deg, #9C27B0 0%, #BA68C8 100%);">
            <el-icon :size="24"><FolderOpened /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ recentFiles.length }}</div>
            <div class="stat-label">最近处理</div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 最近处理的文件 -->
    <div class="recent-section" v-if="recentFiles.length > 0">
      <div class="section-header">
        <h2 class="section-title">最近处理</h2>
        <el-button text type="primary" @click="clearRecent">清空记录</el-button>
      </div>
      <el-table :data="recentFiles" style="width: 100%">
        <el-table-column prop="name" label="文件名" min-width="200">
          <template #default="{ row }">
            <div class="file-name">
              <el-icon><Document /></el-icon>
              <span>{{ row.name }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="type" label="类型" width="100">
          <template #default="{ row }">
            <el-tag size="small">{{ getFileTypeLabel(row.type) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="processedAt" label="处理时间" width="180">
          <template #default="{ row }">
            {{ formatDate(row.processedAt) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" align="center">
          <template #default="{ row }">
            <el-button size="small" text type="primary" @click="openFile(row)">
              打开目录
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
    
    <!-- 使用指南 -->
    <div class="guide-section">
      <h2 class="section-title">使用指南</h2>
      <el-steps :active="0" align-center>
        <el-step title="选择文件" description="支持PDF、Word、Excel、TXT等多种格式" />
        <el-step title="配置规则" description="选择需要脱敏的敏感信息类型" />
        <el-step title="预览确认" description="查看脱敏效果并确认" />
        <el-step title="导出结果" description="下载脱敏后的文件" />
      </el-steps>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useRulesStore } from '@/stores/rules'

const router = useRouter()
const filesStore = useFilesStore()
const rulesStore = useRulesStore()

// 计算属性
const ruleStats = computed(() => rulesStore.ruleStats)
const recentFiles = computed(() => filesStore.recentFiles)

// 开始脱敏
function startMasking() {
  router.push('/file-select')
}

// 跳转到IP映射
function goToIPMapping() {
  router.push('/ip-mapping')
}

// 清空最近记录
function clearRecent() {
  filesStore.recentFiles = []
}

// 获取文件类型标签
function getFileTypeLabel(type) {
  const labels = {
    'pdf': 'PDF',
    'docx': 'Word',
    'xlsx': 'Excel',
    'txt': '文本',
    'csv': 'CSV',
    'json': 'JSON'
  }
  return labels[type] || type?.toUpperCase() || '未知'
}

// 格式化日期
function formatDate(dateStr) {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 打开文件目录
async function openFile(file) {
  // 调用 Tauri 命令打开文件所在目录
  try {
    const { invoke } = await import('@tauri-apps/api')
    await invoke('open_file_location', { path: file.path })
  } catch (error) {
    console.error('打开文件失败:', error)
  }
}
</script>

<style lang="scss" scoped>
.home-page {
  max-width: 1200px;
  margin: 0 auto;
}

// 欢迎区域
.welcome-section {
  text-align: center;
  padding: 40px 0;
  
  .welcome-content {
    .welcome-title {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 16px;
      font-size: 36px;
      font-weight: 700;
      color: #303133;
      margin-bottom: 16px;
    }
    
    .welcome-desc {
      font-size: 16px;
      color: #606266;
      margin-bottom: 32px;
    }
    
    .start-btn {
      padding: 16px 48px;
      font-size: 16px;
      border-radius: 8px;
      background: linear-gradient(135deg, #409EFF 0%, #36D1DC 100%);
      border: none;
      
      &:hover {
        opacity: 0.9;
        transform: translateY(-2px);
      }
    }
  }
  
  .feature-cards {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
    margin-top: 48px;
    
    .feature-card {
      background: #fff;
      border-radius: 12px;
      padding: 32px;
      box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
      transition: transform 0.2s, box-shadow 0.2s;
      
      &:hover {
        transform: translateY(-4px);
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
      }
      
      h3 {
        margin: 16px 0 8px;
        font-size: 18px;
        color: #303133;
      }
      
      p {
        font-size: 14px;
        color: #909399;
        margin: 0;
      }
    }
  }
}

// 统计区域
.stats-section {
  margin-top: 48px;
  
  .section-title {
    font-size: 20px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 20px;
  }
  
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20px;
  }
}

// 最近处理
.recent-section {
  margin-top: 32px;
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  
  .section-title {
    font-size: 20px;
    font-weight: 600;
    color: #303133;
  }
  
  .file-name {
    display: flex;
    align-items: center;
    gap: 8px;
  }
}

// 使用指南
.guide-section {
  margin-top: 48px;
  padding: 32px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  
  .section-title {
    font-size: 20px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 32px;
    text-align: center;
  }
}

// 响应式布局
@media (max-width: 992px) {
  .feature-cards {
    grid-template-columns: repeat(2, 1fr) !important;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr) !important;
  }
}

@media (max-width: 576px) {
  .feature-cards {
    grid-template-columns: 1fr !important;
  }
  
  .stats-grid {
    grid-template-columns: 1fr !important;
  }
}
</style>
