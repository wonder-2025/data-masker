// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="home-page">
    <!-- 顶部欢迎区 -->
    <div class="header-section">
      <div class="header-left">
        <el-icon :size="28" color="#409EFF"><Shield /></el-icon>
        <div class="header-info">
          <h1 class="header-title">Data Masker</h1>
          <p class="header-desc">本地文件脱敏工具 · 数据安全不出本机</p>
        </div>
      </div>
      <el-button type="primary" @click="startMasking">
        <el-icon class="el-icon--left"><Right /></el-icon>
        开始脱敏
      </el-button>
    </div>

    <!-- 统计卡片 -->
    <div class="stats-section">
      <div class="stat-card">
        <div class="stat-icon" style="background: #e6f7ff;">
          <el-icon :size="18" color="#409EFF"><Collection /></el-icon>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ ruleStats.builtinTotal }}</span>
          <span class="stat-label">内置规则</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #f0f9eb;">
          <el-icon :size="18" color="#67C23A"><CircleCheck /></el-icon>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ ruleStats.builtinEnabled }}</span>
          <span class="stat-label">已启用</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #fdf6ec;">
          <el-icon :size="18" color="#E6A23C"><EditPen /></el-icon>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ ruleStats.customTotal }}</span>
          <span class="stat-label">自定义</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon" style="background: #f4e8ff;">
          <el-icon :size="18" color="#9C27B0"><FolderOpened /></el-icon>
        </div>
        <div class="stat-info">
          <span class="stat-value">{{ recentFiles.length }}</span>
          <span class="stat-label">已处理</span>
        </div>
      </div>
    </div>

    <!-- 功能区 -->
    <div class="feature-section">
      <div class="feature-card" @click="startMasking">
        <el-icon :size="20" color="#409EFF"><Upload /></el-icon>
        <span>选择文件</span>
      </div>
      <div class="feature-card" @click="goToRules">
        <el-icon :size="20" color="#67C23A"><SetUp /></el-icon>
        <span>规则配置</span>
      </div>
      <div class="feature-card" @click="goToIPMapping">
        <el-icon :size="20" color="#9C27B0"><Connection /></el-icon>
        <span>IP映射</span>
      </div>
      <div class="feature-card" @click="goToSettings">
        <el-icon :size="20" color="#909399"><Setting /></el-icon>
        <span>设置</span>
      </div>
    </div>

    <!-- 使用指南 -->
    <div class="guide-section">
      <div class="section-header">
        <span class="section-title">使用指南</span>
      </div>
      <div class="guide-steps">
        <div class="step-item">
          <span class="step-num">1</span>
          <span class="step-text">选择文件</span>
        </div>
        <el-icon :size="12" color="#DCDFE6"><ArrowRight /></el-icon>
        <div class="step-item">
          <span class="step-num">2</span>
          <span class="step-text">配置规则</span>
        </div>
        <el-icon :size="12" color="#DCDFE6"><ArrowRight /></el-icon>
        <div class="step-item">
          <span class="step-num">3</span>
          <span class="step-text">预览确认</span>
        </div>
        <el-icon :size="12" color="#DCDFE6"><ArrowRight /></el-icon>
        <div class="step-item">
          <span class="step-num">4</span>
          <span class="step-text">导出结果</span>
        </div>
      </div>
    </div>

    <!-- 支持格式 -->
    <div class="format-section">
      <div class="section-header">
        <span class="section-title">支持格式</span>
      </div>
      <div class="format-list">
        <el-tag size="small" type="info">PDF</el-tag>
        <el-tag size="small" type="info">Word</el-tag>
        <el-tag size="small" type="info">Excel</el-tag>
        <el-tag size="small" type="info">PPT</el-tag>
        <el-tag size="small" type="info">TXT</el-tag>
        <el-tag size="small" type="info">CSV</el-tag>
        <el-tag size="small" type="info">JSON</el-tag>
        <el-tag size="small" type="info">XML</el-tag>
      </div>
    </div>

    <!-- 最近处理 -->
    <div class="recent-section">
      <div class="section-header">
        <span class="section-title">最近处理</span>
        <el-button v-if="recentFiles.length > 0" text type="danger" size="small" @click="clearRecent">
          清空
        </el-button>
      </div>
      <div class="recent-list" v-if="recentFiles.length > 0">
        <div v-for="file in recentFiles.slice(0, 5)" :key="file.path" class="recent-item">
          <el-icon :size="16" :color="getFileIconColor(file.type)"><Document /></el-icon>
          <span class="file-name">{{ file.name }}</span>
          <el-tag size="small" type="info">{{ getFileTypeLabel(file.type) }}</el-tag>
          <span class="file-time">{{ formatDate(file.processedAt) }}</span>
        </div>
      </div>
      <div v-else class="empty-hint">暂无处理记录</div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useFilesStore } from '@/stores/files'
