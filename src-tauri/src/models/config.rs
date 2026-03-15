// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 配置模型定义

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub masking: MaskingConfig,
    pub security: SecurityConfig,
    pub advanced: AdvancedConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GeneralConfig {
    pub language: String,
    pub theme: String,
    pub output_dir: PathBuf,
    pub auto_open_output: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MaskingConfig {
    pub default_strategy: String,
    pub keep_start_digits: usize,
    pub keep_end_digits: usize,
    pub mask_char: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SecurityConfig {
    pub password_protect: bool,
    pub auto_clean_temp: bool,
    pub clean_after_minutes: u32,
    pub encrypt_mapping: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AdvancedConfig {
    pub log_level: String,
    pub max_file_size_mb: u32,
    pub concurrent_files: u32,
    pub enable_ocr: bool,
    pub enable_ner: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            general: GeneralConfig {
                language: "zh-CN".to_string(),
                theme: "light".to_string(),
                output_dir: PathBuf::from("output"),
                auto_open_output: true,
            },
            masking: MaskingConfig {
                default_strategy: "partial_mask".to_string(),
                keep_start_digits: 3,
                keep_end_digits: 4,
                mask_char: "*".to_string(),
            },
            security: SecurityConfig {
                password_protect: false,
                auto_clean_temp: true,
                clean_after_minutes: 30,
                encrypt_mapping: true,
            },
            advanced: AdvancedConfig {
                log_level: "info".to_string(),
                max_file_size_mb: 100,
                concurrent_files: 3,
                enable_ocr: false,
                enable_ner: false,
            },
        }
    }
}

#[allow(dead_code)]
impl AppConfig {
    /// 从文件加载配置
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))
    }
    
    /// 保存配置到文件
    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建配置目录失败: {}", e))?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))
    }
}
