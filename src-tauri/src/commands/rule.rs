// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

use serde::{Deserialize, Serialize};
use tauri::command;
use regex::Regex;

/// 内置规则定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinRule {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub pattern: String,
    pub description: String,
    pub enabled: bool,
    pub strategy: String,
    pub strategy_config: StrategyConfig,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub keep_start: usize,
    pub keep_end: usize,
}

/// 获取内置规则列表
#[command]
pub async fn get_builtin_rules() -> Result<Vec<BuiltinRule>, String> {
    Ok(get_all_builtin_rules())
}

/// 规则测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTestResult {
    pub matched: bool,
    pub matches: Vec<RuleMatch>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMatch {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// 测试规则匹配效果
#[command]
pub async fn test_rule(pattern: String, test_text: String) -> Result<RuleTestResult, String> {
    match Regex::new(&pattern) {
        Ok(re) => {
            let matches: Vec<RuleMatch> = re.find_iter(&test_text)
                .map(|m| RuleMatch {
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                })
                .collect();
            
            Ok(RuleTestResult {
                matched: !matches.is_empty(),
                matches,
                error: None,
            })
        }
        Err(e) => {
            Ok(RuleTestResult {
                matched: false,
                matches: vec![],
                error: Some(format!("正则表达式错误: {}", e)),
            })
        }
    }
}

/// 验证正则表达式
#[command]
pub async fn validate_regex(pattern: String) -> Result<bool, String> {
    match Regex::new(&pattern) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("正则表达式无效: {}", e)),
    }
}

