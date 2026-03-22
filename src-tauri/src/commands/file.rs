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

/// 文件信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub file_type: String,
    pub status: String,
    pub added_at: String,
}

/// 文件内容结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub encoding: String,
    pub line_count: usize,
}

/// 安全验证错误
#[derive(Debug)]
#[allow(dead_code)]
pub enum SecurityError {
    PathTraversal,
    InvalidPath,
    ForbiddenExtension,
    FileTooLarge,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::PathTraversal => write!(f, "路径遍历攻击检测"),
            SecurityError::InvalidPath => write!(f, "无效的文件路径"),
            SecurityError::ForbiddenExtension => write!(f, "禁止访问的文件类型"),
            SecurityError::FileTooLarge => write!(f, "文件大小超过限制"),
        }
    }
}

/// 验证路径安全性
/// 防止路径遍历攻击
pub fn validate_path(path: &PathBuf, app_handle: &tauri::AppHandle) -> Result<PathBuf, SecurityError> {
    // 1. 获取规范路径（解析符号链接和 ..）
    let canonical_path = path.canonicalize()
        .map_err(|_| SecurityError::InvalidPath)?;
    
    // 2. 获取允许的基目录
    let app_data_dir = app_handle.path()
        .app_data_dir()
        .map_err(|_| SecurityError::InvalidPath)?;
    
    let output_dir = app_data_dir.join("output");
    let temp_dir = app_data_dir.join("temp");
    
    // 3. 检查路径是否在允许的目录内
    let is_allowed = canonical_path.starts_with(&output_dir) 
        || canonical_path.starts_with(&temp_dir)
        || canonical_path.starts_with(std::env::current_dir().unwrap_or_default());
    
    if !is_allowed {
        // 记录安全事件
        tracing::warn!(
            "路径遍历尝试被阻止: {:?} (规范路径: {:?})",
            path, canonical_path
        );
        return Err(SecurityError::PathTraversal);
    }
    
    // 4. 检查文件扩展名
    if let Some(ext) = canonical_path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        // 黑名单扩展名（可执行文件等）
        let forbidden_extensions = ["exe", "bat", "cmd", "sh", "ps1", "vbs", "js", "jar"];
        if forbidden_extensions.contains(&ext_lower.as_str()) {
            return Err(SecurityError::ForbiddenExtension);
        }
    }
    
    Ok(canonical_path)
}

/// 验证用户输入的路径（用于文件读取）
pub fn validate_user_path(path: &str) -> Result<PathBuf, String> {
    let file_path = PathBuf::from(path);
    
    // 基本检查
    if !file_path.exists() {
        return Err("文件不存在".to_string());
    }
    
    // 检查路径遍历模式
    let path_str = path.to_lowercase();
    let dangerous_patterns = ["../", "..\\", "~", "$(", "${", "`", "|", ";", "&", "<", ">"];
    
    for pattern in dangerous_patterns {
        if path_str.contains(pattern) {
            tracing::warn!("检测到危险的路径模式: {} in {}", pattern, path);
            return Err("路径包含不安全的字符".to_string());
        }
    }
    
    // 检查扩展名
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        let forbidden_extensions = ["exe", "bat", "cmd", "sh", "ps1", "vbs", "js", "jar"];
        if forbidden_extensions.contains(&ext_lower.as_str()) {
            return Err("不支持该文件类型".to_string());
        }
    }
    
    Ok(file_path)
}

/// 转义路径用于命令行
pub fn escape_path_for_command(path: &str) -> String {
    // 移除或转义危险字符
    let mut escaped = String::new();
    for c in path.chars() {
        match c {
            // 危险字符：移除或替换
            '|' | '&' | ';' | '<' | '>' | '`' | '$' | '(' | ')' => {
                // 跳过这些字符
            }
            // 引号和空格：转义
            '"' | ' ' => {
                escaped.push('\\');
                escaped.push(c);
            }
            // 其他字符：保留
            _ => escaped.push(c),
        }
    }
    escaped
}

