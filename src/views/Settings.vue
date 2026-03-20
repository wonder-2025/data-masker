// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

<template>
  <div class="settings-page">
    <div class="page-header">
      <h1 class="page-title">设置</h1>
      <p class="page-desc">配置应用程序的各项设置</p>
    </div>
    
    <el-tabs v-model="activeTab" class="settings-tabs" tab-position="left">
      <!-- 通用设置 -->
      <el-tab-pane label="通用设置" name="general">
        <div class="settings-section">
          <h3>界面设置</h3>
          <el-form label-width="120px">
            <el-form-item label="语言">
              <el-select v-model="settingsStore.settingsData.general.language" style="width: 200px;">
                <el-option label="简体中文" value="zh-CN" />
                <el-option label="English" value="en-US" />
              </el-select>
            </el-form-item>
          </el-form>
          
          <el-divider />
          
          <h3>输出设置</h3>
          <el-form label-width="120px">
            <el-form-item label="默认输出目录">
              <el-input v-model="settingsStore.settingsData.general.outputDir" style="width: 300px;">
                <template #append>
                  <el-button @click="selectOutputDir">浏览</el-button>
                </template>
              </el-input>
              <div class="form-tip">留空则使用系统默认下载目录</div>
            </el-form-item>
            
            <el-form-item label="自动打开输出">
              <el-switch v-model="settingsStore.settingsData.general.autoOpenOutput" />
              <div class="form-tip">处理完成后自动打开输出目录</div>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
      
      <!-- 脱敏设置 -->
      <el-tab-pane label="脱敏设置" name="masking">
        <div class="settings-section">
          <h3>默认脱敏策略</h3>
          <el-form label-width="140px">
            <el-form-item label="默认策略">
              <el-select v-model="settingsStore.settingsData.masking.defaultStrategy" style="width: 200px;">
                <el-option label="完全隐藏" value="full_mask" />
                <el-option label="部分掩码" value="partial_mask" />
                <el-option label="假数据替换" value="fake_data" />
                <el-option label="可逆加密" value="reversible" />
                <el-option label="哈希脱敏" value="hash" />
              </el-select>
            </el-form-item>
            
            <el-form-item label="保留前几位" v-if="settingsStore.settingsData.masking.defaultStrategy === 'partial_mask'">
              <el-input-number v-model="settingsStore.settingsData.masking.keepStartDigits" :min="0" :max="20" />
            </el-form-item>
            
            <el-form-item label="保留后几位" v-if="settingsStore.settingsData.masking.defaultStrategy === 'partial_mask'">
              <el-input-number v-model="settingsStore.settingsData.masking.keepEndDigits" :min="0" :max="20" />
            </el-form-item>
            
            <el-form-item label="脱敏字符">
              <el-input v-model="settingsStore.settingsData.masking.maskChar" style="width: 60px;" maxlength="1" />
              <div class="form-tip">用于替换敏感信息的字符，默认为 *</div>
            </el-form-item>
          </el-form>
          
          <el-divider />
          
          <h3>假数据生成</h3>
          <el-form label-width="140px">
            <el-form-item label="数据语言">
              <el-select v-model="settingsStore.settingsData.masking.fakeDataLocale" style="width: 200px;">
                <el-option label="中文" value="zh-CN" />
                <el-option label="English" value="en-US" />
              </el-select>
              <div class="form-tip">生成假数据的语言风格</div>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
      
      <!-- 安全设置 -->
      <el-tab-pane label="安全设置" name="security">
        <div class="settings-section">
          <h3>密码保护</h3>
          <el-form label-width="140px">
            <el-form-item label="启用密码保护">
              <el-switch v-model="settingsStore.settingsData.security.passwordProtect" />
              <div class="form-tip">启用后需要输入密码才能打开应用</div>
            </el-form-item>
            
            <el-form-item label="设置密码" v-if="settingsStore.settingsData.security.passwordProtect">
              <el-input
                v-model="settingsStore.settingsData.security.password"
                type="password"
                show-password
                style="width: 200px;"
                placeholder="请输入密码"
              />
            </el-form-item>
          </el-form>
          
          <el-divider />
          
          <h3>临时文件管理</h3>
          <el-form label-width="140px">
            <el-form-item label="自动清理">
              <el-switch v-model="settingsStore.settingsData.security.autoCleanTemp" />
              <div class="form-tip">自动清理处理过程中产生的临时文件</div>
            </el-form-item>
            
            <el-form-item label="清理时间" v-if="settingsStore.settingsData.security.autoCleanTemp">
              <el-input-number v-model="settingsStore.settingsData.security.cleanAfter" :min="1" :max="1440" />
              <span style="margin-left: 8px;">分钟后清理</span>
            </el-form-item>
            
            <el-form-item label="加密映射表">
              <el-switch v-model="settingsStore.settingsData.security.encryptMapping" />
              <div class="form-tip">使用AES-256加密存储可逆脱敏的映射表</div>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
      
      <!-- 高级设置 -->
      <el-tab-pane label="高级设置" name="advanced">
        <div class="settings-section">
          <h3>性能设置</h3>
          <el-form label-width="140px">
            <el-form-item label="最大文件大小">
              <el-input-number v-model="settingsStore.settingsData.advanced.maxFileSize" :min="1" :max="1000" />
              <span style="margin-left: 8px;">MB</span>
              <div class="form-tip">单个文件的最大处理大小限制</div>
            </el-form-item>
            
            <el-form-item label="并发文件数">
              <el-slider v-model="settingsStore.settingsData.advanced.concurrentFiles" :min="1" :max="10" show-input />
              <div class="form-tip">同时处理的文件数量</div>
            </el-form-item>
          </el-form>
          
          <el-divider />

          <h3>日志设置</h3>
          <el-form label-width="140px">
            <el-form-item label="日志级别">
              <el-select v-model="settingsStore.settingsData.advanced.logLevel" style="width: 200px;">
                <el-option label="调试 (Debug)" value="debug" />
                <el-option label="信息 (Info)" value="info" />
                <el-option label="警告 (Warn)" value="warn" />
                <el-option label="错误 (Error)" value="error" />
              </el-select>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
      
      <!-- 日志收集 -->
      <el-tab-pane label="日志收集" name="errorReport">
        <div class="settings-section">
          <h3>使用数据收集</h3>
          <el-alert type="info" :closable="false" style="margin-bottom: 20px;">
            启用后，应用会收集使用数据（操作日志、错误日志等），帮助开发者了解用户需求并优化功能。
          </el-alert>

          <el-form label-width="140px">
            <el-form-item label="启用日志收集">
              <el-switch v-model="settingsStore.settingsData.errorReport.enabled" @change="handleLogCollectorChange" />
              <div class="form-tip">启用后，自动收集使用数据用于优化</div>
            </el-form-item>

            <el-form-item label="服务器地址" v-if="settingsStore.settingsData.errorReport.enabled">
              <el-input
                v-model="settingsStore.settingsData.errorReport.serverUrl"
                placeholder="http://server:port/api/data-masker/logs"
                style="width: 400px;"
              />
              <div class="form-tip">日志收集服务地址</div>
            </el-form-item>

            <el-form-item v-if="settingsStore.settingsData.errorReport.enabled">
              <el-button @click="testConnection" :loading="testingConnection">测试连接</el-button>
              <span v-if="connectionStatus" :style="{ color: connectionStatus === '成功' ? '#67C23A' : '#F56C6C', marginLeft: '10px' }">
                {{ connectionStatus }}
              </span>
            </el-form-item>
            
            <!-- 测试结果详情 -->
            <el-form-item v-if="testResult" label="测试结果">
              <el-card class="test-result-card" shadow="never">
                <div class="result-header">
                  <el-tag :type="testResult.ok ? 'success' : 'danger'" size="small">
                    HTTP {{ testResult.status }}
                  </el-tag>
                </div>
                <pre class="result-body">{{ testResult.body }}</pre>
              </el-card>
            </el-form-item>
          </el-form>

          <el-divider />

          <h3>收集的数据类型</h3>
          <el-form label-width="140px">
            <el-form-item label="错误日志">
              <el-switch v-model="settingsStore.settingsData.errorReport.collectErrors" />
              <div class="form-tip">JS异常、API失败等错误信息</div>
            </el-form-item>
            
            <el-form-item label="操作日志">
              <el-switch v-model="settingsStore.settingsData.errorReport.collectOperations" />
              <div class="form-tip">文件选择、处理、导出等操作</div>
            </el-form-item>
            
            <el-form-item label="行为分析">
              <el-switch v-model="settingsStore.settingsData.errorReport.collectAnalytics" />
              <div class="form-tip">页面访问、功能使用频率</div>
            </el-form-item>
          </el-form>

          <el-divider />

          <h3>数据隐私</h3>
          <div class="privacy-note">
            <el-icon :size="16" color="#67C23A"><CircleCheckFilled /></el-icon>
            <span>所有日志自动脱敏：文件路径、手机号、身份证等敏感信息会被替换</span>
          </div>
        </div>
      </el-tab-pane>

      <!-- 关于 -->
      <el-tab-pane label="关于" name="about">
        <div class="about-section">
          <div class="app-logo">
            <el-icon :size="64" color="#409EFF"><Shield /></el-icon>
          </div>
          
          <h2>Data Masker</h2>
          <p class="version">版本 1.0.0</p>
          
          <el-divider />
          
          <div class="credits">
            <h3>设计团队</h3>
            <div class="credit-item">
              <span class="role">产品设计:</span>
              <span class="name">wonder-宏</span>
            </div>
            <div class="credit-item">
              <span class="role">架构设计/开发实现:</span>
              <span class="name">JARVIS AI Assistant</span>
            </div>
          </div>
          
          <el-divider />
          
          <div class="license">
            <p>本软件为开源项目，采用 MIT 许可证。</p>
            <p>所有数据处理均在本地完成，敏感数据不会上传到云端。</p>
          </div>
        </div>
      </el-tab-pane>
    </el-tabs>
    
    <!-- 底部操作栏 -->
    <div class="action-bar">
      <el-button @click="resetSettings">重置为默认</el-button>
      <el-button type="primary" @click="saveSettings">保存设置</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { ElMessage } from 'element-plus'
