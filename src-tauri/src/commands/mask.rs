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
use std::path::PathBuf;

/// 敏感信息检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct StrategyConfig {
    pub keep_start: usize,
    pub keep_end: usize,
    pub mask_char: String,
    pub custom_text: Option<String>,
}

/// 规则定义（用于 API 接口）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    // 关键字替换模式字段
    #[serde(default)]
    pub mode: String,           // 'regex' 或 'keyword'
    #[serde(default)]
    pub keyword: String,        // 要查找的关键字
    #[serde(default)]
    pub replacement: String,    // 替换文本
    #[serde(default)]
    pub case_sensitive: bool,   // 大小写敏感
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
            // 关键字替换模式字段
            mode: rule.mode,
            keyword: rule.keyword,
            replacement: rule.replacement,
            case_sensitive: rule.case_sensitive,
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
    app: tauri::AppHandle,
    file_path: String,
    rules: Vec<Rule>,
    output_dir: Option<String>,
) -> Result<MaskResult, String> {
    use std::time::Instant;
    
    let start_time = Instant::now();
    let input_path = PathBuf::from(&file_path);
    
    // DEBUG: 记录传入参数
    tracing::info!("[DEBUG] ========== process_file 开始 ==========");
    tracing::info!("[DEBUG] file_path: {}", file_path);
    tracing::info!("[DEBUG] output_dir: {:?}", output_dir);
    tracing::info!("[DEBUG] rules count: {}", rules.len());
    
    // 检查规则内容
    for (i, rule) in rules.iter().enumerate() {
        tracing::info!("[DEBUG] Rule {}: id={}, enabled={}, pattern={}", 
            i, rule.id, rule.enabled, 
            if rule.pattern.len() > 30 { &rule.pattern[..30] } else { &rule.pattern });
    }
    
    // 检查文件是否存在
    if !input_path.exists() {
        return Err(format!("文件不存在: {}", file_path));
    }
    
    // 获取文件扩展名
    let extension = input_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 检查不支持的格式
    if extension == "doc" {
        return Err("不支持旧版 Word 格式 (.doc)，请将文件转换为 .docx 格式后重试".to_string());
    }
    
    // 读取文件内容
    let content = crate::commands::file::read_file_content(file_path.clone()).await?;
    
    // 安全检查：只使用已启用的规则（双重保险）
    let enabled_rules: Vec<Rule> = rules.into_iter()
        .filter(|r| r.enabled)
        .collect();
    
    tracing::info!("[DEBUG] 过滤后的启用规则数量: {}", enabled_rules.len());
    
    // 检查是否有启用的规则
    if enabled_rules.is_empty() {
        return Err("没有启用的脱敏规则，请先在规则配置中启用至少一条规则".to_string());
    }
    
    // 转换规则
    let detector_rules: Vec<crate::services::detector::Rule> = enabled_rules
        .into_iter()
        .map(|r| r.into())
        .collect();
    
    // DEBUG: 记录内容信息
    tracing::info!("[DEBUG] ========== 文件内容检测 ==========");
    tracing::info!("[DEBUG] Content length: {} bytes", content.content.len());
    tracing::info!("[DEBUG] Content preview: {}", 
        if content.content.len() > 200 { &content.content[..200] } else { &content.content });
    tracing::info!("[DEBUG] Rules count: {}", detector_rules.len());
    
    // 检查规则是否正确转换
    for (i, rule) in detector_rules.iter().enumerate() {
        tracing::info!("[DEBUG] Converted Rule {}: id={}, enabled={}, mode={}", 
            i, rule.id, rule.enabled, rule.mode);
    }
    
    // 检测敏感信息
    let detector = crate::services::detector::Detector::new(detector_rules.clone());
    let detections = detector.detect_all(&content.content);
    
    // DEBUG: 记录检测结果
    tracing::info!("[DEBUG] ========== 检测结果 ==========");
    tracing::info!("[DEBUG] Detections count: {}", detections.len());
    
    if detections.is_empty() {
        tracing::warn!("[DEBUG] ⚠️ 没有检测到敏感信息！");
        tracing::warn!("[DEBUG] 可能原因: 1) 规则未启用 2) 规则格式错误 3) 内容中无敏感信息");
    } else {
        for (i, d) in detections.iter().enumerate() {
            tracing::info!("[DEBUG] Detection {}: type={}, original={}", i, d.info_type, d.original);
        }
    }
    
    // 生成替换列表
    let replacements: Vec<(String, String)> = detections.iter()
        .map(|d| (d.original.clone(), d.masked.clone()))
        .collect();
    
    // 应用脱敏
    let masker = crate::services::masker::Masker::new();
    let masked_content = masker.mask_content(&content.content, &detections);
    
    // 生成输出路径 - 优先使用用户设置的路径
    let output_dir = if let Some(ref user_dir) = output_dir {
        tracing::info!("[DEBUG] user_dir: {}", user_dir);
        if !user_dir.is_empty() {
            tracing::info!("[DEBUG] using user output dir: {}", user_dir);
            let user_path = PathBuf::from(user_dir);
            // 确保用户指定的目录存在
            if !user_path.exists() {
                std::fs::create_dir_all(&user_path)
                    .map_err(|e| format!("创建用户输出目录失败: {}", e))?;
            }
            user_path
        } else {
            tracing::info!("[DEBUG] user_dir is empty, using default");
            // 用户路径为空，使用默认路径
            app.path()
                .app_data_dir()
                .map(|p| p.join("output"))
                .map_err(|e| format!("无法获取输出目录: {}", e))?
        }
    } else {
        tracing::info!("[DEBUG] output_dir is None, using default");
        // 未提供用户路径，使用默认路径
        app.path()
            .app_data_dir()
            .map(|p| p.join("output"))
            .map_err(|e| format!("无法获取输出目录: {}", e))?
    };
    
    tracing::info!("[DEBUG] final output_dir: {:?}", output_dir);
    
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("创建输出目录失败: {}", e))?;
    
    let file_stem = input_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let output_name = format!("{}_masked_{}.{}", file_stem, timestamp, extension);
    let output_path = output_dir.join(&output_name);
    
    // DEBUG: 记录输出路径信息
    tracing::info!("[DEBUG] ========== 输出处理 ==========");
    tracing::info!("[DEBUG] Output directory: {:?}", output_dir);
    tracing::info!("[DEBUG] Output file path: {:?}", output_path);
    tracing::info!("[DEBUG] Extension: {}", extension);
    tracing::info!("[DEBUG] Replacements count: {}", replacements.len());
    
    // 根据文件类型保存
    let save_result = match extension.as_str() {
        "txt" | "md" | "json" | "xml" | "csv" => {
            // 文本文件直接保存
            std::fs::write(&output_path, &masked_content)
                .map_err(|e| format!("保存文件失败: {}", e))
        }
        "pdf" => {
            // PDF 文件处理
            if replacements.is_empty() {
                // 没有需要替换的内容，直接复制
                std::fs::copy(&input_path, &output_path)
                    .map_err(|e| format!("复制文件失败: {}", e))?;
                Ok(())
            } else {
                crate::services::parser::pdf::PdfMasker::mask_pdf(
                    &input_path,
                    &output_path,
                    &replacements
                )
            }
        }
        "docx" => {
            // Word 文件处理
            if replacements.is_empty() {
                std::fs::copy(&input_path, &output_path)
                    .map_err(|e| format!("复制文件失败: {}", e))?;
                Ok(())
            } else {
                crate::services::parser::word::WordMasker::mask_word(
                    &input_path,
                    &output_path,
                    &replacements
                )
            }
        }
        "xlsx" | "xls" => {
            // Excel 文件处理
            if replacements.is_empty() {
                std::fs::copy(&input_path, &output_path)
                    .map_err(|e| format!("复制文件失败: {}", e))?;
                Ok(())
            } else {
                crate::services::parser::excel::ExcelMasker::mask_excel(
                    &input_path,
                    &output_path,
                    &replacements
                )
            }
        }
        "pptx" => {
            // PPT 文件处理 - 目前仅支持复制
            // TODO: 实现 PPT 文件的脱敏处理
            std::fs::copy(&input_path, &output_path)
                .map_err(|e| format!("复制文件失败: {}", e))?;
            Ok(())
        }
        _ => {
            // 其他格式保存为文本
            std::fs::write(&output_path, &masked_content)
                .map_err(|e| format!("保存文件失败: {}", e))
        }
    };
    
    // 检查保存结果
    if let Err(e) = save_result {
        tracing::error!("[DEBUG] ❌ 保存文件失败: {}", e);
        return Err(e);
    }
    
    // 验证输出文件
    if !output_path.exists() {
        tracing::error!("[DEBUG] ❌ 输出文件创建失败: {:?}", output_path);
        return Err(format!("输出文件创建失败: {:?}", output_path));
    }
    
    // 获取输出文件大小
    if let Ok(metadata) = std::fs::metadata(&output_path) {
        tracing::info!("[DEBUG] ✅ 输出文件创建成功: {:?} ({} bytes)", output_path, metadata.len());
    }
    
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
        file_name: input_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        status: "done".to_string(),
        sensitive_info,
        masked_content: Some(masked_content),
        output_path: Some(output_path.to_string_lossy().to_string()),
        sensitive_count,
        processing_time,
    })
}


