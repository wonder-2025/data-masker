// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 处理结果模型定义

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

/// 脱敏处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskResult {
    pub id: String,
    pub file_name: String,
    pub file_path: String,
    pub output_path: Option<String>,
    pub status: ProcessStatus,
    pub sensitive_info: Vec<SensitiveInfo>,
    pub processing_time_ms: u64,
    pub created_at: DateTime<Local>,
}

/// 敏感信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveInfo {
    pub rule_id: String,
    pub rule_name: String,
    pub info_type: String,
    pub original: String,
    pub masked: String,
    pub position: TextPosition,
    pub confidence: f32,
}

/// 文本位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPosition {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// 处理状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessStatus {
    Pending,
    Processing,
    Success,
    PartialSuccess,
    Failed,
}

/// 处理统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStats {
    pub total_files: usize,
    pub success_count: usize,
    pub failed_count: usize,
    pub total_sensitive: usize,
    pub by_type: std::collections::HashMap<String, usize>,
}

impl Default for MaskResult {
    fn default() -> Self {
        MaskResult {
            id: uuid::Uuid::new_v4().to_string(),
            file_name: String::new(),
            file_path: String::new(),
            output_path: None,
            status: ProcessStatus::Pending,
            sensitive_info: Vec::new(),
            processing_time_ms: 0,
            created_at: Local::now(),
        }
    }
}

impl ProcessStats {
    pub fn new() -> Self {
        ProcessStats {
            total_files: 0,
            success_count: 0,
            failed_count: 0,
            total_sensitive: 0,
            by_type: std::collections::HashMap::new(),
        }
    }
    
    pub fn add_result(&mut self, result: &MaskResult) {
        self.total_files += 1;
        
        match result.status {
            ProcessStatus::Success | ProcessStatus::PartialSuccess => {
                self.success_count += 1;
            }
            ProcessStatus::Failed => {
                self.failed_count += 1;
            }
            _ => {}
        }
        
        self.total_sensitive += result.sensitive_info.len();
        
        for info in &result.sensitive_info {
            *self.by_type.entry(info.info_type.clone()).or_insert(0) += 1;
        }
    }
}
