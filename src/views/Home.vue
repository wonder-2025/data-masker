// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="home-page">
    <!-- 欢迎区域 - 紧凑版 -->
    <div class="welcome-section">
      <div class="welcome-header">
        <el-icon :size="40" color="#409EFF"><Shield /></el-icon>
        <div class="welcome-text">
          <h1 class="welcome-title">Data Masker</h1>
          <p class="welcome-desc">本地文件脱敏工具，敏感数据不上传云端</p>
        </div>
        <el-button type="primary" size="large" class="start-btn" @click="startMasking">
          <el-icon><Right /></el-icon>
          开始脱敏
        </el-button>
      </div>

      <div class="feature-cards">
        <div class="feature-card">
          <el-icon :size="28" color="#409EFF"><Lock /></el-icon>
          <div class="feature-content">
            <h3>本地处理</h3>
            <p>数据不出本机</p>
          </div>
        </div>
        <div class="feature-card">
          <el-icon :size="28" color="#67C23A"><DocumentChecked /></el-icon>
          <div class="feature-content">
            <h3>智能识别</h3>
            <p>18种识别规则</p>
          </div>
        </div>
        <div class="feature-card">
          <el-icon :size="28" color="#E6A23C"><SetUp /></el-icon>
          <div class="feature-content">
            <h3>灵活配置</h3>
            <p>自定义规则</p>
          </div>
        </div>
        <div class="feature-card" @click="goToIPMapping" style="cursor: pointer;">
          <el-icon :size="28" color="#9C27B0"><Connection /></el-icon>
          <div class="feature-content">
            <h3>IP映射</h3>
            <p>保持网络拓扑</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 统计概览 - 紧凑版 -->
    <div class="stats-section">
      <div class="stat-card">
        <el-icon :size="20" color="#409EFF"><Collection /></el-icon>
        <div class="stat-value">{{ ruleStats.builtinTotal }}</div>
        <div class="stat-label">内置规则</div>
      </div>
      <div class="stat-card">
        <el-icon :size="20" color="#67C23A"><CircleCheck /></el-icon>
        <div class="stat-value">{{ ruleStats.builtinEnabled }}</div>
        <div class="stat-label">已启用</div>
      </div>
      <div class="stat-card">
        <el-icon :size="20" color="#E6A23C"><EditPen /></el-icon>
        <div class="stat-value">{{ ruleStats.customTotal }}</div>
        <div class="stat-label">自定义</div>
      </div>
      <div class="stat-card">
        <el-icon :size="20" color="#9C27B0"><FolderOpened /></el-icon>
        <div class="stat-value">{{ recentFiles.length }}</div>
        <div class="stat-label">最近处理</div>
      </div>
    </div>

    <!-- 使用指南 - 紧凑版 -->
    <div class="guide-section">
      <h3>使用指南</h3>
      <div class="guide-steps">
        <div class="guide-step">
          <div class="step-num">1</div>
          <div class="step-text">选择文件</div>
        </div>
        <el-icon :size="20" color="#C0C4CC"><Right /></el-icon>
        <div class="guide-step">
          <div class="step-num">2</div>
          <div class="step-text">配置规则</div>
        </div>
        <el-icon :size="20" color="#C0C4CC"><Right /></el-icon>
        <div class="guide-step">
          <div class="step-num">3</div>
          <div class="step-text">预览确认</div>
        </div>
        <el-icon :size="20" color="#C0C4CC"><Right /></el-icon>
        <div class="guide-step">
          <div class="step-num">4</div>
          <div class="step-text">导出结果</div>
        </div>
      </div>
    </div>

    <!-- 最近处理的文件 - 紧凑版 -->
    <div class="recent-section" v-if="recentFiles.length > 0">
      <div class="section-header">
        <h3>最近处理</h3>
        <el-button text type="primary" size="small" @click="clearRecent">清空</el-button>
      </div>
      <div class="recent-list">
        <div v-for="file in recentFiles.slice(0, 5)" :key="file.path" class="recent-item">
          <el-icon :size="16"><Document /></el-icon>
          <span class="file-name">{{ file.name }}</span>
          <el-tag size="small" type="info">{{ getFileTypeLabel(file.type) }}</el-tag>
          <span class="file-time">{{ formatDate(file.processedAt) }}</span>
        </div>
      </div>
    </div>

    <!-- 支持格式提示 -->
    <div class="format-tips">
      <el-alert type="info" :closable="false">
        <template #title>
          支持：PDF、Word (.docx)、Excel (.xlsx/.xls)、PPT (.pptx)、TXT、CSV、JSON、PNG/JPG 图片
        </template>
        <template #default>
          注意：旧版 Office 格式 (.doc/.ppt) 暂不支持，请转换为新版格式
        </template>
      </el-alert>
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
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 打开文件目录
async function openFile(file) {
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
  padding: 16px;
}

