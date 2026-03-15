/**
 * 安全存储工具
 * 使用 AES 加密敏感数据后再存储到 localStorage
 * 
 * 产品设计: wonder-宏
 * 架构设计/开发实现: JARVIS AI Assistant
 */

class SecureStorage {
  constructor() {
    this.algorithm = 'AES-GCM'
    this.keyLength = 256
    this.ivLength = 12
    this.saltLength = 16
    this.key = null
  }

  /**
   * 生成加密密钥
   * 使用设备指纹作为密钥材料,增加安全性
   */
  async generateKey() {
    // 使用设备指纹生成密钥
    const fingerprint = await this.getDeviceFingerprint()
    const encoder = new TextEncoder()
    const keyMaterial = await crypto.subtle.importKey(
      'raw',
      encoder.encode(fingerprint),
      'PBKDF2',
      false,
      ['deriveBits', 'deriveKey']
    )

    // 使用固定盐值(实际应用中应该随机生成并存储)
    const salt = encoder.encode('data-masker-salt-v1')
    
    this.key = await crypto.subtle.deriveKey(
      {
        name: 'PBKDF2',
        salt: salt,
        iterations: 100000,
        hash: 'SHA-256'
      },
      keyMaterial,
      { name: this.algorithm, length: this.keyLength },
      false,
      ['encrypt', 'decrypt']
    )

    return this.key
  }

  /**
   * 获取设备指纹
   * 结合多个浏览器特性生成唯一标识
   */
  async getDeviceFingerprint() {
    const components = [
      navigator.userAgent,
      navigator.language,
      screen.width + 'x' + screen.height,
      new Date().getTimezoneOffset(),
      navigator.hardwareConcurrency || 'unknown',
      navigator.deviceMemory || 'unknown'
    ]
    
    const fingerprint = components.join('|')
    const encoder = new TextEncoder()
    const data = encoder.encode(fingerprint)
    const hashBuffer = await crypto.subtle.digest('SHA-256', data)
    const hashArray = Array.from(new Uint8Array(hashBuffer))
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
  }

  /**
   * 加密数据
   */
  async encrypt(plaintext) {
    if (!this.key) {
      await this.generateKey()
    }

    const encoder = new TextEncoder()
    const data = encoder.encode(plaintext)
    
    // 生成随机 IV
    const iv = crypto.getRandomValues(new Uint8Array(this.ivLength))
    
    // 加密
    const encrypted = await crypto.subtle.encrypt(
      {
        name: this.algorithm,
        iv: iv
      },
      this.key,
      data
    )

    // 组合 IV 和加密数据
    const combined = new Uint8Array(iv.length + encrypted.byteLength)
    combined.set(iv)
    combined.set(new Uint8Array(encrypted), iv.length)

    // 转换为 Base64
    return btoa(String.fromCharCode(...combined))
  }

  /**
   * 解密数据
   */
  async decrypt(ciphertext) {
    if (!this.key) {
      await this.generateKey()
    }

    try {
      // Base64 解码
      const combined = new Uint8Array(
        atob(ciphertext).split('').map(c => c.charCodeAt(0))
      )

      // 提取 IV 和加密数据
      const iv = combined.slice(0, this.ivLength)
      const encrypted = combined.slice(this.ivLength)

      // 解密
      const decrypted = await crypto.subtle.decrypt(
        {
          name: this.algorithm,
          iv: iv
        },
        this.key,
        encrypted
      )

      const decoder = new TextDecoder()
      return decoder.decode(decrypted)
    } catch (error) {
      console.error('解密失败:', error)
      return null
    }
  }

  /**
   * 安全存储数据
   */
  async setItem(key, value, encrypt = false) {
    try {
      const dataToStore = encrypt ? await this.encrypt(value) : value
      localStorage.setItem(key, dataToStore)
      return true
    } catch (error) {
      console.error('存储失败:', error)
      return false
    }
  }

  /**
   * 安全读取数据
   */
  async getItem(key, encrypted = false) {
    try {
      const value = localStorage.getItem(key)
      if (value === null) return null
      
      return encrypted ? await this.decrypt(value) : value
    } catch (error) {
      console.error('读取失败:', error)
      return null
    }
  }

  /**
   * 删除数据
   */
  removeItem(key) {
    localStorage.removeItem(key)
  }

  /**
   * 检查密码强度
   * @param {string} password - 密码
   * @returns {object} - 强度评估结果
   */
  checkPasswordStrength(password) {
    const result = {
      score: 0,
      level: 'weak',
      suggestions: [],
      isValid: false
    }

    if (!password || password.length < 8) {
      result.suggestions.push('密码长度至少8位')
      return result
    }

    // 长度得分
    if (password.length >= 8) result.score += 1
    if (password.length >= 12) result.score += 1
    if (password.length >= 16) result.score += 1

    // 复杂度得分
    if (/[a-z]/.test(password)) result.score += 1
    if (/[A-Z]/.test(password)) result.score += 1
    if (/[0-9]/.test(password)) result.score += 1
    if (/[^a-zA-Z0-9]/.test(password)) result.score += 1

    // 避免常见模式
    if (/(.)\1{2,}/.test(password)) {
      result.score -= 1
      result.suggestions.push('避免连续重复字符')
    }
    
    if (/^[a-zA-Z]+$/.test(password) || /^[0-9]+$/.test(password)) {
      result.score -= 1
      result.suggestions.push('不要只使用字母或数字')
    }

    // 计算等级
    if (result.score >= 6) {
      result.level = 'strong'
      result.isValid = true
    } else if (result.score >= 4) {
      result.level = 'medium'
      result.isValid = true
    } else {
      result.level = 'weak'
      result.isValid = false
      if (result.suggestions.length === 0) {
        result.suggestions.push('建议使用大小写字母、数字和特殊字符的组合')
      }
    }

    return result
  }

  /**
   * 清除密钥
   */
  clearKey() {
    this.key = null
  }
}

// 导出单例
export const secureStorage = new SecureStorage()

// 导出类
export default SecureStorage
