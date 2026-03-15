// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 审计日志服务
//! 
//! 记录每次脱敏操作，敏感信息在日志中脱敏显示

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// 审计日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AuditLogEntry {
    pub id: String,
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub action: String,
    pub file_name: String,
    pub rule_name: Option<String>,
    pub details: String,
    pub sensitive_count: usize,
}

/// 审计日志管理器
#[allow(dead_code)]
pub struct AuditLogger {
    log_file: PathBuf,
    entries: Vec<AuditLogEntry>,
}

#[allow(dead_code)]
impl AuditLogger {
    /// 创建新的审计日志管理器
    pub fn new(data_dir: &PathBuf) -> Self {
        let log_file = data_dir.join("audit.log");
        let entries = Self::load_entries(&log_file).unwrap_or_default();
        
        AuditLogger { log_file, entries }
    }
    
    /// 添加日志条目
    pub fn log(&mut self, level: LogLevel, action: &str, file_name: &str, details: &str, sensitive_count: usize) {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Local::now(),
            level,
            action: action.to_string(),
            file_name: file_name.to_string(),
            rule_name: None,
            details: details.to_string(),
            sensitive_count,
        };
        
        self.entries.push(entry);
        self.save_entries();
    }
    
    /// 获取所有日志
    pub fn get_entries(&self) -> &[AuditLogEntry] {
        &self.entries
    }
    
    /// 清空日志
    pub fn clear(&mut self) {
        self.entries.clear();
        self.save_entries();
    }
    
    /// 加载日志条目
    fn load_entries(path: &PathBuf) -> Option<Vec<AuditLogEntry>> {
        if !path.exists() {
            return None;
        }
        
        let content = std::fs::read_to_string(path).ok()?;
        let entries: Vec<AuditLogEntry> = content
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();
        
        Some(entries)
    }
    
    /// 保存日志条目
    fn save_entries(&self) {
        if let Ok(file) = std::fs::File::create(&self.log_file) {
            use std::io::Write;
            let mut writer = std::io::BufWriter::new(file);
            
            for entry in &self.entries {
                if let Ok(json) = serde_json::to_string(entry) {
                    let _ = writeln!(writer, "{}", json);
                }
            }
        }
    }
    
    /// 脱敏日志内容
    pub fn sanitize_for_display(text: &str) -> String {
        use regex::Regex;
        
        let mut result = text.to_string();
        
        // 手机号脱敏
        let phone_re = Regex::new(r"1[3-9]\d{9}").unwrap();
        result = phone_re.replace_all(&result, "138****5678").to_string();
        
        // 身份证脱敏
        let id_re = Regex::new(r"\d{17}[\dXx]").unwrap();
        result = id_re.replace_all(&result, "110101********1234").to_string();
        
        // 邮箱脱敏
        let email_re = Regex::new(r"[\w.-]+@[\w.-]+\.\w+").unwrap();
        result = email_re.replace_all(&result, "u***@example.com").to_string();
        
        result
    }
}

/// 导出审计日志
#[allow(dead_code)]
pub fn export_audit_log(entries: &[AuditLogEntry]) -> String {
    let mut report = String::new();
    
    report.push_str("=" .repeat(60).as_str());
    report.push_str("\n");
    report.push_str("              Data Masker 审计日志\n");
    report.push_str(&("=".repeat(60)));
    report.push_str("\n\n");
    
    for entry in entries {
        report.push_str(&format!("[{}] {}\n", 
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            match entry.level {
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARN",
                LogLevel::Error => "ERROR",
                LogLevel::Success => "SUCCESS",
            }
        ));
        report.push_str(&format!("  操作: {}\n", entry.action));
        report.push_str(&format!("  文件: {}\n", AuditLogger::sanitize_for_display(&entry.file_name)));
        report.push_str(&format!("  详情: {}\n", AuditLogger::sanitize_for_display(&entry.details)));
        if entry.sensitive_count > 0 {
            report.push_str(&format!("  敏感信息: {} 处\n", entry.sensitive_count));
        }
        report.push_str("\n");
    }
    
    report
}
