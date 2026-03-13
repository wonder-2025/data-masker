// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

/**
 * 格式化工具函数
 */

/**
 * 格式化文件大小
 * @param {number} bytes - 字节数
 * @returns {string} 格式化后的字符串
 */
export function formatFileSize(bytes) {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

/**
 * 格式化日期时间
 * @param {string|Date} date - 日期
 * @param {string} format - 格式 ('full' | 'date' | 'time')
 * @returns {string} 格式化后的字符串
 */
export function formatDate(date, format = 'full') {
  if (!date) return '-'
  
  const d = new Date(date)
  
  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const hours = String(d.getHours()).padStart(2, '0')
  const minutes = String(d.getMinutes()).padStart(2, '0')
  const seconds = String(d.getSeconds()).padStart(2, '0')
  
  switch (format) {
    case 'date':
      return `${year}-${month}-${day}`
    case 'time':
      return `${hours}:${minutes}:${seconds}`
    case 'full':
    default:
      return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
  }
}

/**
 * 格式化相对时间
 * @param {string|Date} date - 日期
 * @returns {string} 相对时间描述
 */
export function formatRelativeTime(date) {
  if (!date) return '-'
  
  const now = new Date()
  const d = new Date(date)
  const diff = now - d
  
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)
  
  if (seconds < 60) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 7) return `${days}天前`
  
  return formatDate(date, 'date')
}

/**
 * 格式化持续时间
 * @param {number} ms - 毫秒数
 * @returns {string} 格式化后的字符串
 */
export function formatDuration(ms) {
  if (ms < 1000) return `${ms}ms`
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
  
  const minutes = Math.floor(ms / 60000)
  const seconds = Math.floor((ms % 60000) / 1000)
  
  return `${minutes}m ${seconds}s`
}

/**
 * 格式化数字（千分位）
 * @param {number} num - 数字
 * @returns {string} 格式化后的字符串
 */
export function formatNumber(num) {
  if (num === null || num === undefined) return '-'
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

/**
 * 格式化百分比
 * @param {number} value - 值
 * @param {number} total - 总数
 * @param {number} decimals - 小数位数
 * @returns {string} 百分比字符串
 */
export function formatPercent(value, total, decimals = 1) {
  if (total === 0) return '0%'
  return ((value / total) * 100).toFixed(decimals) + '%'
}

/**
 * 获取文件扩展名
 * @param {string} filename - 文件名
 * @returns {string} 扩展名
 */
export function getFileExtension(filename) {
  if (!filename) return ''
  const ext = filename.split('.').pop()?.toLowerCase()
  return ext || ''
}

/**
 * 获取文件类型图标
 * @param {string} type - 文件类型
 * @returns {Object} 图标信息 { icon, color }
 */
export function getFileIcon(type) {
  const icons = {
    pdf: { icon: 'Document', color: '#F56C6C' },
    docx: { icon: 'Document', color: '#409EFF' },
    doc: { icon: 'Document', color: '#409EFF' },
    xlsx: { icon: 'Document', color: '#67C23A' },
    xls: { icon: 'Document', color: '#67C23A' },
    pptx: { icon: 'Document', color: '#E6A23C' },
    txt: { icon: 'Tickets', color: '#909399' },
    md: { icon: 'Tickets', color: '#909399' },
    csv: { icon: 'Tickets', color: '#E6A23C' },
    json: { icon: 'Tickets', color: '#9C27B0' },
    xml: { icon: 'Tickets', color: '#9C27B0' },
    default: { icon: 'Document', color: '#909399' }
  }
  
  return icons[type?.toLowerCase()] || icons.default
}

/**
 * 获取敏感信息类型标签
 * @param {string} type - 类型
 * @returns {string} 中文标签
 */
export function getSensitiveTypeLabel(type) {
  const labels = {
    id_card: '身份证号',
    phone: '手机号',
    bank_card: '银行卡号',
    passport: '护照号',
    hk_mo_pass: '港澳通行证',
    tw_pass: '台湾通行证',
    credit_code: '统一社会信用代码',
    email: '邮箱',
    license_plate: '车牌号',
    ipv4: 'IPv4地址',
    ipv6: 'IPv6地址',
    mac: 'MAC地址',
    api_key: 'API密钥',
    name: '姓名',
    company: '公司名称',
    address: '地址',
    amount: '金额',
    date: '日期',
    url: 'URL',
    telephone: '电话号码',
    custom: '自定义'
  }
  
  return labels[type] || type
}

/**
 * 获取脱敏策略标签
 * @param {string} strategy - 策略
 * @returns {string} 中文标签
 */
export function getStrategyLabel(strategy) {
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
