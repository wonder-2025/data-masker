// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 加密服务
//! 
//! 使用 AES-256-GCM 加密映射表
//! 密钥由用户密码通过 PBKDF2 派生

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{Engine as _, engine::general_purpose};

/// 加密器
pub struct Encryptor {
    key: [u8; 32],
}

impl Encryptor {
    /// 从密码创建加密器
    pub fn from_password(password: &str, salt: &[u8]) -> Self {
        use sha2::Sha256;
        
        // 使用 PBKDF2 派生密钥
        let mut key = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            salt,
            100_000,  // 迭代次数
            &mut key,
        );
        
        Encryptor { key }
    }
    
    /// 加密数据
    pub fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("创建加密器失败: {}", e))?;
        
        // 生成随机 nonce
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 加密
        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("加密失败: {}", e))?;
        
        // 组合: base64(nonce || ciphertext)
        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);
        
        Ok(general_purpose::STANDARD.encode(&combined))
    }
    
    /// 解密数据
    pub fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("创建解密器失败: {}", e))?;
        
        // 解码 base64
        let combined = general_purpose::STANDARD.decode(encrypted)
            .map_err(|e| format!("Base64解码失败: {}", e))?;
        
        if combined.len() < 12 {
            return Err("无效的加密数据".to_string());
        }
        
        // 分离 nonce 和密文
        let nonce = Nonce::from_slice(&combined[..12]);
        let ciphertext = &combined[12..];
        
        // 解密
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| "解密失败，密码可能不正确".to_string())?;
        
        String::from_utf8(plaintext)
            .map_err(|e| format!("解码明文失败: {}", e))
    }
    
    /// 生成随机盐值
    pub fn generate_salt() -> [u8; 16] {
        rand::random()
    }
}

/// 哈希工具
pub struct Hasher;

impl Hasher {
    /// SHA256 哈希
    pub fn sha256(data: &str) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        
        hex::encode(result)
    }
    
    /// SHA256 哈希（前N位）
    pub fn sha256_prefix(data: &str, prefix_len: usize) -> String {
        let hash = Self::sha256(data);
        hash[..prefix_len.min(hash.len())].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let salt = Encryptor::generate_salt();
        let encryptor = Encryptor::from_password("test_password", &salt);
        
        let original = "这是敏感信息";
        let encrypted = encryptor.encrypt(original).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }
    
    #[test]
    fn test_hash() {
        let hash = Hasher::sha256("test");
        assert_eq!(hash.len(), 64);  // SHA256 输出 32 字节 = 64 十六进制字符
        
        let prefix = Hasher::sha256_prefix("test", 8);
        assert_eq!(prefix.len(), 8);
    }
}
