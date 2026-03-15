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
use tauri::Manager;
use std::collections::HashMap;

/// 导出格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    #[serde(rename = "txt")]
    Txt,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
}

/// 导出单个结果
#[command]
pub async fn export_result(
    result: crate::commands::mask::MaskResult,
    format: ExportFormat,
) -> Result<Vec<u8>, String> {
    match format {
        ExportFormat::Txt => {
            let content = result.masked_content.unwrap_or_default();
            Ok(content.into_bytes())
        }
        ExportFormat::Json => {
            let json = serde_json::to_string_pretty(&result)
                .map_err(|e| format!("JSON序列化失败: {}", e))?;
            Ok(json.into_bytes())
        }
        ExportFormat::Csv => {
            let mut csv = String::from("类型,原始值,脱敏后值,位置\n");
            for info in result.sensitive_info {
                csv.push_str(&format!("{},{},{},行{}列{}\n",
                    info.info_type,
                    info.original,
                    info.masked,
                    info.line,
                    info.column
                ));
            }
            Ok(csv.into_bytes())
        }
    }
}

/// 批量导出处理结果
#[command]
pub async fn export_all_results(
    app: tauri::AppHandle,
    results: Vec<crate::commands::mask::MaskResult>
) -> Result<String, String> {
    let output_dir = app.path()
        .app_data_dir()
        .map(|p| p.join("output"))
        .map_err(|e| format!("无法获取输出目录: {}", e))?;
    
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("创建输出目录失败: {}", e))?;
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let export_dir = output_dir.join(format!("export_{}", timestamp));
    std::fs::create_dir_all(&export_dir)
        .map_err(|e| format!("创建导出目录失败: {}", e))?;
    
    for result in &results {
        if let Some(content) = &result.masked_content {
            let file_name = &result.file_name;
            let output_file = export_dir.join(file_name);
            std::fs::write(&output_file, content)
                .map_err(|e| format!("写入文件失败: {}", e))?;
        }
    }
    
    Ok(export_dir.to_string_lossy().to_string())
}

/// 报告数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    #[serde(default)]
    pub summary: ReportSummary,
    #[serde(default)]
    pub sensitive_stats: HashMap<String, usize>,
    #[serde(default)]
    pub results: Vec<ResultSummary>,
    #[serde(default = "default_generated_at")]
    pub generated_at: String,
}

fn default_generated_at() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

impl Default for ReportData {
    fn default() -> Self {
        ReportData {
            summary: ReportSummary::default(),
            sensitive_stats: HashMap::new(),
            results: Vec::new(),
            generated_at: default_generated_at(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportSummary {
    #[serde(default)]
    pub total_files: usize,
    #[serde(default)]
    pub success_count: usize,
    #[serde(default)]
    pub error_count: usize,
    #[serde(default)]
    pub total_sensitive: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSummary {
    pub file_name: String,
    pub status: String,
    pub sensitive_count: usize,
    pub processing_time: String,
}

impl Default for ResultSummary {
    fn default() -> Self {
        ResultSummary {
            file_name: String::new(),
            status: "pending".to_string(),
            sensitive_count: 0,
            processing_time: "0.00s".to_string(),
        }
    }
}

/// 导出处理报告
#[command]
pub async fn export_report(
    app: tauri::AppHandle,
    report_data: ReportData
) -> Result<String, String> {
    // 确保所有字段都有默认值
    let report_data = ReportData {
        summary: ReportSummary {
            total_files: report_data.summary.total_files,
            success_count: report_data.summary.success_count,
            error_count: report_data.summary.error_count,
            total_sensitive: report_data.summary.total_sensitive,
        },
        sensitive_stats: report_data.sensitive_stats,
        results: report_data.results,
        generated_at: if report_data.generated_at.is_empty() {
            default_generated_at()
        } else {
            report_data.generated_at
        },
    };
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let report_name = format!("report_{}.txt", timestamp);
    
    // 生成报告内容
    let mut report = String::new();
    
    // 标题
    report.push_str(&"=".repeat(60));
    report.push_str("\n");
    report.push_str("              Data Masker 脱敏处理报告\n");
    report.push_str(&"=".repeat(60));
    report.push_str("\n\n");
    
    // 基本信息
    report.push_str(&format!("生成时间: {}\n\n", report_data.generated_at));
    
    // 处理概览
    report.push_str("【处理概览】\n");
    report.push_str(&format!("  总文件数: {}\n", report_data.summary.total_files));
    report.push_str(&format!("  成功处理: {}\n", report_data.summary.success_count));
    report.push_str(&format!("  处理失败: {}\n", report_data.summary.error_count));
    report.push_str(&format!("  敏感信息: {} 处\n\n", report_data.summary.total_sensitive));
    
    // 敏感信息类型分布
    report.push_str("【敏感信息类型分布】\n");
    if report_data.sensitive_stats.is_empty() {
        report.push_str("  无敏感信息\n");
    } else {
        let mut sorted_stats: Vec<_> = report_data.sensitive_stats.iter().collect();
        sorted_stats.sort_by(|a, b| b.1.cmp(a.1));
        for (info_type, count) in sorted_stats {
            report.push_str(&format!("  {}: {} 处\n", info_type, count));
        }
    }
    report.push_str("\n");
    
    // 文件处理详情
    report.push_str("【文件处理详情】\n");
    report.push_str(&"-".repeat(60));
    report.push_str("\n");
    
    if report_data.results.is_empty() {
        report.push_str("  无处理记录\n");
    } else {
        for result in report_data.results {
            report.push_str(&format!("\n文件: {}\n", result.file_name));
            report.push_str(&format!("  状态: {}\n", result.status));
            report.push_str(&format!("  敏感信息: {} 处\n", result.sensitive_count));
            report.push_str(&format!("  耗时: {}\n", result.processing_time));
        }
    }
    
    report.push_str("\n");
    report.push_str(&"=".repeat(60));
    report.push_str("\n");
    report.push_str("报告生成完成\n");
    report.push_str("\n");
    report.push_str("产品设计: wonder-宏\n");
    report.push_str("架构设计/开发实现: JARVIS AI Assistant\n");
    report.push_str(&"=".repeat(60));
    report.push_str("\n");
    
    // 保存报告文件
    let output_dir = app.path()
        .app_data_dir()
        .map(|p| p.join("output"))
        .map_err(|e| format!("无法获取输出目录: {}", e))?;
    
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("创建输出目录失败: {}", e))?;
    
    let report_path = output_dir.join(&report_name);
    std::fs::write(&report_path, &report)
        .map_err(|e| format!("写入报告失败: {}", e))?;
    
    Ok(report_path.to_string_lossy().to_string())
}
