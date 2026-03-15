// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 脱敏处理引擎
//! 
//! 支持多种脱敏策略，确保格式无损

use crate::services::detector::Detection;

/// 脱敏处理器
pub struct Masker {
    mask_char: String,
}

impl Masker {
    /// 创建新的脱敏处理器
    pub fn new() -> Self {
        Masker {
            mask_char: "*".to_string(),
        }
    }
    
    /// 对内容应用脱敏
    pub fn mask_content(&self, content: &str, detections: &[Detection]) -> String {
        if detections.is_empty() {
            return content.to_string();
        }
        
        // 从后往前替换，避免位置偏移
        let mut result = content.to_string();
        let sorted_detections: Vec<_> = detections.iter()
            .collect();
        
        // 按位置倒序排列
        let mut sorted = sorted_detections.clone();
        sorted.sort_by(|a, b| b.start.cmp(&a.start));
        
        for detection in sorted {
            // 验证索引在有效范围内
            if detection.start <= result.len() && detection.end <= result.len() {
                // 验证边界是有效的 UTF-8 边界
                if result.is_char_boundary(detection.start) && result.is_char_boundary(detection.end) {
                    result.replace_range(detection.start..detection.end, &detection.masked);
                } else {
                    tracing::warn!("Invalid UTF-8 boundary in detection: {:?}", detection);
                }
            }
        }
        
        result
    }
    
    /// 应用脱敏策略
    pub fn apply_mask(&self, content: &str, detections: &[Detection], strategy: &str) -> String {
        match strategy {
            "full_mask" => self.apply_full_mask(content, detections),
            "partial_mask" => self.apply_partial_mask(content, detections),
            "hash" => self.apply_hash_mask(content, detections),
            _ => self.mask_content(content, detections),
        }
    }
    
    /// 完全隐藏
    fn apply_full_mask(&self, content: &str, detections: &[Detection]) -> String {
        let mut result = content.to_string();
        
        for detection in detections.iter().rev() {
            let mask = self.mask_char.repeat(detection.original.chars().count());
            if detection.start <= result.len() && detection.end <= result.len() {
                result.replace_range(detection.start..detection.end, &mask);
            }
        }
        
        result
    }
    
    /// 部分掩码
    fn apply_partial_mask(&self, content: &str, detections: &[Detection]) -> String {
        self.mask_content(content, detections)
    }
    
    /// 哈希脱敏
    fn apply_hash_mask(&self, content: &str, detections: &[Detection]) -> String {
        use sha2::{Sha256, Digest};
        
        let mut result = content.to_string();
        
        for detection in detections.iter().rev() {
            let mut hasher = Sha256::new();
            hasher.update(detection.original.as_bytes());
            let hash = hasher.finalize();
            let hash_str = hex::encode(&hash[..4]); // 前8位
            
            if detection.start <= result.len() && detection.end <= result.len() {
                result.replace_range(detection.start..detection.end, &hash_str);
            }
        }
        
        result
    }
    
    /// 假数据替换（生成格式相似但不同的数据）
    #[allow(dead_code)]
    pub fn generate_fake_data(&self, info_type: &str, original: &str) -> String {
        match info_type {
            "phone" => self.fake_phone(original),
            "id_card" => self.fake_id_card(original),
            "email" => self.fake_email(original),
            "name" => self.fake_name(original),
            _ => self.mask_char.repeat(original.chars().count()),
        }
    }
    
    /// 生成假手机号
    fn fake_phone(&self, original: &str) -> String {
        // 安全边界检查
        if original.len() < 3 {
            return "13800000000".to_string();
        }
        // 保持前3位，后面随机生成
        let prefix = &original[..3];
        let suffix: String = (0..8)
            .map(|_| rand::random::<u8>().to_string())
            .collect::<String>()
            .chars()
            .take(8)
            .collect();
        format!("{}{}", prefix, suffix)
    }
    
    /// 生成假身份证号
    fn fake_id_card(&self, original: &str) -> String {
        // 安全边界检查
        let area = if original.len() >= 6 { &original[..6] } else { "110101" };
        let check = original.chars().last().unwrap_or('0');
        let middle: String = (0..11)
            .map(|_| rand::random::<u8>().to_string())
            .collect::<String>()
            .chars()
            .take(11)
            .collect();
        format!("{}{}{}", area, middle, check)
    }
    
    /// 生成假邮箱
    fn fake_email(&self, original: &str) -> String {
        let domain = original.split('@').last().unwrap_or("example.com");
        let user: String = (0..8)
            .map(|_| {
                let chars = "abcdefghijklmnopqrstuvwxyz0123456789";
                chars.chars().nth(rand::random::<usize>() % chars.len()).unwrap()
            })
            .collect();
        format!("{}@{}", user, domain)
    }
    
    /// 生成假姓名
    fn fake_name(&self, _original: &str) -> String {
        let surnames = ["张", "李", "王", "刘", "陈", "杨", "赵", "黄", "周", "吴"];
        let names = ["伟", "芳", "娜", "秀英", "敏", "静", "丽", "强", "磊", "军"];
        
        let surname = surnames[rand::random::<usize>() % surnames.len()];
        let name = names[rand::random::<usize>() % names.len()];
        
        format!("{}{}", surname, name)
    }
}

