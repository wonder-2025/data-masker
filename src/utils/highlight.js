// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant

/**
 * 高亮工具函数
 */

/**
 * 高亮文本中的敏感信息
 * @param {string} text - 原始文本
 * @param {Array} sensitiveList - 敏感信息列表
 * @returns {string} 高亮后的HTML
 */
export function highlightSensitive(text, sensitiveList) {
  if (!text || !sensitiveList || sensitiveList.length === 0) {
    return escapeHtml(text)
  }
  
  let html = escapeHtml(text)
  
  // 从后往前替换，避免位置偏移
  const sorted = [...sensitiveList].sort((a, b) => b.start - a.start)
  
  sorted.forEach(item => {
    const before = html.substring(0, item.start)
    const sensitive = html.substring(item.start, item.end)
    const after = html.substring(item.end)
    
    html = before + `<span class="highlight-sensitive" title="${item.type}">${sensitive}</span>` + after
  })
  
  return html
}

/**
 * 高亮脱敏后的文本
 * @param {string} text - 脱敏后文本
 * @param {Array} sensitiveList - 敏感信息列表
 * @returns {string} 高亮后的HTML
 */
export function highlightMasked(text, sensitiveList) {
  if (!text || !sensitiveList || sensitiveList.length === 0) {
    return escapeHtml(text)
  }
  
  let html = escapeHtml(text)
  
  const sorted = [...sensitiveList].sort((a, b) => b.start - a.start)
  
  sorted.forEach(item => {
    const before = html.substring(0, item.start)
    const masked = html.substring(item.start, item.end)
    const after = html.substring(item.end)
    
    html = before + `<span class="highlight-masked" title="原: ${item.original}">${masked}</span>` + after
  })
  
  return html
}

/**
 * HTML 转义
 * @param {string} text - 原始文本
 * @returns {string} 转义后的文本
 */
export function escapeHtml(text) {
  if (!text) return ''
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

/**
 * 计算文本位置
 * @param {string} text - 文本内容
 * @param {number} offset - 字符偏移量
 * @returns {Object} { line, column }
 */
export function calculatePosition(text, offset) {
  let line = 1
  let column = 1
  
  for (let i = 0; i < offset && i < text.length; i++) {
    if (text[i] === '\n') {
      line++
      column = 1
    } else {
      column++
    }
  }
  
  return { line, column }
}
