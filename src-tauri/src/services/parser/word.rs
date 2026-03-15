// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! Word 文件解析器
//! 
//! 支持解析和脱敏 .docx 格式 Word 文档
//! 注意: 不支持旧版 .doc 格式（OLE Compound Document）

use std::path::PathBuf;
use std::io::{Read, Cursor};
use zip::ZipArchive;
use regex::Regex;

/// Word 解析结果
pub struct WordParseResult {
    pub text: String,
    pub paragraphs: usize,
}

/// 解析 Word 文档 (.docx)
/// 
/// .docx 格式是基于 XML 的 ZIP 压缩包
/// 主要内容在 word/document.xml 中
pub fn parse_word(path: &PathBuf) -> Result<WordParseResult, String> {
    // 检查文件扩展名
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 拒绝处理 .doc 格式
    if extension == "doc" {
        return Err("不支持旧版 Word 格式 (.doc)。请将文件转换为 .docx 格式后重试。\n\n转换方法:\n1. 用 Microsoft Word 打开 .doc 文件\n2. 点击 '文件' -> '另存为'\n3. 选择格式为 '.docx'\n4. 保存后重新选择该文件".to_string());
    }
    
    // 验证文件是否存在
    if !path.exists() {
        return Err(format!("文件不存在: {:?}", path));
    }
    
    // 检查文件大小
    let metadata = std::fs::metadata(path)
        .map_err(|e| format!("无法读取文件信息: {}", e))?;
    
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!("文件过大 ({}MB)，最大支持 100MB", metadata.len() / 1024 / 1024));
    }
    
    // 打开 ZIP 文件
    let file = std::fs::File::open(path)
        .map_err(|e| format!("打开文件失败: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| {
            if e.to_string().contains("Invalid zip header") {
                "文件格式错误: 不是有效的 .docx 文件。\n\n可能的原因:\n1. 文件已损坏\n2. 文件是旧版 .doc 格式（不支持）\n3. 文件扩展名错误\n\n建议: 请用 Word 打开文件确认格式，或转换为 .docx 格式".to_string()
            } else {
                format!("解析 ZIP 失败: {}", e)
            }
        })?;
    
    // 提取文档内容
    let mut content = String::new();
    let mut paragraphs = 0;
    
    // 读取主文档
    if let Ok(mut doc) = archive.by_name("word/document.xml") {
        let mut xml_content = String::new();
        doc.read_to_string(&mut xml_content)
            .map_err(|e| format!("读取文档内容失败: {}", e))?;
        
        // 提取文本内容
        let (text, para_count) = extract_text_from_docx(&xml_content);
        content = text;
        paragraphs = para_count;
    } else {
        return Err("无法找到文档内容: 文件可能已损坏或不是有效的 Word 文档".to_string());
    }
    
    if content.trim().is_empty() {
        return Err("Word 文档内容为空".to_string());
    }
    
    Ok(WordParseResult { text: content, paragraphs })
}

/// 从 Word XML 中提取文本
fn extract_text_from_docx(xml: &str) -> (String, usize) {
    // 匹配 <w:t> 标签中的文本
    let re = Regex::new(r"<w:t[^>]*>([^<]*)</w:t>").unwrap();
    
    let mut text = String::new();
    let mut in_paragraph = false;
    let mut paragraph_count = 0;
    
    // 简单的状态机来识别段落
    let mut last_end = 0;
    
    for cap in re.captures_iter(xml) {
        if let Some(m) = cap.get(0) {
            // 检查是否在新的段落中
            let before = &xml[last_end..m.start()];
            if before.contains("<w:p ") || before.contains("<w:p>") {
                if in_paragraph {
                    text.push('\n');
                }
                paragraph_count += 1;
                in_paragraph = true;
            }
            
            if let Some(text_content) = cap.get(1) {
                text.push_str(text_content.as_str());
            }
            
            last_end = m.end();
        }
    }
    
    (text, paragraph_count)
}

/// Word 脱敏处理器
pub struct WordMasker;

impl WordMasker {
    /// 对 Word 文档进行脱敏处理
    pub fn mask_word(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 验证输入文件
        if !input_path.exists() {
            return Err(format!("输入文件不存在: {:?}", input_path));
        }
        
        // 检查是否是 .doc 格式
        let extension = input_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if extension == "doc" {
            return Err("不支持旧版 Word 格式 (.doc)，请将文件转换为 .docx 格式后重试".to_string());
        }
        
        // 检查是否有替换内容
        if replacements.is_empty() {
            // 没有需要替换的内容，直接复制文件
            std::fs::copy(input_path, output_path)
                .map_err(|e| format!("复制文件失败: {:?}", e))?;
            return Ok(());
        }
        
        // 读取输入文件
        let input_bytes = std::fs::read(input_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;
        
        let reader = Cursor::new(input_bytes);
        let mut archive = ZipArchive::new(reader)
            .map_err(|e| format!("解析 ZIP 失败: {}", e))?;
        
        // 处理 document.xml
        let mut document_xml = String::new();
        {
            let mut doc_file = archive.by_name("word/document.xml")
                .map_err(|e| format!("无法找到文档内容: {}", e))?;
            doc_file.read_to_string(&mut document_xml)
                .map_err(|e| format!("读取文档内容失败: {}", e))?;
        }
        
        // 执行替换
        for (original, masked) in replacements {
            document_xml = document_xml.replace(original, masked);
        }
        
        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建输出目录失败: {:?}", e))?;
        }
        
        // 创建输出文件
        let output_file = std::fs::File::create(output_path)
            .map_err(|e| format!("创建输出文件失败: {:?}", e))?;
        
        let mut zip_writer = zip::ZipWriter::new(output_file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        
        // 复制所有文件，替换 document.xml
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| format!("读取 ZIP 条目失败: {}", e))?;
            let file_name = file.name().to_string();
            
            if file_name == "word/document.xml" {
                // 写入修改后的内容
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入 ZIP 条目失败: {:?}", e))?;
                zip_writer.write_all(document_xml.as_bytes())
                    .map_err(|e| format!("写入内容失败: {:?}", e))?;
            } else {
                // 直接复制其他文件
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入 ZIP 条目失败: {:?}", e))?;
                
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| format!("读取 ZIP 条目失败: {:?}", e))?;
                zip_writer.write_all(&buffer)
                    .map_err(|e| format!("写入内容失败: {:?}", e))?;
            }
        }
        
        zip_writer.finish()
            .map_err(|e| format!("完成 ZIP 写入失败: {:?}", e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_text() {
        let xml = r#"<w:p><w:r><w:t>Hello</w:t></w:r></w:p>"#;
        let (text, _) = extract_text_from_docx(xml);
        assert!(text.contains("Hello"));
    }
}