/// 选择文件对话框
#[command]
pub async fn select_files(app: tauri::AppHandle) -> Result<Vec<FileInfo>, String> {
    use tauri_plugin_dialog::DialogExt;
    use tauri_plugin_dialog::FilePath;
    
    let paths = app.dialog()
        .file()
        .add_filter("支持的文件", &[
            "pdf", "docx", "xlsx", "xls", 
            "txt", "md", "csv", "json", "xml", "pptx"
        ])
        .add_filter("所有文件", &["*"])
        .blocking_pick_files();
    
    match paths {
        Some(files) => {
            let file_infos: Vec<FileInfo> = files.iter()
                .filter_map(|path| {
                    // 正确处理 FilePath 枚举
                    if let FilePath::Path(p) = path {
                        // 验证路径安全性
                        if let Err(e) = validate_user_path(&p.to_string_lossy()) {
                            tracing::warn!("文件路径验证失败: {}", e);
                            return None;
                        }
                        
                        // 检查文件格式是否支持
                        let ext = p.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();
                        
                        // 不支持的格式给出提示
                        if ext == "doc" {
                            tracing::warn!("不支持旧版 Word 格式 (.doc)，请转换为 .docx 格式: {:?}", p);
                            return None;
                        }
                        
                        create_file_info(p)
                    } else {
                        None // 忽略 URL 类型
                    }
                })
                .collect();
            Ok(file_infos)
        }
        None => Ok(vec![]),
    }
}

/// 选择目录对话框
#[command]
pub async fn select_directory(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    use tauri_plugin_dialog::FilePath;
    
    let folder = app.dialog()
        .file()
        .blocking_pick_folder();
    
    match folder {
        Some(path) => {
            // 正确处理 FilePath 枚举
            match path {
                FilePath::Path(p) => Ok(p.to_string_lossy().to_string()),
                FilePath::Url(u) => Ok(u.to_string()),
            }
        }
        None => Ok(String::new()),
    }
}

/// 读取文件内容（用于预览）
#[command]
pub async fn read_file_content(path: String) -> Result<FileContent, String> {
    // 验证路径安全性
    let file_path = validate_user_path(&path)?;
    
    // 根据文件类型读取
    let extension = file_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 检查不支持的格式
    if extension == "doc" {
        return Err("不支持旧版 Word 格式 (.doc)，请将文件转换为 .docx 格式后重试".to_string());
    }
    
    let content = match extension.as_str() {
        "txt" | "md" | "json" | "xml" | "csv" => {
            read_text_file(&file_path)?
        }
        "pdf" => {
            read_pdf_preview(&file_path)?
        }
        "docx" => {
            read_docx_preview(&file_path)?
        }
        "xlsx" | "xls" => {
            read_xlsx_preview(&file_path)?
        }
        "pptx" => {
            read_pptx_preview(&file_path)?
        }
        _ => {
            return Err(format!("不支持的文件类型: {}", extension));
        }
    };
    
    let line_count = content.lines().count();
    
    Ok(FileContent {
        path: path.clone(),
        content,
        encoding: "utf-8".to_string(),
        line_count,
    })
}

/// 读取文件预览（限制内容长度）
#[command]
pub async fn read_file_preview(path: String) -> Result<String, String> {
    let file_content = read_file_content(path).await?;
    
    // 限制预览长度为 50000 字符
    let preview = if file_content.content.len() > 50000 {
        file_content.content[..50000].to_string() + "\n\n... (内容过长，已截断)"
    } else {
        file_content.content
    };
    
    Ok(preview)
}

/// 保存文件
#[command]
pub async fn save_file(path: String, content: Vec<u8>, app: tauri::AppHandle) -> Result<(), String> {
    let file_path = PathBuf::from(&path);
    
    // 验证路径安全性
    let canonical_path = validate_path(&file_path, &app)
        .map_err(|e| format!("路径验证失败: {}", e))?;
    
    // 确保父目录存在
    if let Some(parent) = canonical_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }
    
    std::fs::write(&canonical_path, content)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(())
}

/// 打开文件所在目录
#[command]
pub async fn open_file_location(path: String) -> Result<(), String> {
    // 验证路径安全性
    let file_path = validate_user_path(&path)?;
    
    // 获取规范路径，防止路径遍历攻击
    let canonical_path = file_path.canonicalize()
        .map_err(|e| format!("路径解析失败: {}", e))?;
    
    // 转换为字符串并转义危险字符
    let path_str = escape_path_for_command(&canonical_path.to_string_lossy());
    
    // 验证转义后的路径不为空
    if path_str.is_empty() {
        return Err("无效的文件路径".to_string());
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path_str])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path_str])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Some(parent) = canonical_path.parent() {
            let parent_str = escape_path_for_command(&parent.to_string_lossy());
            std::process::Command::new("xdg-open")
                .arg(&parent_str)
                .spawn()
                .map_err(|e| format!("打开目录失败: {}", e))?;
        }
    }
    
    Ok(())
}

/// 打开输出目录
#[command]
pub async fn open_output_directory(app: tauri::AppHandle) -> Result<(), String> {
    let output_dir = app.path()
        .app_data_dir()
        .map(|p| p.join("output"))
        .map_err(|e| format!("无法获取输出目录: {}", e))?;
    
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&output_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&output_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&output_dir)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    Ok(())
}