import { useRulesStore } from '@/stores/rules'
import { logCollector } from '@/utils/logCollector'

const router = useRouter()
const filesStore = useFilesStore()
const rulesStore = useRulesStore()

const ruleStats = computed(() => rulesStore.ruleStats)
const recentFiles = computed(() => filesStore.recentFiles)

function startMasking() {
  logCollector.featureUse('start_masking')
  router.push('/file-select')
}

function goToRules() {
  logCollector.featureUse('go_to_rules')
  router.push('/rule-config')
}

function goToIPMapping() {
  logCollector.featureUse('go_to_ip_mapping')
  router.push('/ip-mapping')
}

function goToSettings() {
  logCollector.featureUse('go_to_settings')
  router.push('/settings')
}

function clearRecent() {
  filesStore.recentFiles = []
}

function getFileTypeLabel(type) {
  const labels = { pdf: 'PDF', docx: 'Word', xlsx: 'Excel', txt: '文本', csv: 'CSV', json: 'JSON' }
  return labels[type] || type?.toUpperCase() || '未知'
}

function getFileIconColor(type) {
  const colors = { pdf: '#F56C6C', docx: '#409EFF', xlsx: '#67C23A', txt: '#909399' }
  return colors[type] || '#909399'
}

function formatDate(dateStr) {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}
</script>

<style lang="scss" scoped>
.home-page {
  max-width: 800px;
  margin: 0 auto;
  padding: 16px;
}

// 顶部欢迎区
.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  
  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .header-title {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
    color: #303133;
  }
  
  .header-desc {
    font-size: 12px;
    color: #909399;
    margin: 2px 0 0;
  }
}

// 统计卡片
.stats-section {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-bottom: 16px;
  
  .stat-card {
    display: flex;
    align-items: center;
    gap: 10px;
    background: #fff;
    border-radius: 8px;
    padding: 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
    
    .stat-icon {
      width: 36px;
      height: 36px;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    
    .stat-info {
      display: flex;
      flex-direction: column;
    }
    
    .stat-value {
      font-size: 18px;
      font-weight: 600;
      color: #303133;
    }
    
    .stat-label {
      font-size: 11px;
      color: #909399;
    }
  }
}

// 功能区
.feature-section {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
  margin-bottom: 16px;
  
  .feature-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    background: #fff;
    border-radius: 8px;
    padding: 16px 12px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
    cursor: pointer;
    transition: all 0.2s;
    
    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }
    
    span {
      font-size: 12px;
      color: #606266;
    }
  }
}

// 通用区块标题
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  
  .section-title {
    font-size: 13px;
    font-weight: 600;
    color: #303133;
  }
}

// 使用指南
.guide-section {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
  margin-bottom: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  
  .guide-steps {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .step-item {
    display: flex;
    align-items: center;
    gap: 6px;
    
    .step-num {
      width: 20px;
      height: 20px;
      border-radius: 50%;
      background: linear-gradient(135deg, #409EFF, #36D1DC);
      color: #fff;
      font-size: 11px;
      font-weight: 600;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    
    .step-text {
      font-size: 12px;
      color: #606266;
    }
  }
}

// 支持格式
.format-section {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
  margin-bottom: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  
  .format-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
}

// 最近处理
.recent-section {
  background: #fff;
  border-radius: 8px;
  padding: 14px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  
  .recent-list {
    .recent-item {
      display: flex;
      align-items: center;
      gap: 10px;
      padding: 10px 0;
      border-bottom: 1px solid #f0f0f0;
      
      &:last-child {
        border-bottom: none;
        padding-bottom: 0;
      }
      
      .file-name {
        flex: 1;
        font-size: 12px;
        color: #303133;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
      
      .file-time {
        font-size: 11px;
        color: #909399;
        white-space: nowrap;
      }
    }
  }
  
  .empty-hint {
    font-size: 12px;
    color: #909399;
    text-align: center;
    padding: 20px 0;
  }
}
</style>

<style lang="scss">
/* 暗色模式 */
html.dark {
  .home-page {
    .stats-section .stat-card,
    .feature-section .feature-card,
    .guide-section,
    .format-section,
    .recent-section {
      background: #1f1f1f;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    }
    
    .stat-value {
      color: #e5eaf3;
    }
    
    .section-title {
      color: #e5eaf3;
    }
    
    .step-text {
      color: #a3a6ad;
    }
    
    .recent-item .file-name {
      color: #e5eaf3;
    }
  }
}
</style>