import { logCollector } from '@/utils/logCollector'

const settingsStore = useSettingsStore()

const activeTab = ref('general')
const testingConnection = ref(false)
const connectionStatus = ref('')
const testResult = ref(null)

// 日志收集开关变化
function handleLogCollectorChange(enabled) {
  logCollector.updateConfig({
    enabled,
    serverUrl: settingsStore.settingsData.errorReport.serverUrl
  })
  if (enabled) {
    ElMessage.success('日志收集已启用')
  } else {
    ElMessage.info('日志收集已禁用')
  }
}

// 测试连接
async function testConnection() {
  testingConnection.value = true
  connectionStatus.value = ''
  testResult.value = null

  try {
    const response = await fetch(settingsStore.settingsData.errorReport.serverUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        app_name: 'data-masker',
        version: '1.0.0',
        os: navigator.platform,
        error_type: 'TestConnection',
        error_message: '测试连接',
        user_description: '这是一条测试消息'
      })
    })

    // 获取响应内容
    const responseText = await response.text()
    let responseBody
    try {
      responseBody = JSON.stringify(JSON.parse(responseText), null, 2)
    } catch {
      responseBody = responseText
    }

    // 保存测试结果
    testResult.value = {
      ok: response.ok,
      status: response.status,
      body: responseBody
    }

    if (response.ok) {
      connectionStatus.value = '成功'
      ElMessage.success('连接测试成功')
    } else {
      connectionStatus.value = '失败'
      ElMessage.error(`连接测试失败 (HTTP ${response.status})`)
    }
  } catch (error) {
    connectionStatus.value = '失败'
    testResult.value = {
      ok: false,
      status: 'Error',
      body: `连接失败: ${error.message}\n\n可能的原因:\n1. 服务器地址不正确\n2. 服务器未运行\n3. 网络连接问题\n4. CORS 跨域限制`
    }
    ElMessage.error('连接失败: ' + error.message)
  } finally {
    testingConnection.value = false
  }
}