/// 清除临时文件
#[command]
pub async fn clear_temp_files(app: tauri::AppHandle) -> Result<(), String> {
    let temp_dir = app.path()
        .app_data_dir()
        .map(|p| p.join("temp"))
        .map_err(|e| format!("无法获取临时目录: {}", e))?;
    
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)
            .map_err(|e| format!("清除临时文件失败: {}", e))?;
        
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("创建临时目录失败: {}", e))?;
    }
    
    Ok(())
}

// ============== 辅助函数 ==============

/// 创建文件信息
fn create_file_info(path: &std::path::Path) -> Option<FileInfo> {
    let metadata = std::fs::metadata(path).ok()?;
    let name = path.file_name()?.to_string_lossy().to_string();
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 文件大小限制：100MB（与解析器保持一致）
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
    if metadata.len() > MAX_FILE_SIZE {
        tracing::warn!("文件过大，跳过: {} ({} 字节)", name, metadata.len());
        return None;
    }
    
    Some(FileInfo {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        file_type: extension,
        status: "pending".to_string(),
        added_at: chrono::Local::now().to_rfc3339(),
    })
}

/// 读取文本文件
fn read_text_file(path: &PathBuf) -> Result<String, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 尝试 UTF-8 解码
    match String::from_utf8(bytes.clone()) {
        Ok(content) => Ok(content),
        Err(_) => {
            // 尝试 GBK 解码（常见于中文 Windows）
            let (content, _encoding, _had_errors) = encoding_rs::GBK.decode(&bytes);
            Ok(content.to_string())
        }
    }
}

