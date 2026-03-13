// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="rule-card" :class="{ disabled: !rule.enabled }">
    <div class="rule-header">
      <div class="rule-info">
        <el-switch v-model="localEnabled" @change="handleToggle" />
        <span class="rule-name">{{ rule.name }}</span>
      </div>
      <el-tag size="small" type="info">{{ rule.type }}</el-tag>
    </div>
    
    <div class="rule-desc" v-if="rule.description">
      {{ rule.description }}
    </div>
    
    <div class="rule-strategy" v-if="rule.enabled">
      <span class="strategy-label">脱敏策略:</span>
      <el-select
        v-model="localStrategy"
        size="small"
        style="width: 140px;"
        @change="handleStrategyChange"
      >
        <el-option label="完全隐藏" value="full_mask" />
        <el-option label="部分掩码" value="partial_mask" />
        <el-option label="假数据替换" value="fake_data" />
        <el-option label="可逆加密" value="reversible" />
        <el-option label="哈希脱敏" value="hash" />
      </el-select>
      
      <!-- 部分掩码配置 -->
      <div class="mask-config" v-if="localStrategy === 'partial_mask'">
        <span>保留前</span>
        <el-input-number
          v-model="localKeepStart"
          size="small"
          :min="0"
          :max="20"
          style="width: 80px;"
          @change="handleConfigChange"
        />
        <span>位，保留后</span>
        <el-input-number
          v-model="localKeepEnd"
          size="small"
          :min="0"
          :max="20"
          style="width: 80px;"
          @change="handleConfigChange"
        />
        <span>位</span>
      </div>
      
      <!-- 自定义替换配置 -->
      <div class="mask-config" v-if="localStrategy === 'custom'">
        <span>替换为:</span>
        <el-input
          v-model="localCustomText"
          size="small"
          style="width: 150px;"
          placeholder="输入替换文本"
          @change="handleConfigChange"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed } from 'vue'

const props = defineProps({
  rule: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['toggle', 'update-strategy'])

// 本地状态
const localEnabled = ref(props.rule.enabled)
const localStrategy = ref(props.rule.strategy)
const localKeepStart = ref(props.rule.strategyConfig?.keepStart || 3)
const localKeepEnd = ref(props.rule.strategyConfig?.keepEnd || 4)
const localCustomText = ref(props.rule.strategyConfig?.customText || '')

// 监听外部变化
watch(() => props.rule.enabled, (val) => {
  localEnabled.value = val
})

watch(() => props.rule.strategy, (val) => {
  localStrategy.value = val
})

watch(() => props.rule.strategyConfig, (val) => {
  localKeepStart.value = val?.keepStart || 3
  localKeepEnd.value = val?.keepEnd || 4
  localCustomText.value = val?.customText || ''
}, { deep: true })

// 切换启用状态
function handleToggle() {
  emit('toggle', props.rule.id)
}

// 策略变化
function handleStrategyChange() {
  emit('update-strategy', props.rule.id, localStrategy.value, {
    keepStart: localKeepStart.value,
    keepEnd: localKeepEnd.value,
    customText: localCustomText.value
  })
}

// 配置变化
function handleConfigChange() {
  emit('update-strategy', props.rule.id, localStrategy.value, {
    keepStart: localKeepStart.value,
    keepEnd: localKeepEnd.value,
    customText: localCustomText.value
  })
}
</script>

<style lang="scss" scoped>
.rule-card {
  background: #fff;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s;
  
  &:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }
  
  &.disabled {
    opacity: 0.6;
    background: #f5f7fa;
  }
  
  .rule-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    
    .rule-info {
      display: flex;
      align-items: center;
      gap: 12px;
      
      .rule-name {
        font-weight: 500;
        color: #303133;
      }
    }
  }
  
  .rule-desc {
    font-size: 12px;
    color: #909399;
    margin-bottom: 12px;
  }
  
  .rule-strategy {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 8px;
    padding-top: 12px;
    border-top: 1px solid #f0f0f0;
    
    .strategy-label {
      font-size: 13px;
      color: #606266;
    }
    
    .mask-config {
      display: flex;
      align-items: center;
      gap: 4px;
      font-size: 13px;
      color: #606266;
      
      span {
        white-space: nowrap;
      }
    }
  }
}
</style>
