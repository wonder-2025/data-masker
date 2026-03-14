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
        <h3>🌐 IP映射管理</h3>
        <p>智能IP映射，保持网络拓扑关系</p>
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
      
      <!-- IP输入测试 -->
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
      
      <!-- 映射表管理 -->
      <div class="mapping-table-section">
        <div class="section-header">
          <h4>映射表 ({{ mappingCount }} 条记录)</h4>
          <div class="actions">
            <el-button size="small" @click="loadMappings">刷新</el-button>
            <el-button size="small" type="primary" @click="showExportDialog">导出</el-button>
            <el-button size="small" @click="showImportDialog">导入</el-button>
            <el-button size="small" type="danger" @click="clearMappings">清空</el-button>
          </div>
        </div>
        
        <el-table :data="mappingRecords" border style="width: 100%">
          <el-table-column prop="original_ip" label="原始IP" min-width="150" />
          <el-table-column prop="mapped_ip" label="映射后" min-width="150" />
          <el-table-column prop="ip_type" label="类型" width="100">
            <template #default="{ row }">
              <el-tag size="small" :type="row.ip_type === 'internal' ? 'success' : 'warning'">
                {{ row.ip_type === 'internal' ? '内网' : '公网' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="timestamp" label="时间" width="180">
            <template #default="{ row }">
              {{ formatTime(row.timestamp) }}
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>
    
    <!-- 导出对话框 -->
    <el-dialog v-model="exportDialogVisible" title="导出映射表" width="450px">
      <el-form label-width="100px">
        <el-form-item label="加密导出">
          <el-switch v-model="exportEncrypt" />
        </el-form-item>
        <el-form-item v-if="exportEncrypt" label="密码">
          <el-input v-model="exportPassword" type="password" placeholder="请输入加密密码" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="exportDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="doExport">确认导出</el-button>
      </template>
    </el-dialog>
    
    <!-- 导入对话框 -->
    <el-dialog v-model="importDialogVisible" title="导入映射表" width="500px">
      <el-form label-width="100px">
        <el-form-item label="加密导入">
          <el-switch v-model="importEncrypted" />
        </el-form-item>
        <el-form-item v-if="importEncrypted" label="密码">
          <el-input v-model="importPassword" type="password" placeholder="请输入解密密码" show-password />
        </el-form-item>
      </el-form>
      <el-upload
        drag
        accept=".json"
        :auto-upload="false"
        :on-change="handleImportFile"
      >
        <el-icon :size="48"><Upload /></el-icon>
        <div>拖拽JSON文件到此处</div>
      </el-upload>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'

const internalPrefix = ref('10.10')
const publicStrategy = ref('rfc5737')
const testIP = ref('')
const testResults = ref([])
const mappingRecords = ref([])
const mappingCount = ref(0)
const importDialogVisible = ref(false)
const exportDialogVisible = ref(false)
const exportEncrypt = ref(false)
const exportPassword = ref('')
const importEncrypted = ref(false)
const importPassword = ref('')

const examples = computed(() => [
  { original: '192.168.1.100', type: '内网', mapped: `${internalPrefix.value}.1.100` },
  { original: '10.0.0.50', type: '内网', mapped: `${internalPrefix.value}.0.50` },
  { original: '106.12.190.227', type: '公网', mapped: publicStrategy.value === 'hide' ? '[PUBLIC_IP_HIDDEN]' : '203.0.113.227' },
])

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

// 加载映射表
async function loadMappings() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    mappingRecords.value = await invoke('get_ip_mappings')
    mappingCount.value = await invoke('get_mapping_count')
  } catch (error) {
    ElMessage.error('加载失败: ' + error)
  }
}

// 显示导出对话框
function showExportDialog() {
  exportEncrypt.value = false
  exportPassword.value = ''
  exportDialogVisible.value = true
}

// 显示导入对话框
function showImportDialog() {
  importEncrypted.value = false
  importPassword.value = ''
  importDialogVisible.value = true
}

// 导出映射表
async function doExport() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const result = await invoke('export_ip_mappings', {
      encrypt: exportEncrypt.value,
      password: exportPassword.value || null
    })
    
    const blob = new Blob([JSON.stringify(result, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `ip-mappings-${Date.now()}.json`
    a.click()
    URL.revokeObjectURL(url)
    
    exportDialogVisible.value = false
    ElMessage.success(exportEncrypt.value ? '加密导出成功' : '导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error)
  }
}

// 导入映射表
async function handleImportFile(file) {
  const reader = new FileReader()
  reader.onload = async (e) => {
    try {
      const data = JSON.parse(e.target.result)
      const { invoke } = await import('@tauri-apps/api/core')
      
      // 检测是否为加密文件
      const isEncryptedFile = data.encrypted === true
      
      if (isEncryptedFile) {
        // 加密文件导入
        await invoke('import_ip_mappings', {
          records: [],
          encrypted: true,
          password: importPassword.value,
          encrypted_data: data.data,
          salt: data.salt
        })
      } else {
        // 普通文件导入
        await invoke('import_ip_mappings', {
          records: data,
          encrypted: false
        })
      }
      
      ElMessage.success('导入成功')
      importDialogVisible.value = false
      loadMappings()
    } catch (error) {
      ElMessage.error('导入失败: ' + error)
    }
  }
  reader.readAsText(file.raw)
}

// 清空映射表
async function clearMappings() {
  try {
    await ElMessageBox.confirm('确定要清空所有映射记录吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('clear_ip_mappings')
    ElMessage.success('已清空')
    loadMappings()
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('清空失败: ' + error)
    }
  }
}

// 格式化时间
function formatTime(timestamp) {
  if (!timestamp) return '-'
  return new Date(timestamp * 1000).toLocaleString('zh-CN')
}

onMounted(() => {
  loadMappings()
})
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
  
  .strategy-section, .test-section, .mapping-table-section {
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
  
  .test-input {
    margin-bottom: 16px;
  }
  
  .test-results {
    margin-top: 16px;
  }
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    
    .actions {
      display: flex;
      gap: 8px;
    }
  }
}
</style>
