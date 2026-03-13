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
use std::collections::HashMap;

/// 预览结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewResult {
    pub original: String,
    pub masked: String,
    pub sensitive_info: Vec<SensitiveInfoPreview>,
    pub stats: PreviewStats,
}

/// 敏感信息预览项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveInfoPreview {
    #[serde(rename = "type")]
    pub info_type: String,
    pub original: String,
    pub masked: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// 预览统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewStats {
    pub total_sensitive: usize,
    pub by_type: HashMap<String, usize>,
}

/// 生成预览
#[command]
pub async fn generate_preview(
    file_path: String,
    rules: Vec<crate::commands::mask::Rule>,
) -> Result<PreviewResult, String> {
    use crate::services::detector::Detector;
    use crate::services::masker::Masker;
    
    // 读取文件内容
    let content = crate::commands::file::read_file_content(file_path).await?;
    
    // 转换规则
    let detector_rules: Vec<crate::services::detector::Rule> = rules
        .into_iter()
        .map(|r| r.into())
        .collect();
    
    // 检测敏感信息
    let detector = Detector::new(detector_rules);
    let detections = detector.detect_all(&content.content);
    
    // 应用脱敏
    let masker = Masker::new();
    let masked_content = masker.mask_content(&content.content, &detections);
    
    // 计算位置信息
    let sensitive_info: Vec<SensitiveInfoPreview> = detections.iter()
        .map(|d| {
            SensitiveInfoPreview {
                info_type: d.info_type.clone(),
                original: d.original.clone(),
                masked: d.masked.clone(),
                start: d.start,
                end: d.end,
                line: d.line,
                column: d.column,
            }
        })
        .collect();
    
    // 统计信息 - 先计算长度，再移动
    let total_count = sensitive_info.len();
    let mut by_type: HashMap<String, usize> = HashMap::new();
    for info in &sensitive_info {
        *by_type.entry(info.info_type.clone()).or_insert(0) += 1;
    }
    
    Ok(PreviewResult {
        original: content.content,
        masked: masked_content,
        sensitive_info,
        stats: PreviewStats {
            total_sensitive: total_count,
            by_type,
        },
    })
}