/// 读取 PDF 预览
fn read_pdf_preview(path: &PathBuf) -> Result<String, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("读取PDF失败: {}", e))?;
    
    match pdf_extract::extract_text_from_mem(&bytes) {
        Ok(text) if !text.is_empty() => Ok(text),
        Ok(_) => {
            // 如果提取的文本为空，尝试使用 lopdf
            match lopdf::Document::load_mem(&bytes) {
                Ok(doc) => {
                    let mut text = String::new();
                    let pages: Vec<(u32, lopdf::ObjectId)> = doc.get_pages().into_iter().collect();
                    
                    for (_page_num, page_id) in pages {
                        if let Ok(page) = doc.get_object(page_id) {
                            if let lopdf::Object::Dictionary(dict) = page {
                                if let Ok(contents) = dict.get(b"Contents") {
                                    match contents {
                                        lopdf::Object::Reference(stream_id) => {
                                            if let Ok(stream_obj) = doc.get_object(*stream_id) {
                                                if let lopdf::Object::Stream(s) = stream_obj {
                                                    if let Ok(content) = s.decompressed_content() {
                                                        text.push_str(&String::from_utf8_lossy(&content));
                                                        text.push('\n');
                                                    }
                                                }
                                            }
                                        }
                                        lopdf::Object::Array(arr) => {
                                            for obj in arr {
                                                if let lopdf::Object::Reference(stream_id) = obj {
                                                    if let Ok(stream_obj) = doc.get_object(*stream_id) {
                                                        if let lopdf::Object::Stream(s) = stream_obj {
                                                            if let Ok(content) = s.decompressed_content() {
                                                                text.push_str(&String::from_utf8_lossy(&content));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    
                    if text.is_empty() {
                        Err("PDF 文件可能包含扫描图像或加密内容，无法提取文本".to_string())
                    } else {
                        Ok(text)
                    }
                }
                Err(e) => Err(format!("解析PDF失败: {}", e)),
            }
        }
        Err(e) => Err(format!("解析PDF失败: {}", e)),
    }
}

/// 读取 Word 文档预览
fn read_docx_preview(path: &PathBuf) -> Result<String, String> {
    use std::io::Read;
    use zip::ZipArchive;
    
    let file = std::fs::File::open(path)
        .map_err(|e| format!("打开文件失败: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("解析ZIP失败: {}，请确认文件是有效的 .docx 格式", e))?;
    
    let mut content = String::new();
    
    if let Ok(mut doc) = archive.by_name("word/document.xml") {
        let mut xml_content = String::new();
        doc.read_to_string(&mut xml_content)
            .map_err(|e| format!("读取文档内容失败: {}", e))?;
        
        content = extract_text_from_docx_xml(&xml_content);
    }
    
    if content.is_empty() {
        Err("Word 文档内容为空或无法解析".to_string())
    } else {
        Ok(content)
    }
}

/// 从 Word XML 中提取文本
fn extract_text_from_docx_xml(xml: &str) -> String {
    use regex::Regex;
    
    let re = Regex::new(r"<w:t[^>]*>([^<]+)</w:t>").unwrap();
    let texts: Vec<&str> = re.captures_iter(xml)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str())
        .collect();
    
    texts.join("")
}

/// 读取 Excel 文件预览
fn read_xlsx_preview(path: &PathBuf) -> Result<String, String> {
    
    
    // 检查文件是否存在
    if !path.exists() {
        return Err(format!("文件不存在: {:?}", path));
    }
    
    let mut workbook: calamine::Xlsx<_> = match calamine::open_workbook(path) {
        Ok(wb) => wb,
        Err(e) => {
            // 尝试使用 xls 格式
            if path.extension().map(|e| e == "xls").unwrap_or(false) {
                let mut xls_workbook: calamine::Xls<_> = calamine::open_workbook(path)
                    .map_err(|e| format!("打开Excel失败: {}", e))?;
                return read_excel_workbook(&mut xls_workbook);
            }
            return Err(format!("打开Excel失败: {}", e));
        }
    };
    
    read_excel_workbook(&mut workbook)
}

/// 读取 Excel 工作簿内容
fn read_excel_workbook<R: std::io::Read + std::io::Seek>(workbook: &mut impl calamine::Reader<R>) -> Result<String, String> {
    let sheets = workbook.sheet_names().to_vec();
    let mut content = String::new();
    
    for sheet_name in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                let cells: Vec<String> = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
                if !cells.iter().all(|c| c.is_empty()) {
                    content.push_str(&cells.join("\t"));
                    content.push('\n');
                }
            }
        }
    }
    
    if content.is_empty() {
        Ok("[Excel 文件为空或无法读取内容]".to_string())
    } else {
        Ok(content)
    }
}

/// 读取 PowerPoint 文件预览
fn read_pptx_preview(path: &PathBuf) -> Result<String, String> {
    use std::io::Read;
    use zip::ZipArchive;
    use std::time::{Duration, Instant};
    
    tracing::info!("[PPT] 开始读取PPT文件: {:?}", path);
    
    // 设置超时保护：30秒
    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    
    let file = std::fs::File::open(path)
        .map_err(|e| {
            tracing::error!("[PPT] 打开文件失败: {}", e);
            format!("打开文件失败: {}", e)
        })?;
    
    // 限制文件大小
    let metadata = file.metadata().map_err(|e| format!("获取文件信息失败: {}", e))?;
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!("文件过大 ({}MB)，最大支持 100MB", metadata.len() / 1024 / 1024));
    }
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| {
            tracing::error!("[PPT] 解析ZIP失败: {}", e);
            format!("解析ZIP失败: {}", e)
        })?;
    
    let mut content = String::new();
    let mut slide_count = 0;
    let max_slides = 30; // 降低限制，最多处理30页幻灯片
    
    // 遍历所有幻灯片
    let total_files = archive.len();
    for i in 0..total_files {
        // 检查超时
        if start_time.elapsed() > timeout {
            tracing::warn!("[PPT] 读取超时，已处理 {} 页幻灯片", slide_count);
            content.push_str(&format!("\n... (读取超时，仅显示前{}页)\n", slide_count));
            break;
        }
        
        // 限制处理数量，防止内存溢出
        if slide_count >= max_slides {
            content.push_str(&format!("\n... (共超过{}页，仅显示前{}页)\n", max_slides, max_slides));
            break;
        }
        
        // 使用 try-catch 风格的错误处理
        let file_result = archive.by_index(i);
        let Ok(mut file) = file_result else {
            tracing::warn!("[PPT] 无法读取第 {} 个文件，跳过", i);
            continue;
        };
        
        let name = file.name().to_string();
        if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
            // 安全的文本读取
            let read_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content).map(|_| xml_content)
            }));
            
            let Ok(Ok(xml_content)) = read_result else {
                tracing::warn!("[PPT] 读取幻灯片 {} 失败，跳过", i + 1);
                continue;
            };
            
            // 提取文本（带错误处理）
            let slide_text = extract_text_from_pptx_xml(&xml_content);
            if !slide_text.is_empty() {
                content.push_str(&format!("=== 幻灯片 {} ===\n", i + 1));
                content.push_str(&slide_text);
                content.push_str("\n\n");
                slide_count += 1;
            }
        }
    }
    
    tracing::info!("[PPT] 读取完成，共处理{}页幻灯片", slide_count);
    
    if content.is_empty() {
        Ok("[PowerPoint 文件内容为空或无法解析]".to_string())
    } else {
        Ok(content)
    }
}