// 选择输出目录
async function selectOutputDir() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const selected = await invoke('select_directory')
    if (selected) {
      settingsStore.settingsData.general.outputDir = selected
      // 立即保存设置，不等待防抖
      settingsStore.saveSettings()
      console.log('[Settings] 输出目录已更新并保存:', selected)
    }
  } catch (error) {
    console.error('选择目录失败:', error)
  }
}

// 重置设置
function resetSettings() {
  settingsStore.resetToDefault()
  ElMessage.success('设置已重置为默认值')
}

// 保存设置
function saveSettings() {
  settingsStore.saveSettings()
  ElMessage.success('设置已保存')
}
</script>

<style lang="scss" scoped>
.settings-page {
  max-width: 1000px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 16px;
  
  .page-title {
    font-size: 20px;
    font-weight: 700;
    margin: 0;
  }
  
  .page-desc {
    font-size: 12px;
    color: #909399;
    margin: 4px 0 0;
  }
}

.settings-tabs {
  background: #fff;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.06);
  
  :deep(.el-tabs__content) {
    padding-left: 16px;
  }
}

.settings-section {
  h3 {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 16px;
  }
}

.form-tip {
  font-size: 11px;
  color: #909399;
  margin-top: 4px;
}

.about-section {
  text-align: center;
  padding: 24px 0;
  
  .app-logo {
    margin-bottom: 12px;
  }
  
  h2 {
    font-size: 24px;
    font-weight: 700;
    margin: 0 0 8px;
  }
  
  .version {
    font-size: 12px;
    color: #909399;
    margin: 0;
  }
  
  .credits {
    .credit-item {
      display: flex;
      justify-content: center;
      gap: 8px;
      margin: 8px 0;
      
      .role {
        color: #909399;
      }
      
      .name {
        font-weight: 500;
      }
    }
  }
  
  .license {
    p {
      font-size: 12px;
      color: #909399;
      margin: 8px 0;
    }
  }
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}

.privacy-note {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #E6A23C;
  font-size: 12px;
}

.test-result-card {
  width: 100%;
  max-width: 500px;
  
  .result-header {
    margin-bottom: 8px;
  }
  
  .result-body {
    background: #f5f7fa;
    padding: 12px;
    border-radius: 4px;
    font-size: 12px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-all;
    margin: 0;
    max-height: 200px;
    overflow-y: auto;
  }
}
</style>
