/**
 * Data Masker - IP映射管理页面
 * 
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

<template>
  <div class="ip-mapping-page">
    <div class="page-card">
      <div class="section-title">
        <h3>🌐 IP网段映射</h3>
        <p>配置IP网段映射规则，处理后的IP将按照映射规则转换</p>
      </div>
      
      <!-- 映射策略配置 -->
      <div class="strategy-section">
        <h4>映射策略</h4>
        <div class="strategy-config">
          <div class="config-item">
            <span>内网IP前缀:</span>
            <el-select v-model="internalPrefix" style="width: 180px;" @change="updateStrategy">
              <el-option label="10.0.0.0/8" value="10.0" />
              <el-option label="10.10.0.0/16" value="10.10" />
              <el-option label="172.16.0.0/12" value="172.16" />
            </el-select>
          </div>
          <div class="config-item">
            <span>公网IP策略:</span>
            <el-select v-model="publicStrategy" style="width: 180px;" @change="updateStrategy">
              <el-option label="RFC 5737 文档地址" value="rfc5737" />
              <el-option label="完全隐藏" value="hide" />
              <el-option label="部分掩码" value="mask" />
            </el-select>
          </div>
        </div>
        
        <div class="mapping-example">
          <h5>映射示例</h5>
          <el-table :data="examples" border size="small">
            <el-table-column prop="original" label="原始IP" width="150" />
            <el-table-column prop="type" label="类型" width="100">
              <template #default="{ row }">
                <el-tag size="small" :type="row.type === '内网' ? 'success' : 'warning'">{{ row.type }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="mapped" label="映射后" width="150" />
          </el-table>
        </div>
      </div>
      
      <!-- 网段映射配置 -->
      <div class="mapping-config-section">
        <div class="section-header">
          <h4>网段映射配置</h4>
          <el-button type="primary" size="small" @click="addMapping">
            <el-icon><Plus /></el-icon>
            添加映射
          </el-button>
        </div>
        
        <div class="mapping-table">
          <el-table :data="settingsStore.settingsData.ipMappings" border style="width: 100%">
            <el-table-column label="原始网段" min-width="200">
              <template #default="{ row, $index }">
                <el-input 
                  v-model="row.original" 
                  placeholder="例如: 192.168.1.0/24"
                  :class="{ 'is-error': !validateCIDR(row.original) && row.original }"
                />
                <div v-if="!validateCIDR(row.original) && row.original" class="error-tip">
                  请输入有效的CIDR格式
                </div>
              </template>
            </el-table-column>
            <el-table-column label="映射网段" min-width="200">
              <template #default="{ row, $index }">
                <el-input 
                  v-model="row.mapped" 
                  placeholder="例如: 10.0.1.0/24"
                  :class="{ 'is-error': !validateCIDR(row.mapped) && row.mapped }"
                />
                <div v-if="!validateCIDR(row.mapped) && row.mapped" class="error-tip">
                  请输入有效的CIDR格式
                </div>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="80" align="center">
              <template #default="{ $index }">
                <el-button 
                  type="danger" 
                  size="small" 
                  :icon="Delete"
                  circle
                  @click="deleteMapping($index)"
                />
              </template>
            </el-table-column>
          </el-table>
          
          <div v-if="settingsStore.settingsData.ipMappings.length === 0" class="empty-tip">
            暂无网段映射配置，点击"添加映射"按钮添加
          </div>
        </div>
        
        <div class="action-bar">
          <el-button @click="resetMappings">重置</el-button>
          <el-button type="primary" @click="saveMappings">保存配置</el-button>
        </div>
      </div>
      
      <!-- IP映射测试 -->
      <div class="test-section">
        <h4>IP映射测试</h4>
        <div class="test-input">
          <el-input
            v-model="testIP"
            placeholder="输入IP地址，多个用逗号分隔"
            @keyup.enter="testMapping"
          >
            <template #append>
              <el-button @click="testMapping">测试</el-button>
            </template>
          </el-input>
        </div>
        
        <div v-if="testResults.length > 0" class="test-results">
          <el-table :data="testResults" border size="small">
            <el-table-column prop="original" label="原始IP" />
            <el-table-column prop="mapped" label="映射后" />
            <el-table-column prop="type" label="类型" width="100">
              <template #default="{ row }">
                <el-tag size="small" :type="row.type === 'internal' ? 'success' : 'warning'">
                  {{ row.type === 'internal' ? '内网' : '公网' }}
                </el-tag>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Delete } from '@element-plus/icons-vue'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

const internalPrefix = ref('10.10')
const publicStrategy = ref('rfc5737')
const testIP = ref('')
const testResults = ref([])

const examples = computed(() => [
  { original: '192.168.1.100', type: '内网', mapped: `${internalPrefix.value}.1.100` },
  { original: '10.0.0.50', type: '内网', mapped: `${internalPrefix.value}.0.50` },
  { original: '106.12.190.227', type: '公网', mapped: publicStrategy.value === 'hide' ? '[PUBLIC_IP_HIDDEN]' : '203.0.113.227' },
])

// CIDR格式验证正则
const CIDR_REGEX = /^((25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)\.){3}(25[0-5]|2[0-4]\d|1\d{2}|[1-9]?\d)\/([0-9]|[1-2][0-9]|3[0-2])$/

// 验证CIDR格式
function validateCIDR(cidr) {
  if (!cidr) return true // 空值不验证
  return CIDR_REGEX.test(cidr)
}

// 添加映射
function addMapping() {
  settingsStore.settingsData.ipMappings.push({
    original: '',
    mapped: ''
  })
}

// 删除映射
async function deleteMapping(index) {
  const mapping = settingsStore.settingsData.ipMappings[index]
  if (mapping.original || mapping.mapped) {
    try {
      await ElMessageBox.confirm('确定要删除此映射吗？', '确认删除', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      })
      settingsStore.settingsData.ipMappings.splice(index, 1)
    } catch {
      // 用户取消
    }
  } else {
    settingsStore.settingsData.ipMappings.splice(index, 1)
  }
}

// 重置映射
async function resetMappings() {
  if (settingsStore.settingsData.ipMappings.length === 0) return
  
  try {
    await ElMessageBox.confirm('确定要清空所有映射配置吗？', '确认重置', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    settingsStore.settingsData.ipMappings = []
    ElMessage.success('已重置')
  } catch {
    // 用户取消
  }
}

// 保存映射
function saveMappings() {
  // 验证所有映射
  const invalidMappings = settingsStore.settingsData.ipMappings.filter(
    m => (m.original && !validateCIDR(m.original)) || (m.mapped && !validateCIDR(m.mapped))
  )
  
  if (invalidMappings.length > 0) {
    ElMessage.error('存在无效的CIDR格式，请检查')
    return
  }
  
  // 过滤掉空映射
  const validMappings = settingsStore.settingsData.ipMappings.filter(
    m => m.original && m.mapped
  )
  
  settingsStore.settingsData.ipMappings = validMappings
  settingsStore.saveSettings()
  ElMessage.success('配置已保存')
}

// 测试映射
async function testMapping() {
  if (!testIP.value.trim()) return
  
  const ips = testIP.value.split(',').map(ip => ip.trim()).filter(Boolean)
  
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const results = await invoke('map_ip_batch', { ips })
    
    testResults.value = results.map(([original, mapped]) => ({
      original,
      mapped,
      type: original.startsWith('192.168.') || original.startsWith('10.') ? 'internal' : 'public'
    }))
  } catch (error) {
    ElMessage.error('映射失败: ' + error)
  }
}

// 更新策略
async function updateStrategy() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('set_mapping_strategy', {
      internalPrefix: internalPrefix.value,
      publicStrategy: publicStrategy.value
    })
    ElMessage.success('策略已更新')
  } catch (error) {
    ElMessage.error('更新失败: ' + error)
  }
}
</script>

<style lang="scss" scoped>
.ip-mapping-page {
  max-width: 1200px;
  margin: 0 auto;
  
  .section-title {
    margin-bottom: 24px;
    
    h3 {
      font-size: 20px;
      font-weight: 600;
      margin-bottom: 8px;
    }
    
    p {
      color: #909399;
      font-size: 14px;
    }
  }
  
  .strategy-section, .test-section, .mapping-config-section {
    margin-bottom: 32px;
    
    h4 {
      font-size: 16px;
      font-weight: 600;
      margin-bottom: 16px;
    }
  }
  
  .strategy-config {
    display: flex;
    gap: 32px;
    margin-bottom: 24px;
    
    .config-item {
      display: flex;
      align-items: center;
      gap: 12px;
    }
  }
  
  .mapping-example {
    h5 {
      font-size: 14px;
      margin-bottom: 12px;
      color: #606266;
    }
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }
  
  .mapping-table {
    margin-bottom: 16px;
    
    .error-tip {
      font-size: 11px;
      color: #f56c6c;
      margin-top: 4px;
    }
    
    :deep(.is-error .el-input__wrapper) {
      box-shadow: 0 0 0 1px #f56c6c inset;
    }
  }
  
  .empty-tip {
    text-align: center;
    color: #909399;
    padding: 32px 0;
    font-size: 14px;
  }
  
  .test-input {
    margin-bottom: 16px;
  }
  
  .test-results {
    margin-top: 16px;
  }
  
  .action-bar {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding-top: 16px;
    border-top: 1px solid #ebeef5;
  }
}
</style>