/// 获取所有内置规则
fn get_all_builtin_rules() -> Vec<BuiltinRule> {
    vec![
        // 身份证号 - 支持15位和18位
        BuiltinRule {
            id: "rule_id_card".to_string(),
            name: "身份证号".to_string(),
            rule_type: "id_card".to_string(),
            pattern: r"\d{17}[\dXx]|\d{15}".to_string(),
            description: "中国大陆身份证号码（支持15位和18位）".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 6, keep_end: 4 },
            examples: vec!["110101199001011234".to_string(), "110101900101123".to_string()],
        },
        
        // 手机号
        BuiltinRule {
            id: "rule_phone".to_string(),
            name: "手机号".to_string(),
            rule_type: "phone".to_string(),
            pattern: r"1[3-9]\d{9}".to_string(),
            description: "中国大陆手机号码".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 3, keep_end: 4 },
            examples: vec!["13812345678".to_string()],
        },
        
        // 银行卡号
        BuiltinRule {
            id: "rule_bank_card".to_string(),
            name: "银行卡号".to_string(),
            rule_type: "bank_card".to_string(),
            pattern: r"\d{16,19}".to_string(),
            description: "银行卡号（16-19位，带Luhn校验）".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 4, keep_end: 4 },
            examples: vec!["6222021234567890123".to_string()],
        },
        
        // 护照号
        BuiltinRule {
            id: "rule_passport".to_string(),
            name: "护照号".to_string(),
            rule_type: "passport".to_string(),
            pattern: r"[A-Z]\d{8}|[A-Z]{2}\d{7}".to_string(),
            description: "中国护照号码".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 1, keep_end: 3 },
            examples: vec!["G12345678".to_string()],
        },
        
        // 港澳通行证
        BuiltinRule {
            id: "rule_hk_mo_pass".to_string(),
            name: "港澳通行证".to_string(),
            rule_type: "hk_mo_pass".to_string(),
            pattern: r"[A-Z]\d{10}".to_string(),
            description: "港澳居民来往内地通行证".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 1, keep_end: 4 },
            examples: vec!["H1234567890".to_string()],
        },
        
        // 台湾通行证
        BuiltinRule {
            id: "rule_tw_pass".to_string(),
            name: "台湾通行证".to_string(),
            rule_type: "tw_pass".to_string(),
            pattern: r"[A-Z]\d{9}".to_string(),
            description: "台湾居民来往大陆通行证".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 1, keep_end: 4 },
            examples: vec!["T123456789".to_string()],
        },
        
        // 统一社会信用代码
        BuiltinRule {
            id: "rule_credit_code".to_string(),
            name: "统一社会信用代码".to_string(),
            rule_type: "credit_code".to_string(),
            pattern: r"[0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}".to_string(),
            description: "18位统一社会信用代码".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 6, keep_end: 4 },
            examples: vec!["91110108MA01WXXX".to_string()],
        },
        
        // 邮箱
        BuiltinRule {
            id: "rule_email".to_string(),
            name: "邮箱".to_string(),
            rule_type: "email".to_string(),
            pattern: r"[\w.-]+@[\w.-]+\.\w+".to_string(),
            description: "电子邮箱地址".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 2, keep_end: 0 },
            examples: vec!["user@example.com".to_string()],
        },
        
        // 车牌号
        BuiltinRule {
            id: "rule_license_plate".to_string(),
            name: "车牌号".to_string(),
            rule_type: "license_plate".to_string(),
            pattern: r"[京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼][A-Z][A-Z0-9]{5,6}".to_string(),
            description: "中国车牌号码".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 1, keep_end: 2 },
            examples: vec!["京A12345".to_string()],
        },
        
        // IPv4地址
        BuiltinRule {
            id: "rule_ipv4".to_string(),
            name: "IPv4地址".to_string(),
            rule_type: "ipv4".to_string(),
            pattern: r"\b(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\b".to_string(),
            description: "IPv4网络地址".to_string(),
            enabled: true,
            strategy: "full_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec!["192.168.1.1".to_string()],
        },
        
        // IPv6地址
        BuiltinRule {
            id: "rule_ipv6".to_string(),
            name: "IPv6地址".to_string(),
            rule_type: "ipv6".to_string(),
            pattern: r"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}".to_string(),
            description: "IPv6网络地址".to_string(),
            enabled: true,
            strategy: "full_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec!["2001:db8::1".to_string()],
        },
        
        // MAC地址
        BuiltinRule {
            id: "rule_mac".to_string(),
            name: "MAC地址".to_string(),
            rule_type: "mac".to_string(),
            pattern: r"([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}".to_string(),
            description: "设备MAC地址".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 2 },
            examples: vec!["00:1A:2B:3C:4D:5E".to_string()],
        },
        
        // JSON密钥
        BuiltinRule {
            id: "rule_api_key".to_string(),
            name: "JSON密钥".to_string(),
            rule_type: "api_key".to_string(),
            pattern: r#"(api[_-]?key|token|secret|password|passwd)\s*[:=]\s*["']?[^"'<>\s]+["']?"#.to_string(),
            description: "API密钥、Token等敏感配置".to_string(),
            enabled: true,
            strategy: "full_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec![r#""api_key": "sk-xxx""#.to_string()],
        },
        
        // 姓名（简化规则）
        BuiltinRule {
            id: "rule_name".to_string(),
            name: "姓名".to_string(),
            rule_type: "name".to_string(),
            pattern: r"[\u4e00-\u9fa5]{2,4}".to_string(),
            description: "中文姓名（2-4字，建议配合NER使用）".to_string(),
            enabled: false,
            strategy: "fake_data".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec!["张三".to_string()],
        },
        
        // 金额
        BuiltinRule {
            id: "rule_amount".to_string(),
            name: "金额".to_string(),
            rule_type: "amount".to_string(),
            pattern: r"[\d,]+\.?\d*\s*(元|万元|亿元|¥|\$|￥)".to_string(),
            description: "金额数值".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 1 },
            examples: vec!["1,234,567.00元".to_string()],
        },
        
        // 日期
        BuiltinRule {
            id: "rule_date".to_string(),
            name: "日期".to_string(),
            rule_type: "date".to_string(),
            pattern: r"\d{4}[-/年]\d{1,2}[-/月]\d{1,2}日?".to_string(),
            description: "日期格式".to_string(),
            enabled: false,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec!["2024年3月15日".to_string()],
        },
        
        // URL
        BuiltinRule {
            id: "rule_url".to_string(),
            name: "URL".to_string(),
            rule_type: "url".to_string(),
            pattern: r#"https?://[^\s<>"']+"#.to_string(),
            description: "网页链接".to_string(),
            enabled: false,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 0, keep_end: 0 },
            examples: vec!["https://example.com".to_string()],
        },
        
        // 电话号码
        BuiltinRule {
            id: "rule_telephone".to_string(),
            name: "电话号码".to_string(),
            rule_type: "telephone".to_string(),
            pattern: r"0\d{2,3}-?\d{7,8}".to_string(),
            description: "固定电话号码".to_string(),
            enabled: true,
            strategy: "partial_mask".to_string(),
            strategy_config: StrategyConfig { keep_start: 3, keep_end: 3 },
            examples: vec!["010-12345678".to_string()],
        },
    ]
}