impl Default for Masker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mask_content() {
        let masker = Masker::new();
        // 使用纯英文内容避免中文字符边界问题
        let content = "phone: 13812345678, id: 110101199001011234";
        let detections = vec![
            Detection {
                info_type: "phone".to_string(),
                original: "13812345678".to_string(),
                masked: "138****5678".to_string(),
                start: 7,
                end: 18,
                line: 1,
                column: 8,
                confidence: 1.0,
            },
        ];
        
        let result = masker.mask_content(content, &detections);
        assert!(result.contains("138****5678"));
    }
    
    #[test]
    fn test_partial_mask_phone() {
        let masker = Masker::new();
        let content = "phone: 13812345678";
        let detections = vec![
            Detection {
                info_type: "phone".to_string(),
                original: "13812345678".to_string(),
                masked: "138****5678".to_string(),
                start: 7,
                end: 18,
                line: 1,
                column: 8,
                confidence: 1.0,
            },
        ];
        
        let result = masker.apply_mask(content, &detections, "partial_mask");
        assert!(result.contains("138****5678"));
        // 验证格式：前3后4，中间4个星号
        assert!(result.contains("138") && result.contains("5678"));
    }
    
    #[test]
    fn test_full_mask() {
        let masker = Masker::new();
        let content = "id: 110101199001011234";
        let detections = vec![
            Detection {
                info_type: "id_card".to_string(),
                original: "110101199001011234".to_string(),
                masked: "******************".to_string(),
                start: 4,
                end: 22,
                line: 1,
                column: 5,
                confidence: 1.0,
            },
        ];
        
        let result = masker.apply_mask(content, &detections, "full_mask");
        // 完全隐藏应该全部替换为星号
        assert!(result.contains("******************"));
        assert!(!result.contains("110101"));
    }
    
    #[test]
    fn test_hash_mask() {
        let masker = Masker::new();
        let content = "email: test@example.com";
        let detections = vec![
            Detection {
                info_type: "email".to_string(),
                original: "test@example.com".to_string(),
                masked: "".to_string(), // 哈希脱敏会生成新值
                start: 7,
                end: 23,
                line: 1,
                column: 8,
                confidence: 1.0,
            },
        ];
        
        let result = masker.apply_mask(content, &detections, "hash");
        // 哈希脱敏应该生成8位十六进制字符串
        // 验证邮箱原文不存在
        assert!(!result.contains("test@example.com"));
    }
    
    #[test]
    fn test_fake_phone_generation() {
        let masker = Masker::new();
        let original = "13812345678";
        let fake = masker.generate_fake_data("phone", original);
        
        // 验证假数据格式
        assert_eq!(fake.len(), 11);
        // 前3位应该保持不变
        assert_eq!(&fake[..3], "138");
        // 后8位应该不同
        assert_ne!(&fake[3..], "12345678");
    }
    
    #[test]
    fn test_fake_email_generation() {
        let masker = Masker::new();
        let original = "user@company.com";
        let fake = masker.generate_fake_data("email", original);
        
        // 验证假邮箱格式
        assert!(fake.contains("@"));
        assert!(fake.ends_with("company.com"));
        // 用户名应该不同
        let fake_user = fake.split('@').next().unwrap();
        assert_ne!(fake_user, "user");
    }
    
    #[test]
    fn test_fake_id_card_generation() {
        let masker = Masker::new();
        let original = "110101199001011234";
        let fake = masker.generate_fake_data("id_card", original);
        
        // 验证假身份证格式
        assert_eq!(fake.len(), 18);
        // 地区码应该保持不变
        assert_eq!(&fake[..6], "110101");
    }
    
    #[test]
    fn test_fake_name_generation() {
        let masker = Masker::new();
        let original = "张三";
        let fake = masker.generate_fake_data("name", original);
        
        // 验证假姓名是中文
        assert!(fake.chars().count() >= 2);
        assert!(fake.chars().all(|c| c > '\u{4E00}' && c < '\u{9FFF}'));
    }
    
    #[test]
    fn test_mask_empty_detections() {
        let masker = Masker::new();
        let content = "没有任何敏感信息";
        let detections: Vec<Detection> = vec![];
        
        let result = masker.mask_content(content, &detections);
        assert_eq!(result, content);
    }
    
    #[test]
    fn test_mask_multiple_detections() {
        let masker = Masker::new();
        let content = "phone 13812345678, email test@example.com";
        let detections = vec![
            Detection {
                info_type: "phone".to_string(),
                original: "13812345678".to_string(),
                masked: "138****5678".to_string(),
                start: 6,
                end: 17,
                line: 1,
                column: 7,
                confidence: 1.0,
            },
            Detection {
                info_type: "email".to_string(),
                original: "test@example.com".to_string(),
                masked: "t***@example.com".to_string(),
                start: 24,
                end: 40,
                line: 1,
                column: 25,
                confidence: 1.0,
            },
        ];
        
        let result = masker.mask_content(content, &detections);
        assert!(result.contains("138****5678"));
        assert!(result.contains("t***@example.com"));
    }
}