// 欢迎区域 - 紧凑版
.welcome-section {
  .welcome-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;

    .welcome-text {
      flex: 1;

      .welcome-title {
        font-size: 24px;
        font-weight: 700;
        color: #303133;
        margin: 0;
      }

      .welcome-desc {
        font-size: 14px;
        color: #909399;
        margin: 4px 0 0;
      }
    }

    .start-btn {
      padding: 12px 32px;
    }
  }

  .feature-cards {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;

    .feature-card {
      display: flex;
      align-items: center;
      gap: 12px;
      background: #fff;
      border-radius: 8px;
      padding: 16px;
      box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
      transition: transform 0.2s, box-shadow 0.2s;

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
      }

      .feature-content {
        h3 {
          font-size: 14px;
          font-weight: 600;
          color: #303133;
          margin: 0 0 2px;
        }

        p {
          font-size: 12px;
          color: #909399;
          margin: 0;
        }
      }
    }
  }
}

// 统计区域 - 紧凑版
.stats-section {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-top: 20px;

  .stat-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: #fff;
    border-radius: 8px;
    padding: 16px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

    .stat-value {
      font-size: 24px;
      font-weight: 700;
      color: #303133;
    }

    .stat-label {
      font-size: 12px;
      color: #909399;
    }
  }
}

// 使用指南 - 紧凑版
.guide-section {
  background: #fff;
  border-radius: 8px;
  padding: 16px 20px;
  margin-top: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

  h3 {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
    margin: 0 0 12px;
  }

  .guide-steps {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;

    .guide-step {
      display: flex;
      align-items: center;
      gap: 8px;

      .step-num {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        background: linear-gradient(135deg, #409EFF 0%, #36D1DC 100%);
        color: #fff;
        font-size: 12px;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .step-text {
        font-size: 13px;
        color: #606266;
      }
    }
  }
}

// 最近处理 - 紧凑版
.recent-section {
  background: #fff;
  border-radius: 8px;
  padding: 16px 20px;
  margin-top: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;

    h3 {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      margin: 0;
    }
  }

  .recent-list {
    .recent-item {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 0;
      border-bottom: 1px solid #f0f0f0;

      &:last-child {
        border-bottom: none;
      }

      .file-name {
        flex: 1;
        font-size: 13px;
        color: #303133;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .file-time {
        font-size: 12px;
        color: #909399;
      }
    }
  }
}

// 格式提示
.format-tips {
  margin-top: 20px;
}

// 响应式布局
@media (max-width: 992px) {
  .feature-cards {
    grid-template-columns: repeat(2, 1fr) !important;
  }

  .stats-section {
    grid-template-columns: repeat(2, 1fr) !important;
  }
}

@media (max-width: 576px) {
  .welcome-header {
    flex-direction: column;
    text-align: center;

    .welcome-text {
      .welcome-title {
        font-size: 20px;
      }
    }
  }

  .feature-cards {
    grid-template-columns: 1fr !important;
  }

  .stats-section {
    grid-template-columns: 1fr 1fr !important;
  }

  .guide-steps {
    flex-wrap: wrap;
  }
}
</style>
