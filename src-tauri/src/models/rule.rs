// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 规则模型定义

use serde::{Deserialize, Serialize};

/// 脱敏规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    pub pattern: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub priority: u32,
    pub strategy: MaskStrategy,
}

/// 规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    // 个人信息
    IdCard,
    Passport,
    HkMoPass,       // 港澳通行证
    TwPass,         // 台湾通行证
    Name,
    
    // 联系方式
    Phone,
    Telephone,
    Email,
    
    // 金融信息
    BankCard,
    CreditCode,     // 统一社会信用代码
    
    // 网络信息
    Ipv4,
    Ipv6,
    Mac,
    ApiKey,
    Url,
    
    // 其他
    LicensePlate,   // 车牌号
    Company,
    Address,
    Amount,
    Date,
    
    // 自定义
    Custom,
}

/// 脱敏策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaskStrategy {
    FullMask,
    PartialMask { keep_start: usize, keep_end: usize },
    FakeData,
    Reversible,
    Hash { prefix_len: usize },
    Custom { replacement: String },
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            rule_type: RuleType::Custom,
            pattern: String::new(),
            description: None,
            enabled: true,
            priority: 0,
            strategy: MaskStrategy::FullMask,
        }
    }
}

impl Rule {
    /// 创建新的自定义规则
    pub fn custom(name: &str, pattern: &str) -> Self {
        Rule {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            rule_type: RuleType::Custom,
            pattern: pattern.to_string(),
            description: None,
            enabled: true,
            priority: 0,
            strategy: MaskStrategy::FullMask,
        }
    }
    
    /// 设置部分掩码策略
    pub fn with_partial_mask(mut self, keep_start: usize, keep_end: usize) -> Self {
        self.strategy = MaskStrategy::PartialMask { keep_start, keep_end };
        self
    }
    
    /// 设置描述
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
}
