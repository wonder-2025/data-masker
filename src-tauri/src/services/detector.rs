// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 敏感信息检测引擎
//! 
//! 支持正则表达式匹配、Luhn校验、词典匹配等多种检测方式

use regex::Regex;
use std::collections::HashMap;

/// 检测结果
#[derive(Debug, Clone)]
pub struct Detection {
    pub info_type: String,
    pub original: String,
    pub masked: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub confidence: f32,
}

/// 规则定义
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub rule_type: String,
    pub pattern: String,
    pub strategy: String,
    pub strategy_config: StrategyConfig,
    pub need_luhn_check: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub keep_start: usize,
    pub keep_end: usize,
    pub mask_char: String,
}

/// 敏感信息检测器
pub struct Detector {
    rules: Vec<Rule>,
    compiled_patterns: HashMap<String, Regex>,
}

impl Detector {
    /// 创建新的检测器
    pub fn new(rules: Vec<Rule>) -> Self {
        let mut compiled_patterns = HashMap::new();
        
        for rule in &rules {
            if !rule.pattern.is_empty() {
                if let Ok(re) = Regex::new(&rule.pattern) {
                    compiled_patterns.insert(rule.id.clone(), re);
                }
            }
        }
        
        Detector { rules, compiled_patterns }
    }
    
    /// 综合检测所有敏感信息
    pub fn detect_all(&self, content: &str) -> Vec<Detection> {
        let mut all_detections = Vec::new();
        
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            let detections = self.detect_by_rule(content, rule);
            all_detections.extend(detections);
        }
        
        // 按位置排序
        all_detections.sort_by_key(|d| d.start);
        
        // 去重（处理重叠匹配）
        all_detections = self.remove_overlaps(all_detections);
        
        // 计算行号和列号
        for detection in &mut all_detections {
            let (line, column) = self.calculate_position(content, detection.start);
            detection.line = line;
            detection.column = column;
        }
        
        all_detections
    }
    
    /// 根据规则检测
    fn detect_by_rule(&self, content: &str, rule: &Rule) -> Vec<Detection> {
        let mut detections = Vec::new();
        
        if let Some(re) = self.compiled_patterns.get(&rule.id) {
            for cap in re.find_iter(content) {
                let matched_text = cap.as_str();
                
                // Luhn校验（银行卡号）
                if rule.need_luhn_check && !self.luhn_check(matched_text) {
                    continue;
                }
                
                // 身份证校验
                if rule.rule_type == "id_card" && !self.validate_id_card(matched_text) {
                    continue;
                }
                
                // 生成脱敏值
                let masked = self.generate_masked_value(matched_text, rule);
                
                detections.push(Detection {
                    info_type: rule.rule_type.clone(),
                    original: matched_text.to_string(),
                    masked,
                    start: cap.start(),
                    end: cap.end(),
                    line: 0,
                    column: 0,
                    confidence: 1.0,
                });
            }
        }
        
        detections
    }
    
    /// 生成脱敏值
    fn generate_masked_value(&self, text: &str, rule: &Rule) -> String {
        let config = &rule.strategy_config;
        
        match rule.strategy.as_str() {
            "full_mask" => {
                config.mask_char.repeat(text.chars().count())
            }
            "partial_mask" => {
                let chars: Vec<char> = text.chars().collect();
                let len = chars.len();
                let keep_start = config.keep_start.min(len);
                let keep_end = config.keep_end.min(len - keep_start);
                
                let mut result = String::new();
                
                // 保留前几位
                for i in 0..keep_start {
                    result.push(chars[i]);
                }
                
                // 脱敏中间部分
                let mask_len = len - keep_start - keep_end;
                result.push_str(&config.mask_char.repeat(mask_len));
                
                // 保留后几位
                for i in (len - keep_end)..len {
                    result.push(chars[i]);
                }
                
                result
            }
            "hash" => {
                // SHA256 前8位
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(text.as_bytes());
                let result = hasher.finalize();
                hex::encode(&result[..4])
            }
            _ => {
                config.mask_char.repeat(text.chars().count())
            }
        }
    }
    
    /// Luhn算法校验银行卡号
    fn luhn_check(&self, number: &str) -> bool {
        let digits: Vec<u32> = number.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() < 13 || digits.len() > 19 {
            return false;
        }
        
        let mut sum = 0;
        let mut alternate = false;
        
        for &digit in digits.iter().rev() {
            let mut n = digit;
            
            if alternate {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            
            sum += n;
            alternate = !alternate;
        }
        
        sum % 10 == 0
    }
    
    /// 身份证号校验
    fn validate_id_card(&self, id: &str) -> bool {
        let id = id.to_uppercase();
        
        // 15位身份证
        if id.len() == 15 {
            return id.chars().all(|c| c.is_ascii_digit());
        }
        
        // 18位身份证
        if id.len() == 18 {
            if !id[..17].chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            
            let last_char = id.chars().last().unwrap();
            if !last_char.is_ascii_digit() && last_char != 'X' {
                return false;
            }
            
            // 校验码验证
            let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
            let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
            
            let mut sum: u32 = 0;
            for (i, c) in id[..17].chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    sum += d * weights[i];
                }
            }
            
            let check_index = (sum % 11) as usize;
            let expected_check = check_codes[check_index];
            
            return id.chars().last().unwrap() == expected_check;
        }
        
        false
    }
    
    /// 计算行号和列号
    fn calculate_position(&self, text: &str, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        
        for (i, c) in text.char_indices() {
            if i >= offset {
                break;
            }
            
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        (line, column)
    }
    
    /// 移除重叠的检测结果（保留更具体的）
    fn remove_overlaps(&self, detections: Vec<Detection>) -> Vec<Detection> {
        if detections.is_empty() {
            return detections;
        }
        
        let mut result = Vec::new();
        let mut current = detections[0].clone();
        
        for detection in detections.into_iter().skip(1) {
            if detection.start < current.end {
                // 重叠，保留更长的
                if detection.end - detection.start > current.end - current.start {
                    current = detection;
                }
            } else {
                result.push(current);
                current = detection;
            }
        }
        
        result.push(current);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_luhn_check() {
        let detector = Detector::new(vec![]);
        
        // 有效的银行卡号（测试用）
        assert!(detector.luhn_check("4532015112830366"));
        
        // 无效的银行卡号
        assert!(!detector.luhn_check("4532015112830367"));
    }
    
    #[test]
    fn test_id_card_validation() {
        let detector = Detector::new(vec![]);
        
        // 有效的18位身份证号（测试用）
        assert!(detector.validate_id_card("11010519491231002X"));
        
        // 无效的身份证号
        assert!(!detector.validate_id_card("110105194912310021"));
    }
}
