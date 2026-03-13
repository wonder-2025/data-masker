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

/// 敏感信息检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveInfo {
    pub id: usize,
    #[serde(rename = "type")]
    pub info_type: String,
    pub original: String,
    pub masked: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// 脱敏处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskResult {
    pub file_id: String,
    pub file_name: String,
    pub status: String,
    pub sensitive_info: Vec<SensitiveInfo>,
    pub masked_content: Option<String>,
    pub output_path: Option<String>,
    pub sensitive_count: usize,
    pub processing_time: String,
}

/// 脱敏策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub keep_start: usize,
    pub keep_end: usize,
    pub mask_char: String,
    pub custom_text: Option<String>,
}

/// 规则定义（用于 API 接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub pattern: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub strategy: String,
    pub strategy_config: Option<StrategyConfig>,
    #[serde(default)]
    pub need_luhn_check: bool,
}

/// 将 API Rule 转换为 detector Rule
impl From<Rule> for crate::services::detector::Rule {
    fn from(rule: Rule) -> Self {
        let config = rule.strategy_config.unwrap_or(StrategyConfig {
            keep_start: 0,
            keep_end: 0,
            mask_char: "*".to_string(),
            custom_text: None,
        });
        
        crate::services::detector::Rule {
            id: rule.id,
            name: rule.name,
            rule_type: rule.rule_type,
            pattern: rule.pattern,
            strategy: rule.strategy,
            strategy_config: crate::services::detector::StrategyConfig {
                keep_start: config.keep_start,
                keep_end: config.keep_end,
                mask_char: config.mask_char,
            },
            need_luhn_check: rule.need_luhn_check,
            enabled: rule.enabled,
        }
    }
}

/// 将 Detection 转换为 SensitiveInfo
impl From<crate::services::detector::Detection> for SensitiveInfo {
    fn from(d: crate::services::detector::Detection) -> Self {
        SensitiveInfo {
            id: 0,
            info_type: d.info_type,
            original: d.original,
            masked: d.masked,
            start: d.start,
            end: d.end,
            line: d.line,
            column: d.column,
        }
    }
}

/// 检测敏感信息
#[command]
pub async fn detect_sensitive(
    content: String,
    rules: Vec<Rule>,
) -> Result<Vec<SensitiveInfo>, String> {
    let detector_rules: Vec<crate::services::detector::Rule> = rules
        .into_iter()
        .map(|r| r.into())
        .collect();
    
    let detector = crate::services::detector::Detector::new(detector_rules);
    let detections = detector.detect_all(&content);
    
    Ok(detections.into_iter()
        .enumerate()
        .map(|(i, d)| {
            let mut info: SensitiveInfo = d.into();
            info.id = i;
            info
        })
        .collect())
}

/// 应用脱敏
#[command]
pub async fn apply_mask(
    content: String,
    detections: Vec<SensitiveInfo>,
    strategy: String,
) -> Result<String, String> {
    let masker = crate::services::masker::Masker::new();
    
    // 转换 detections
    let detector_detections: Vec<crate::services::detector::Detection> = detections
        .into_iter()
        .map(|d| crate::services::detector::Detection {
            info_type: d.info_type,
            original: d.original,
            masked: d.masked,
            start: d.start,
            end: d.end,
            line: d.line,
            column: d.column,
            confidence: 1.0,
        })
        .collect();
    
    let masked = masker.apply_mask(&content, &detector_detections, &strategy);
    Ok(masked)
}

/// 处理单个文件
#[command]
pub async fn process_file(
    file_path: String,
    rules: Vec<Rule>,
) -> Result<MaskResult, String> {
    use std::time::Instant;
    
    let start_time = Instant::now();
    
    // 读取文件内容
    let content = crate::commands::file::read_file_content(file_path.clone()).await?;
    
    // 转换规则
    let detector_rules: Vec<crate::services::detector::Rule> = rules
        .into_iter()
        .map(|r| r.into())
        .collect();
    
    // 检测敏感信息
    let detector = crate::services::detector::Detector::new(detector_rules);
    let detections = detector.detect_all(&content.content);
    
    // 应用脱敏
    let masker = crate::services::masker::Masker::new();
    let masked_content = masker.mask_content(&content.content, &detections);
    
    // 生成输出路径
    let output_path = generate_output_path(&file_path);
    
    // 计算处理时间
    let elapsed = start_time.elapsed();
    let processing_time = format!("{:.2}s", elapsed.as_secs_f64());
    
    // 转换检测结果
    let sensitive_info: Vec<SensitiveInfo> = detections.into_iter()
        .enumerate()
        .map(|(i, d)| {
            let mut info: SensitiveInfo = d.into();
            info.id = i;
            info
        })
        .collect();
    
    let sensitive_count = sensitive_info.len();
    
    Ok(MaskResult {
        file_id: uuid::Uuid::new_v4().to_string(),
        file_name: content.path.split('/').last().unwrap_or("unknown").to_string(),
        status: "done".to_string(),
        sensitive_info,
        masked_content: Some(masked_content),
        output_path: Some(output_path),
        sensitive_count,
        processing_time,
    })
}

/// 生成输出文件路径
fn generate_output_path(input_path: &str) -> String {
    let path = std::path::PathBuf::from(input_path);
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("txt");
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let output_name = format!("{}_masked_{}.{}", file_stem, timestamp, extension);
    
    // 返回相对于用户文档目录的路径
    format!("output/{}", output_name)
}
