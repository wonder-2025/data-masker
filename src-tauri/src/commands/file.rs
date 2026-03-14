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

/// 选择文件对话框
#[command]
pub async fn select_files(app: tauri::AppHandle) -> Result<Vec<FileInfo>, String> {
    use tauri_plugin_dialog::DialogExt;
    use tauri_plugin_dialog::FilePath;
    
    let paths = app.dialog()
        .file()
        .add_filter("支持的文件", &[
            "pdf", "docx", "doc", "xlsx", "xls", 
            "txt", "md", "csv", "json", "xml", "pptx"
        ])
        .blocking_pick_files();
    
    match paths {
        Some(files) => {
            let file_infos: Vec<FileInfo> = files.iter()
                .filter_map(|path| {
                    // 正确处理 FilePath 枚举
                    if let FilePath::Path(p) = path {
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
    let file_path = PathBuf::from(&path);
    
    if !file_path.exists() {
        return Err(format!("文件不存在: {}", path));
    }
    
    // 根据文件类型读取
    let extension = file_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let content = match extension.as_str() {
        "txt" | "md" | "json" | "xml" | "csv" => {
            read_text_file(&file_path)?
        }
        "pdf" => {
            read_pdf_preview(&file_path)?
        }
        "docx" | "xlsx" | "pptx" => {
            read_office_preview(&file_path, &extension)?
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
pub async fn save_file(path: String, content: Vec<u8>) -> Result<(), String> {
    let file_path = PathBuf::from(&path);
    
    // 确保父目录存在
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }
    
    std::fs::write(&file_path, content)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    
    Ok(())
}

/// 打开文件所在目录
#[command]
pub async fn open_file_location(path: String) -> Result<(), String> {
    let file_path = PathBuf::from(&path);
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &file_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &file_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Some(parent) = file_path.parent() {
            std::process::Command::new("xdg-open")
                .arg(parent)
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
        Ok(text) => Ok(text),
        Err(e) => Err(format!("解析PDF失败: {}", e)),
    }
}

/// 读取 Office 文件预览
fn read_office_preview(path: &PathBuf, extension: &str) -> Result<String, String> {
    match extension {
        "docx" => read_docx_preview(path),
        "xlsx" => read_xlsx_preview(path),
        "pptx" => read_pptx_preview(path),
        _ => Err(format!("不支持的文件类型: {}", extension)),
    }
}

/// 读取 Word 文档预览
fn read_docx_preview(path: &PathBuf) -> Result<String, String> {
    use std::io::Read;
    use zip::ZipArchive;
    
    let file = std::fs::File::open(path)
        .map_err(|e| format!("打开文件失败: {}", e))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("解析ZIP失败: {}", e))?;
    
    let mut content = String::new();
    
    if let Ok(mut doc) = archive.by_name("word/document.xml") {
        let mut xml_content = String::new();
        doc.read_to_string(&mut xml_content)
            .map_err(|e| format!("读取文档内容失败: {}", e))?;
        
        content = extract_text_from_docx_xml(&xml_content);
    }
    
    Ok(content)
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
    use calamine::Reader;
    
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(path)
        .map_err(|e| format!("打开Excel失败: {}", e))?;
    
    let mut content = String::new();
    
    let sheets = workbook.sheet_names().to_vec();
    for sheet_name in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                let cells: Vec<String> = row.iter()
                    .map(|cell| cell.to_string())
                    .collect();
                content.push_str(&cells.join("\t"));
                content.push('\n');
            }
        }
    }
    
    Ok(content)
}

/// 读取 PowerPoint 文件预览
fn read_pptx_preview(_path: &PathBuf) -> Result<String, String> {
    Ok("[PowerPoint 文件预览暂不支持]".to_string())
}

/// 扫描文件夹中的支持文件
#[command]
pub async fn scan_folder(path: String) -> Result<Vec<FileInfo>, String> {
    let folder_path = PathBuf::from(&path);
    
    if !folder_path.exists() || !folder_path.is_dir() {
        return Err(format!("文件夹不存在: {}", path));
    }
    
    let supported_extensions = [
        "pdf", "docx", "doc", "xlsx", "xls", 
        "txt", "md", "csv", "json", "xml", "pptx"
    ];
    
    let mut files = Vec::new();
    
    fn scan_dir(dir: &PathBuf, files: &mut Vec<FileInfo>, extensions: &[&str]) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    scan_dir(&path, files, extensions);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if extensions.contains(&ext.to_lowercase().as_str()) {
                        if let Some(file_info) = create_file_info(&path) {
                            files.push(file_info);
                        }
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
