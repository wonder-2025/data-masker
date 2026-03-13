// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="progress-bar-wrapper">
    <div class="progress-header" v-if="showHeader">
      <span class="progress-label">{{ label }}</span>
      <span class="progress-text">{{ current }} / {{ total }}</span>
    </div>
    
    <div class="progress-container">
      <div 
        class="progress-track"
        :style="trackStyle"
      >
        <div 
          class="progress-fill"
          :class="{ 'animate': animate }"
          :style="fillStyle"
        >
          <div class="progress-shine" v-if="animate && percentage > 0 && percentage < 100"></div>
        </div>
      </div>
      
      <span class="progress-percentage" v-if="showPercentage">
        {{ displayPercentage }}%
      </span>
    </div>
    
    <div class="progress-status" v-if="status !== 'default'">
      <el-icon :size="16" :color="statusColor">
        <component :is="statusIcon" />
      </el-icon>
      <span :style="{ color: statusColor }">{{ statusText }}</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  // 当前进度
  current: {
    type: Number,
    default: 0
  },
  // 总数
  total: {
    type: Number,
    default: 100
  },
  // 标签
  label: {
    type: String,
    default: ''
  },
  // 是否显示头部
  showHeader: {
    type: Boolean,
    default: true
  },
  // 是否显示百分比
  showPercentage: {
    type: Boolean,
    default: true
  },
  // 进度条高度
  height: {
    type: Number,
    default: 20
  },
  // 进度条颜色
  color: {
    type: String,
    default: '#409EFF'
  },
  // 状态: default, success, warning, error
  status: {
    type: String,
    default: 'default'
  },
  // 是否显示动画
  animate: {
    type: Boolean,
    default: true
  }
})

// 计算百分比
const percentage = computed(() => {
  if (props.total === 0) return 0
  return Math.min(100, Math.round((props.current / props.total) * 100))
})

// 显示的百分比
const displayPercentage = computed(() => {
  return Math.min(100, Math.round((props.current / props.total) * 100))
})

// 进度条轨道样式
const trackStyle = computed(() => ({
  height: `${props.height}px`,
  borderRadius: `${props.height / 2}px`
}))

// 进度条填充样式
const fillStyle = computed(() => ({
  width: `${percentage.value}%`,
  backgroundColor: statusColor.value || props.color,
  borderRadius: `${props.height / 2}px`
}))

// 状态颜色
const statusColor = computed(() => {
  const colors = {
    default: props.color,
    success: '#67C23A',
    warning: '#E6A23C',
    error: '#F56C6C'
  }
  return colors[props.status] || props.color
})

// 状态图标
const statusIcon = computed(() => {
  const icons = {
    success: 'CircleCheckFilled',
    warning: 'WarningFilled',
    error: 'CircleCloseFilled'
  }
  return icons[props.status] || 'Loading'
})

// 状态文字
const statusText = computed(() => {
  const texts = {
    success: '完成',
    warning: '警告',
    error: '失败'
  }
  return texts[props.status] || ''
})
</script>

<style lang="scss" scoped>
.progress-bar-wrapper {
  width: 100%;
  
  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    
    .progress-label {
      font-size: 14px;
      font-weight: 500;
      color: #303133;
    }
    
    .progress-text {
      font-size: 13px;
      color: #909399;
    }
  }
  
  .progress-container {
    display: flex;
    align-items: center;
    gap: 12px;
    
    .progress-track {
      flex: 1;
      background: #ebeef5;
      overflow: hidden;
      position: relative;
      
      .progress-fill {
        height: 100%;
        transition: width 0.3s ease;
        position: relative;
        
        &.animate {
          background: linear-gradient(135deg, #409EFF 0%, #36D1DC 100%);
        }
        
        .progress-shine {
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: linear-gradient(
            90deg,
            transparent 0%,
            rgba(255, 255, 255, 0.3) 50%,
            transparent 100%
          );
          animation: shine 2s infinite;
        }
      }
    }
    
    .progress-percentage {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
      min-width: 45px;
      text-align: right;
    }
  }
  
  .progress-status {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-top: 8px;
    font-size: 13px;
  }
}

@keyframes shine {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>
