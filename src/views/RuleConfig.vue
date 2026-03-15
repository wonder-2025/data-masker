// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="rule-config-page">
    <div class="page-header">
      <h1 class="page-title">规则配置</h1>
      <p class="page-desc">选择需要脱敏的敏感信息类型，并配置脱敏策略</p>
    </div>
    
    <el-tabs v-model="activeTab" class="config-tabs">
      <!-- 内置规则标签页 -->
      <el-tab-pane label="内置规则" name="builtin">
        <div class="rule-categories">
          <el-collapse v-model="activeCategories">
            <!-- 个人信息 -->
            <el-collapse-item title="个人信息" name="personal">
              <div class="rule-grid">
                <RuleCard
                  v-for="rule in personalRules"
                  :key="rule.id"
                  :rule="rule"
                  @toggle="toggleRule"
                  @update-strategy="updateRuleStrategy"
                />
              </div>
            </el-collapse-item>
            
            <!-- 联系方式 -->
            <el-collapse-item title="联系方式" name="contact">
              <div class="rule-grid">
                <RuleCard
                  v-for="rule in contactRules"
                  :key="rule.id"
                  :rule="rule"
                  @toggle="toggleRule"
                  @update-strategy="updateRuleStrategy"
                />
              </div>
            </el-collapse-item>
            
            <!-- 网络信息 -->
            <el-collapse-item title="网络信息" name="network">
              <div class="rule-grid">
                <RuleCard
                  v-for="rule in networkRules"
                  :key="rule.id"
                  :rule="rule"
                  @toggle="toggleRule"
                  @update-strategy="updateRuleStrategy"
                />
              </div>
            </el-collapse-item>
            
            <!-- 其他信息 -->
            <el-collapse-item title="其他信息" name="other">
              <div class="rule-grid">
                <RuleCard
                  v-for="rule in otherRules"
                  :key="rule.id"
                  :rule="rule"
                  @toggle="toggleRule"
                  @update-strategy="updateRuleStrategy"
                />
              </div>
            </el-collapse-item>
          </el-collapse>
        </div>
      </el-tab-pane>
      
      <!-- 自定义规则标签页 -->
      <el-tab-pane label="自定义规则" name="custom">
        <div class="custom-rules-section">
          <div class="section-header">
            <el-button type="primary" @click="showAddRuleDialog">
              <el-icon><Plus /></el-icon>
              添加规则
            </el-button>
            <div class="import-export">
              <el-button @click="importRules">
                <el-icon><Upload /></el-icon>
                导入规则
              </el-button>
              <el-button @click="exportRules" :disabled="customRules.length === 0">
                <el-icon><Download /></el-icon>
                导出规则
              </el-button>
            </div>
          </div>
          
          <el-empty v-if="customRules.length === 0" description="暂无自定义规则">
            <el-button type="primary" @click="showAddRuleDialog">添加第一个规则</el-button>
          </el-empty>
          
          <el-table v-else :data="customRules" style="width: 100%">
            <el-table-column prop="name" label="规则名称" min-width="120" />
            <el-table-column prop="mode" label="类型" width="100" align="center">
              <template #default="{ row }">
                <el-tag size="small" :type="row.mode === 'keyword' ? 'success' : 'primary'">
                  {{ row.mode === 'keyword' ? '关键字' : '正则' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="匹配内容" min-width="200" show-overflow-tooltip>
              <template #default="{ row }">
                <span v-if="row.mode === 'keyword'">
                  "{{ row.keyword }}" → "{{ row.replacement }}"
                  <el-tag v-if="row.caseSensitive" size="small" type="warning" style="margin-left: 4px">区分大小写</el-tag>
                </span>
                <span v-else>{{ row.pattern }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="strategy" label="脱敏策略" width="120">
              <template #default="{ row }">
                <el-tag size="small" v-if="row.mode === 'regex'">{{ getStrategyLabel(row.strategy) }}</el-tag>
                <el-tag size="small" type="info" v-else>直接替换</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="enabled" label="状态" width="100" align="center">
              <template #default="{ row }">
                <el-switch v-model="row.enabled" @change="toggleCustomRule(row.id)" />
              </template>
            </el-table-column>
            <el-table-column label="操作" width="150" align="center">
              <template #default="{ row }">
                <el-button size="small" text type="primary" @click="editRule(row)">编辑</el-button>
                <el-button size="small" text type="danger" @click="deleteRule(row.id)">删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-tab-pane>
    </el-tabs>
    
    <!-- 规则统计 -->
    <div class="rule-stats">
      <div class="stat-item">
        <el-icon :size="20" color="#409EFF"><Collection /></el-icon>
        <span>已启用 {{ enabledRulesCount }} 条规则</span>
      </div>
      <div class="stat-item">
        <el-icon :size="20" color="#67C23A"><CircleCheck /></el-icon>
        <span>自定义 {{ customRules.length }} 条规则</span>
      </div>
    </div>
    
    <!-- 底部操作栏 -->
    <div class="action-bar">
      <el-button @click="goBack">
        <el-icon class="el-icon--left"><ArrowLeft /></el-icon>
        上一步
      </el-button>
      <el-button @click="resetRules">重置为默认</el-button>
      <el-button type="primary" @click="goNext">
        下一步：预览确认
        <el-icon class="el-icon--right"><ArrowRight /></el-icon>
      </el-button>
    </div>
    
    <!-- 添加/编辑规则对话框 -->
    <el-dialog
      v-model="ruleDialogVisible"
      :title="isEditMode ? '编辑规则' : '添加自定义规则'"
      width="600px"
      destroy-on-close
    >
      <el-form :model="ruleForm" :rules="ruleFormRules" ref="ruleFormRef" label-width="100px">
        <el-form-item label="规则名称" prop="name">
          <el-input v-model="ruleForm.name" placeholder="请输入规则名称" />
        </el-form-item>
        
        <el-form-item label="规则类型" prop="mode">
          <el-radio-group v-model="ruleForm.mode">
            <el-radio value="regex">正则表达式</el-radio>
            <el-radio value="keyword">关键字替换</el-radio>
          </el-radio-group>
        </el-form-item>
        
        <!-- 正则表达式模式 -->
        <el-form-item v-if="ruleForm.mode === 'regex'" label="正则表达式" prop="pattern">
          <el-input
            v-model="ruleForm.pattern"
            type="textarea"
            :rows="3"
            placeholder="请输入正则表达式"
          />
          <div class="form-tip">
            示例：\d{17}[\dXx] (匹配身份证号)
          </div>
        </el-form-item>
        
        <!-- 关键字替换模式 -->
        <template v-if="ruleForm.mode === 'keyword'">
          <el-form-item label="查找关键字" prop="keyword">
            <el-input
              v-model="ruleForm.keyword"
              placeholder="请输入要查找的关键字"
            />
            <div class="form-tip">
              将在文件中查找此关键字并替换
            </div>
          </el-form-item>
          
          <el-form-item label="替换为" prop="replacement">
            <el-input
              v-model="ruleForm.replacement"
              placeholder="请输入替换后的文本"
            />
          </el-form-item>
          
          <el-form-item label="大小写敏感">
            <el-switch v-model="ruleForm.caseSensitive" />
            <span class="switch-label">{{ ruleForm.caseSensitive ? '是' : '否（默认）' }}</span>
          </el-form-item>
        </template>
        
        <el-form-item label="规则描述" prop="description">
          <el-input v-model="ruleForm.description" placeholder="请输入规则描述（可选）" />
        </el-form-item>
        
        <el-form-item v-if="ruleForm.mode === 'regex'" label="脱敏策略" prop="strategy">
          <el-select v-model="ruleForm.strategy" style="width: 100%;">
            <el-option label="完全隐藏" value="full_mask" />
            <el-option label="部分掩码" value="partial_mask" />
            <el-option label="假数据替换" value="fake_data" />
            <el-option label="可逆加密" value="reversible" />
            <el-option label="哈希脱敏" value="hash" />
            <el-option label="自定义替换" value="custom" />
          </el-select>
        </el-form-item>
        
        <el-form-item
          v-if="ruleForm.mode === 'regex' && ruleForm.strategy === 'partial_mask'"
          label="保留位数"
        >
          <div class="keep-digits">
            <el-input-number v-model="ruleForm.keepStart" :min="0" :max="20" placeholder="前" />
            <span>前</span>
            <el-input-number v-model="ruleForm.keepEnd" :min="0" :max="20" placeholder="后" />
            <span>后</span>
          </div>
        </el-form-item>
        
        <el-form-item
          v-if="ruleForm.mode === 'regex' && ruleForm.strategy === 'custom'"
          label="替换文本"
          prop="customText"
        >
          <el-input v-model="ruleForm.customText" placeholder="请输入替换文本" />
        </el-form-item>
      </el-form>
      
      <template #footer>
        <el-button @click="ruleDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveRule">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useRulesStore } from '@/stores/rules'
import { useFilesStore } from '@/stores/files'
import { ElMessage, ElMessageBox } from 'element-plus'
import RuleCard from '@/components/RuleCard.vue'
import { regexValidator } from '@/utils/regexValidator'

const router = useRouter()
const rulesStore = useRulesStore()
const filesStore = useFilesStore()

// 当前标签页
const activeTab = ref('builtin')

// 展开的分类
const activeCategories = ref(['personal', 'contact', 'network', 'other'])

// 按分类分组规则
const personalRules = computed(() => 
  rulesStore.builtinRulesList.filter(r => 
    ['id_card', 'passport', 'name'].includes(r.type)
  )
)

const contactRules = computed(() => 
  rulesStore.builtinRulesList.filter(r => 
    ['phone', 'email', 'telephone'].includes(r.type)
  )
)

const networkRules = computed(() => 
  rulesStore.builtinRulesList.filter(r => 
    ['ipv4', 'ipv6', 'mac', 'url', 'api_key'].includes(r.type)
  )
)

const otherRules = computed(() => 
  rulesStore.builtinRulesList.filter(r => 
    ['bank_card', 'credit_code', 'license_plate', 'company', 'address', 'amount', 'date'].includes(r.type)
  )
)

const customRules = computed(() => rulesStore.customRules)

// 已启用规则数量
const enabledRulesCount = computed(() => rulesStore.ruleStats.totalEnabled)

// 规则对话框
const ruleDialogVisible = ref(false)
const isEditMode = ref(false)
const editingRuleId = ref(null)
const ruleFormRef = ref(null)

const ruleForm = reactive({
  name: '',
  mode: 'regex', // 'regex' 或 'keyword'
  pattern: '',
  keyword: '',
  replacement: '***',
  caseSensitive: false,
  description: '',
  strategy: 'full_mask',
  keepStart: 3,
  keepEnd: 4,
  customText: ''
})

const ruleFormRules = {
  name: [{ required: true, message: '请输入规则名称', trigger: 'blur' }],
  pattern: [{ 
    required: true, 
    message: '请输入正则表达式', 
    trigger: 'blur',
    validator: (rule, value, callback) => {
      if (ruleForm.mode === 'regex' && !value) {
        callback(new Error('请输入正则表达式'))
      } else {
        callback()
      }
    }
  }],
  keyword: [{ 
    required: true, 
    message: '请输入要查找的关键字', 
    trigger: 'blur',
    validator: (rule, value, callback) => {
      if (ruleForm.mode === 'keyword' && !value) {
        callback(new Error('请输入要查找的关键字'))
      } else {
        callback()
      }
    }
  }],
  strategy: [{ required: true, message: '请选择脱敏策略', trigger: 'change' }]
}

// 切换规则状态
function toggleRule(ruleId) {
  rulesStore.toggleRule(ruleId)
}

// 切换自定义规则状态
function toggleCustomRule(ruleId) {
  rulesStore.toggleRule(ruleId)
}

// 更新规则策略
function updateRuleStrategy(ruleId, strategy, config) {
  rulesStore.updateRuleStrategy(ruleId, strategy, config)
}

// 获取策略标签
function getStrategyLabel(strategy) {
  const labels = {
    full_mask: '完全隐藏',
    partial_mask: '部分掩码',
    fake_data: '假数据替换',
    reversible: '可逆加密',
    hash: '哈希脱敏',
    custom: '自定义替换'
  }
  return labels[strategy] || strategy
}

// 显示添加规则对话框
function showAddRuleDialog() {
  isEditMode.value = false
  editingRuleId.value = null
  Object.assign(ruleForm, {
    name: '',
    mode: 'regex',
    pattern: '',
    keyword: '',
    replacement: '***',
    caseSensitive: false,
    description: '',
    strategy: 'full_mask',
    keepStart: 3,
    keepEnd: 4,
    customText: ''
  })
  ruleDialogVisible.value = true
}

// 编辑规则
function editRule(rule) {
  isEditMode.value = true
  editingRuleId.value = rule.id
  Object.assign(ruleForm, {
    name: rule.name,
    mode: rule.mode || 'regex',
    pattern: rule.pattern || '',
    keyword: rule.keyword || '',
    replacement: rule.replacement || '***',
    caseSensitive: rule.caseSensitive || false,
    description: rule.description || '',
    strategy: rule.strategy || 'full_mask',
    keepStart: rule.strategyConfig?.keepStart || 3,
    keepEnd: rule.strategyConfig?.keepEnd || 4,
    customText: rule.strategyConfig?.customText || ''
  })
  ruleDialogVisible.value = true
}

// 保存规则
async function saveRule() {
  try {
    await ruleFormRef.value.validate()
    
    // 如果是正则表达式模式，进行安全验证
    if (ruleForm.mode === 'regex' && ruleForm.pattern) {
      const validation = regexValidator.validate(ruleForm.pattern)
      
      if (!validation.isValid) {
        ElMessage.error(validation.errors.join('; '))
        return
      }
      
      // 显示警告信息
      if (validation.warnings.length > 0) {
        validation.warnings.forEach(warning => {
          ElMessage.warning(warning)
        })
        
        // 高复杂度需要用户确认
        if (validation.complexity === 'high') {
          try {
            await ElMessageBox.confirm(
              '正则表达式复杂度较高，可能影响性能。是否继续？',
              '性能警告',
              {
                confirmButtonText: '继续保存',
                cancelButtonText: '取消',
                type: 'warning'
              }
            )
          } catch {
            return  // 用户取消
          }
        }
      }
      
      // 显示优化建议
      const suggestions = regexValidator.getSuggestions(ruleForm.pattern)
      if (suggestions.length > 0 && !isEditMode.value) {
        ElMessage.info({
          message: '优化建议: ' + suggestions[0],
          duration: 5000
        })
      }
    }
    
    const ruleData = {
      name: ruleForm.name,
      mode: ruleForm.mode,
      // 正则模式字段
      pattern: ruleForm.mode === 'regex' ? ruleForm.pattern : '',
      // 关键字模式字段
      keyword: ruleForm.mode === 'keyword' ? ruleForm.keyword : '',
      replacement: ruleForm.mode === 'keyword' ? ruleForm.replacement : '***',
      caseSensitive: ruleForm.mode === 'keyword' ? ruleForm.caseSensitive : false,
      // 通用字段
      description: ruleForm.description,
      strategy: ruleForm.mode === 'regex' ? ruleForm.strategy : 'keyword_replace',
      strategyConfig: ruleForm.mode === 'regex' && ruleForm.strategy === 'partial_mask' 
        ? { keepStart: ruleForm.keepStart, keepEnd: ruleForm.keepEnd }
        : ruleForm.mode === 'regex' && ruleForm.strategy === 'custom'
        ? { customText: ruleForm.customText }
        : {}
    }
    
    if (isEditMode.value) {
      rulesStore.updateCustomRule(editingRuleId.value, ruleData)
      ElMessage.success('规则已更新')
    } else {
      rulesStore.addCustomRule(ruleData)
      ElMessage.success('规则已添加')
    }
    
    ruleDialogVisible.value = false
  } catch (error) {
    console.error('表单验证失败:', error)
  }
}

// 删除规则
function deleteRule(ruleId) {
  ElMessageBox.confirm('确定要删除这条规则吗？', '确认删除', {
    confirmButtonText: '删除',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    rulesStore.deleteCustomRule(ruleId)
    ElMessage.success('规则已删除')
  }).catch(() => {})
}

// 导入规则
async function importRules() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.json'
  input.onchange = async (e) => {
    const file = e.target.files[0]
    if (file) {
      try {
        const text = await file.text()
        
        // 验证是否是有效的 JSON
        let data
        try {
          data = JSON.parse(text)
        } catch (parseError) {
          throw new Error('文件不是有效的 JSON 格式，请确认选择的是规则导出文件')
        }
        
        // 检查数据格式
        if (!data || typeof data !== 'object') {
          throw new Error('规则文件格式不正确')
        }
        
        // 调用 store 的导入方法
        const success = rulesStore.importRules(data)
        if (success) {
          ElMessage.success('规则导入成功')
        }
      } catch (error) {
        console.error('导入规则失败:', error)
        ElMessage.error('导入规则失败: ' + error.message)
      }
    }
  }
  input.click()
}

// 导出规则
function exportRules() {
  try {
    const data = rulesStore.exportRules()
    const jsonStr = JSON.stringify(data, null, 2)
    const blob = new Blob([jsonStr], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    const timestamp = new Date().toISOString().slice(0, 10)
    a.download = `data-masker-rules-${timestamp}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
    ElMessage.success(`规则已导出到: ${a.download}`)
  } catch (error) {
    console.error('导出规则失败:', error)
    ElMessage.error('导出规则失败: ' + error.message)
  }
}

// 重置规则
function resetRules() {
  ElMessageBox.confirm('确定要重置所有规则为默认设置吗？自定义规则将被清除。', '确认重置', {
    confirmButtonText: '重置',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    rulesStore.resetRules()
  }).catch(() => {})
}

// 返回上一步
function goBack() {
  router.push('/file-select')
}

// 下一步
function goNext() {
  if (enabledRulesCount.value === 0) {
    ElMessage.warning('请至少启用一条脱敏规则')
    return
  }
  router.push('/preview')
}
</script>

<style lang="scss" scoped>
.rule-config-page {
  max-width: 1200px;
  margin: 0 auto;
}

.config-tabs {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
}

.rule-categories {
  :deep(.el-collapse) {
    border: none;
  }
  
  :deep(.el-collapse-item__header) {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    background: transparent;
    border-bottom: 1px solid #ebeef5;
  }
  
  :deep(.el-collapse-item__content) {
    padding: 20px 0;
  }
}

.rule-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.custom-rules-section {
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    
    .import-export {
      display: flex;
      gap: 12px;
    }
  }
}

.rule-stats {
  display: flex;
  gap: 32px;
  margin-top: 24px;
  padding: 16px 24px;
  background: #f5f7fa;
  border-radius: 8px;
  
  .stat-item {
    display: flex;
    align-items: center;
    gap: 8px;
    color: #606266;
  }
}

.action-bar {
  display: flex;
  justify-content: space-between;
  margin-top: 32px;
  padding-top: 24px;
  border-top: 1px solid #ebeef5;
}

.form-tip {
  font-size: 12px;
  color: #909399;
  margin-top: 8px;
}

.keep-digits {
  display: flex;
  align-items: center;
  gap: 8px;
  
  span {
    color: #606266;
  }
}

.switch-label {
  margin-left: 8px;
  color: #909399;
  font-size: 12px;
}
</style>