/// 从 PowerPoint XML 中提取文本 - 使用安全的字符串操作
fn extract_text_from_pptx_xml(xml: &str) -> String {
    // 安全检查：限制输入大小
    const MAX_XML_SIZE: usize = 5 * 1024 * 1024; // 5MB
    if xml.len() > MAX_XML_SIZE {
        tracing::warn!("[PPT] XML 内容过大: {} bytes", xml.len());
        return "[内容过大]".to_string();
    }
    
    // 使用安全的字符串查找，避免 regex 库
    let mut results = Vec::new();
    let mut search_pos = 0;
    const MAX_MATCHES: usize = 5000;
    const MAX_TEXT_LEN: usize = 500;
    
    while results.len() < MAX_MATCHES {
        // 查找 <a:t> 标签开始
        match xml[search_pos..].find("<a:t>") {
            Some(start_idx) => {
                let absolute_start = search_pos + start_idx + 5; // 跳过 <a:t>
                // 查找 </a:t> 结束标签
                match xml[absolute_start..].find("</a:t>") {
                    Some(end_idx) => {
                        let text = &xml[absolute_start..absolute_start + end_idx];
                        // 限制单个文本长度
                        let truncated = if text.len() > MAX_TEXT_LEN {
                            &text[..MAX_TEXT_LEN]
                        } else {
                            text
                        };
                        results.push(truncated.to_string());
                        search_pos = absolute_start + end_idx;
                    }
                    None => break,
                }
            }
            None => break,
        }
    }
    
    if results.is_empty() {
        // 尝试备选方案：简单的文本提取
        "[无法提取文本]".to_string()
    } else {
        results.join(" ")
    }
}

/// 扫描文件夹中的支持文件
#[command]
pub async fn scan_folder(path: String) -> Result<Vec<FileInfo>, String> {
    // 验证路径安全性
    let folder_path = validate_user_path(&path)?;
    
    if !folder_path.is_dir() {
        return Err("指定路径不是文件夹".to_string());
    }
    
    // 支持所有格式
    let supported_extensions = [
        "pdf", "docx", "xlsx", "xls", 
        "txt", "md", "csv", "json", "xml", "pptx"
    ];
    
    let mut files = Vec::new();
    
    fn scan_dir(dir: &PathBuf, files: &mut Vec<FileInfo>, extensions: &[&str]) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // 递归扫描子目录
                    scan_dir(&path, files, extensions);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if extensions.contains(&ext_lower.as_str()) {
                        if let Some(file_info) = create_file_info(&path) {
                            files.push(file_info);
                        }
                    }
                    // 对于 .doc 文件给出警告
                    if ext_lower == "doc" {
                        tracing::warn!("跳过不支持的 .doc 文件: {:?}", path);
                    }
                }
            }
        }
    }
    
    scan_dir(&folder_path, &mut files, &supported_extensions);
    
    // 按文件名排序
    files.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(files)
}

/// 读取文件并返回Base64编码（用于下载）
#[command]
pub async fn read_file_base64(path: String) -> Result<String, String> {
    let file_path = validate_user_path(&path)?;
    
    // 读取文件
    let bytes = std::fs::read(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 转换为Base64
    use base64::{Engine as _, engine::general_purpose};
    let base64_data = general_purpose::STANDARD.encode(&bytes);
    
    // 获取文件名
    let file_name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("download")
        .to_string();
    
    // 返回包含文件名和Base64数据的JSON
    let result = serde_json::json!({
        "fileName": file_name,
        "data": base64_data,
        "mimeType": get_mime_type(&file_name)
    });
    
    Ok(result.to_string())
}

/// 获取日志文件路径
#[command]
pub fn get_log_path() -> Result<String, String> {
    let log_dir = dirs::data_local_dir()
        .map(|p| p.join("DataMasker").join("logs"))
        .unwrap_or_else(|| std::path::PathBuf::from("./logs"));
    
    let log_path = log_dir.join(format!("data-masker-{}.log", 
        chrono::Local::now().format("%Y-%m-%d")));
    
    Ok(log_path.to_string_lossy().to_string())
}

/// 根据文件扩展名获取MIME类型
fn get_mime_type(file_name: &str) -> String {
    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xls" => "application/vnd.ms-excel",
        "txt" => "text/plain",
        "md" => "text/markdown",
        "csv" => "text/csv",
        "json" => "application/json",
        "xml" => "application/xml",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        _ => "application/octet-stream",
    }.to_string()
}
