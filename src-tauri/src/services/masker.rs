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
            if detection.start <= result.len() && detection.end <= result.len() {
                result.replace_range(detection.start..detection.end, &detection.masked);
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
        // 保持地区码，随机生成日期和顺序码
        let area = &original[..6];
        let check = original.chars().last().unwrap();
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
        let content = "手机号：13812345678，身份证：110101199001011234";
        let detections = vec![
            Detection {
                info_type: "phone".to_string(),
                original: "13812345678".to_string(),
                masked: "138****5678".to_string(),
                start: 4,
                end: 15,
                line: 1,
                column: 5,
                confidence: 1.0,
            },
        ];
        
        let result = masker.mask_content(content, &detections);
        assert!(result.contains("138****5678"));
    }
}
