// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// Tauri API 封装工具

/**
 * 带超时的 invoke 调用
 * @param {string} cmd - 命令名称
 * @param {Object} args - 命令参数
 * @param {number} timeout - 超时时间（毫秒），默认 60000ms
 * @returns {Promise<any>} 命令结果
 */
export async function invokeWithTimeout(cmd, args = {}, timeout = 60000) {
  const { invoke } = await import('@tauri-apps/api/core')
  
  return new Promise((resolve, reject) => {
    // 创建超时定时器
    const timeoutId = setTimeout(() => {
      reject(new Error(`命令 "${cmd}" 执行超时 (${timeout}ms)`))
    }, timeout)
    
    // 执行命令
    invoke(cmd, args)
      .then((result) => {
        clearTimeout(timeoutId)
        resolve(result)
      })
      .catch((error) => {
        clearTimeout(timeoutId)
        reject(error)
      })
  })
}

/**
 * 批量执行命令
 * @param {Array<{cmd: string, args?: Object}>} cmds - 命令数组
 * @param {Object} options - 选项
 * @param {boolean} options.stopOnError - 遇到错误时停止，默认 true
 * @param {number} options.concurrency - 并发数量，默认 1（串行）
 * @returns {Promise<{results: Array, errors: Array}>} 结果和错误数组
 */
export async function invokeBatch(cmds, options = {}) {
  const { stopOnError = true, concurrency = 1 } = options
  const results = []
  const errors = []
  
  const { invoke } = await import('@tauri-apps/api/core')
  
  if (concurrency === 1) {
    // 串行执行
    for (let i = 0; i < cmds.length; i++) {
      const { cmd, args = {} } = cmds[i]
      try {
        const result = await invoke(cmd, args)
        results.push({ index: i, cmd, result })
      } catch (error) {
        errors.push({ index: i, cmd, error })
        if (stopOnError) {
          break
        }
      }
    }
  } else {
    // 并发执行
    const chunks = []
    for (let i = 0; i < cmds.length; i += concurrency) {
      chunks.push(cmds.slice(i, i + concurrency))
    }
    
    for (const chunk of chunks) {
      const promises = chunk.map(async ({ cmd, args = {} }, idx) => {
        const index = cmds.indexOf(chunk[idx])
        try {
          const result = await invoke(cmd, args)
          return { index, cmd, result }
        } catch (error) {
          return { index, cmd, error }
        }
      })
      
      const chunkResults = await Promise.all(promises)
      for (const item of chunkResults) {
        if (item.error) {
          errors.push({ index: item.index, cmd: item.cmd, error: item.error })
          if (stopOnError) {
            return { results, errors }
          }
        } else {
          results.push(item)
        }
      }
    }
  }
  
  return { results, errors }
}

/**
 * 带重试的 invoke 调用
 * @param {string} cmd - 命令名称
 * @param {Object} args - 命令参数
 * @param {number} maxRetries - 最大重试次数，默认 3
 * @param {number} retryDelay - 重试间隔（毫秒），默认 1000ms
 * @returns {Promise<any>} 命令结果
 */
export async function invokeWithRetry(cmd, args = {}, maxRetries = 3, retryDelay = 1000) {
  const { invoke } = await import('@tauri-apps/api/core')
  
  let lastError = null
  
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const result = await invoke(cmd, args)
      return result
    } catch (error) {
      lastError = error
      
      if (attempt < maxRetries) {
        // 等待后重试
        await new Promise(resolve => setTimeout(resolve, retryDelay))
        console.warn(`命令 "${cmd}" 执行失败，第 ${attempt} 次重试...`)
      }
    }
  }
  
  throw new Error(`命令 "${cmd}" 执行失败，已重试 ${maxRetries} 次: ${lastError?.message || lastError}`)
}

/**
 * 带超时和重试的 invoke 调用（组合）
 * @param {string} cmd - 命令名称
 * @param {Object} args - 命令参数
 * @param {Object} options - 选项
 * @param {number} options.timeout - 超时时间（毫秒）
 * @param {number} options.maxRetries - 最大重试次数
 * @param {number} options.retryDelay - 重试间隔（毫秒）
 * @returns {Promise<any>} 命令结果
 */
export async function invokeWithTimeoutAndRetry(cmd, args = {}, options = {}) {
  const { timeout = 60000, maxRetries = 3, retryDelay = 1000 } = options
  
  let lastError = null
  
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await invokeWithTimeout(cmd, args, timeout)
    } catch (error) {
      lastError = error
      
      if (attempt < maxRetries) {
        await new Promise(resolve => setTimeout(resolve, retryDelay))
        console.warn(`命令 "${cmd}" 超时或失败，第 ${attempt} 次重试...`)
      }
    }
  }
  
  throw lastError
}
