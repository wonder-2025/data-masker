// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="home-page">
    <!-- 欢迎区域 -->
    <div class="welcome-section">
      <div class="welcome-header">
        <div class="welcome-left">
          <el-icon :size="32" color="#409EFF"><Shield /></el-icon>
          <div class="welcome-text">
            <h1 class="welcome-title">Data Masker</h1>
            <p class="welcome-desc">本地文件脱敏工具 · 数据不出本机</p>
          </div>
        </div>
        <el-button type="primary" class="start-btn" @click="startMasking">
          <el-icon><Right /></el-icon>开始脱敏
        </el-button>
      </div>
      <div class="feature-cards">
        <div class="feature-card">
          <el-icon :size="22" color="#409EFF"><Lock /></el-icon>
          <span>本地处理</span>
        </div>
        <div class="feature-card">
          <el-icon :size="22" color="#67C23A"><DocumentChecked /></el-icon>
          <span>18种规则</span>
        </div>
        <div class="feature-card">
          <el-icon :size="22" color="#E6A23C"><SetUp /></el-icon>
          <span>自定义</span>
        </div>
        <div class="feature-card" @click="goToIPMapping" style="cursor:pointer">
          <el-icon :size="22" color="#9C27B0"><Connection /></el-icon>
          <span>IP映射</span>
        </div>
      </div>
    </div>

    <!-- 主内容区 -->
    <div class="main-content">
      <div class="left-panel">
        <!-- 统计 -->
        <div class="stats-row">
          <div class="stat-item">
            <el-icon :size="16" color="#409EFF"><Collection /></el-icon>
            <span class="stat-num">{{ ruleStats.builtinTotal }}</span>
            <span class="stat-label">内置规则</span>
          </div>
          <div class="stat-item">
            <el-icon :size="16" color="#67C23A"><CircleCheck /></el-icon>
            <span class="stat-num">{{ ruleStats.builtinEnabled }}</span>
            <span class="stat-label">已启用</span>
          </div>
          <div class="stat-item">
            <el-icon :size="16" color="#E6A23C"><EditPen /></el-icon>
            <span class="stat-num">{{ ruleStats.customTotal }}</span>
            <span class="stat-label">自定义</span>
          </div>
          <div class="stat-item">
            <el-icon :size="16" color="#9C27B0"><FolderOpened /></el-icon>
            <span class="stat-num">{{ recentFiles.length }}</span>
            <span class="stat-label">最近处理</span>
          </div>
        </div>

        <!-- 使用指南 -->
        <div class="guide-box">
          <div class="guide-title">使用指南</div>
          <div class="guide-flow">
            <span class="step"><i>1</i>选择文件</span>
            <el-icon :size="14" color="#C0C4CC"><Right /></el-icon>
            <span class="step"><i>2</i>配置规则</span>
            <el-icon :size="14" color="#C0C4CC"><Right /></el-icon>
            <span class="step"><i>3</i>预览确认</span>
            <el-icon :size="14" color="#C0C4CC"><Right /></el-icon>
            <span class="step"><i>4</i>导出结果</span>
          </div>
        </div>

        <!-- 支持格式 -->
        <div class="format-box">
          <div class="format-title">支持格式</div>
          <div class="format-tags">
            <el-tag size="small">PDF</el-tag>
            <el-tag size="small">Word</el-tag>
            <el-tag size="small">Excel</el-tag>
            <el-tag size="small">PPT</el-tag>
            <el-tag size="small">TXT</el-tag>
            <el-tag size="small">CSV</el-tag>
            <el-tag size="small">JSON</el-tag>
          </div>
        </div>
      </div>

      <div class="right-panel">
        <!-- 最近处理 -->
        <div class="recent-box">
          <div class="recent-header">
            <span>最近处理</span>
            <el-button v-if="recentFiles.length > 0" text type="primary" size="small" @click="clearRecent">清空</el-button>
          </div>
          <div class="recent-list" v-if="recentFiles.length > 0">
            <div v-for="file in recentFiles.slice(0, 5)" :key="file.path" class="recent-item">
              <el-icon :size="14"><Document /></el-icon>
              <span class="file-name">{{ file.name }}</span>
              <el-tag size="small" type="info">{{ getFileTypeLabel(file.type) }}</el-tag>
              <span class="file-time">{{ formatDate(file.processedAt) }}</span>
            </div>
          </div>
          <el-empty v-else description="暂无记录" :image-size="50" />
        </div>
      </div>
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

const ruleStats = computed(() => rulesStore.ruleStats)
const recentFiles = computed(() => filesStore.recentFiles)

function startMasking() {
  router.push('/file-select')
}

function goToIPMapping() {
  router.push('/ip-mapping')
}

function clearRecent() {
  filesStore.recentFiles = []
}

function getFileTypeLabel(type) {
  const labels = { pdf: 'PDF', docx: 'Word', xlsx: 'Excel', txt: '文本', csv: 'CSV', json: 'JSON' }
  return labels[type] || type?.toUpperCase() || '未知'
}

function formatDate(dateStr) {
  if (!dateStr) return '-'
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}
</script>

<style lang="scss" scoped>
.home-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 12px;
  overflow: hidden;
}

.welcome-section {
  .welcome-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
    .welcome-left { display: flex; align-items: center; gap: 10px; }
    .welcome-title { font-size: 18px; font-weight: 700; margin: 0; }
    .welcome-desc { font-size: 11px; color: #909399; margin: 2px 0 0; }
    .start-btn { padding: 8px 20px; }
  }
  .feature-cards {
    display: flex; gap: 8px;
    .feature-card {
      display: flex; align-items: center; gap: 6px;
      background: #fff; border-radius: 6px; padding: 8px 12px;
      box-shadow: 0 1px 3px rgba(0,0,0,0.05);
      &:hover { transform: translateY(-1px); box-shadow: 0 2px 6px rgba(0,0,0,0.08); }
      span { font-size: 11px; color: #606266; }
    }
  }
}

.main-content {
  flex: 1; display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-top: 12px; min-height: 0;
}

.left-panel, .right-panel {
  display: flex; flex-direction: column; gap: 10px;
}

.stats-row {
  display: grid; grid-template-columns: repeat(4, 1fr); gap: 6px;
  .stat-item {
    display: flex; flex-direction: column; align-items: center; gap: 2px;
    background: #fff; border-radius: 6px; padding: 8px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
    .stat-num { font-size: 16px; font-weight: 700; color: #303133; }
    .stat-label { font-size: 10px; color: #909399; }
  }
}

.guide-box, .format-box, .recent-box {
  background: #fff; border-radius: 6px; padding: 10px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.05);
}

.guide-title, .format-title, .recent-header {
  font-size: 12px; font-weight: 600; color: #303133; margin-bottom: 8px;
  display: flex; justify-content: space-between; align-items: center;
}

.guide-flow {
  display: flex; align-items: center; gap: 6px;
  .step {
    display: flex; align-items: center; gap: 4px;
    font-size: 11px; color: #606266;
    i {
      width: 18px; height: 18px; border-radius: 50%;
      background: linear-gradient(135deg, #409EFF, #36D1DC);
      color: #fff; font-style: normal; font-size: 10px;
      display: flex; align-items: center; justify-content: center;
    }
  }
}

.format-tags { display: flex; flex-wrap: wrap; gap: 4px; }

.recent-list {
  .recent-item {
    display: flex; align-items: center; gap: 6px; padding: 6px 0;
    border-bottom: 1px solid #f0f0f0;
    &:last-child { border-bottom: none; }
    .file-name { flex: 1; font-size: 11px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .file-time { font-size: 10px; color: #909399; }
  }
}
</style>
