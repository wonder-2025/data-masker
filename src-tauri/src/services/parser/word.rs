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
use std::io::{Read, Cursor, Write};
use zip::ZipArchive;
use regex::Regex;

/// Word 解析结果
#[allow(dead_code)]
pub struct WordParseResult {
    pub text: String,
    pub paragraphs: usize,
}

/// 解析 Word 文档 (.docx)
/// 
/// .docx 格式是基于 XML 的 ZIP 压缩包
/// 主要内容在 word/document.xml 中
#[allow(dead_code)]
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
    
    // 1. 读取主文档内容 (word/document.xml)
    {
        let mut doc = archive.by_name("word/document.xml")
            .map_err(|_| "无法找到文档内容: 文件可能已损坏或不是有效的 Word 文档".to_string())?;
        
        let mut xml_content = String::new();
        doc.read_to_string(&mut xml_content)
            .map_err(|e| format!("读取文档内容失败: {}", e))?;
        
        // 提取主文档文本和文本框内容
        let (doc_text, doc_paras) = extract_text_from_docx(&xml_content);
        content.push_str(&doc_text);
        paragraphs += doc_paras;
    }
    
    // 2. 读取页眉 (word/header*.xml)
    let header_patterns = ["word/header1.xml", "word/header2.xml", "word/header3.xml"];
    for pattern in &header_patterns {
        if let Ok(mut header_file) = archive.by_name(pattern) {
            let mut xml_content = String::new();
            if header_file.read_to_string(&mut xml_content).is_ok() {
                let (header_text, _) = extract_text_from_docx(&xml_content);
                if !header_text.trim().is_empty() {
                    content.push_str("\n\n[页眉]\n");
                    content.push_str(&header_text);
                    tracing::debug!("[Word] 读取页眉: {}", pattern);
                }
            }
        }
    }
    
    // 3. 读取页脚 (word/footer*.xml)
    let footer_patterns = ["word/footer1.xml", "word/footer2.xml", "word/footer3.xml"];
    for pattern in &footer_patterns {
        if let Ok(mut footer_file) = archive.by_name(pattern) {
            let mut xml_content = String::new();
            if footer_file.read_to_string(&mut xml_content).is_ok() {
                let (footer_text, _) = extract_text_from_docx(&xml_content);
                if !footer_text.trim().is_empty() {
                    content.push_str("\n\n[页脚]\n");
                    content.push_str(&footer_text);
                    tracing::debug!("[Word] 读取页脚: {}", pattern);
                }
            }
        }
    }
    
    // 4. 读取脚注 (word/footnotes.xml)
    if let Ok(mut footnotes_file) = archive.by_name("word/footnotes.xml") {
        let mut xml_content = String::new();
        if footnotes_file.read_to_string(&mut xml_content).is_ok() {
            let (footnotes_text, _) = extract_text_from_docx(&xml_content);
            if !footnotes_text.trim().is_empty() {
                content.push_str("\n\n[脚注]\n");
                content.push_str(&footnotes_text);
                tracing::debug!("[Word] 读取脚注");
            }
        }
    }
    
    // 5. 读取尾注 (word/endnotes.xml)
    if let Ok(mut endnotes_file) = archive.by_name("word/endnotes.xml") {
        let mut xml_content = String::new();
        if endnotes_file.read_to_string(&mut xml_content).is_ok() {
            let (endnotes_text, _) = extract_text_from_docx(&xml_content);
            if !endnotes_text.trim().is_empty() {
                content.push_str("\n\n[尾注]\n");
                content.push_str(&endnotes_text);
                tracing::debug!("[Word] 读取尾注");
            }
        }
    }
    
    // 重新计算段落数
    paragraphs = content.lines().filter(|line| !line.trim().is_empty()).count();
    
    if content.trim().is_empty() {
        return Err("Word 文档内容为空".to_string());
    }
    
    Ok(WordParseResult { text: content, paragraphs })
}

/// 从 Word XML 中提取文本（包括主内容和文本框）
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
    
    // 检查文本框内容 (w:txbxContent)
    // 文本框内容通常在 <w:txbxContent> 标签内
    if let Ok(txb盒_re) = Regex::new(r"<w:txbxContent[^>]*>([\s\S]*?)</w:txbxContent>") {
        let mut has_textbox = false;
        let mut textbox_text = String::new();
        
        for cap in txb盒_re.captures_iter(xml) {
            if let Some(content) = cap.get(1) {
                // 在文本框内容中提取 <w:t> 标签
                let txb_re = Regex::new(r"<w:t[^>]*>([^<]*)</w:t>").unwrap();
                for text_cap in txb_re.captures_iter(content.as_str()) {
                    if let Some(t) = text_cap.get(1) {
                        if !has_textbox {
                            has_textbox = true;
                            text.push_str("\n\n[文本框]\n");
                        }
                        textbox_text.push_str(t.as_str());
                    }
                }
            }
        }
        
        if has_textbox {
            text.push_str(&textbox_text);
            paragraph_count += 1;
        }
    }
    
    (text, paragraph_count)
}

/// Word 脱敏处理器
pub struct WordMasker;

impl WordMasker {
    /// 检查内容是否包含潜在的 XML 注入攻击
    fn contains_xml_injection(content: &str) -> bool {
        // 检查常见的 XML 注入模式
        let dangerous_patterns = [
            "<!DOCTYPE",
            "<!ENTITY",
            "<![CDATA[",
            "<?xml",
            "javascript:",
            "vbscript:",
            "onload=",
            "onerror=",
            "onclick=",
        ];
        
        let lower = content.to_lowercase();
        dangerous_patterns.iter().any(|pattern| lower.contains(&pattern.to_lowercase()))
    }
    
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
        
        // 收集需要处理的 XML 文件
        let mut xml_files_to_process: Vec<(String, String)> = Vec::new();
        
        // 处理 document.xml
        let mut document_xml = String::new();
        {
            let mut doc_file = archive.by_name("word/document.xml")
                .map_err(|e| format!("无法找到文档内容: {}", e))?;
            doc_file.read_to_string(&mut document_xml)
                .map_err(|e| format!("读取文档内容失败: {}", e))?;
            xml_files_to_process.push(("word/document.xml".to_string(), document_xml.clone()));
        }
        
        // 处理页眉 (word/header*.xml)
        let file_names: Vec<String> = archive.file_names().map(|n| n.to_string()).collect();
        for name in &file_names {
            if name.starts_with("word/header") && name.ends_with(".xml") {
                if let Ok(mut file) = archive.by_name(name) {
                    let mut xml_content = String::new();
                    if file.read_to_string(&mut xml_content).is_ok() {
                        tracing::debug!("[Word Mask] 处理页眉: {}", name);
                        xml_files_to_process.push((name.clone(), xml_content));
                    }
                }
            }
        }
        
        // 处理页脚 (word/footer*.xml)
        for name in &file_names {
            if name.starts_with("word/footer") && name.ends_with(".xml") {
                if let Ok(mut file) = archive.by_name(name) {
                    let mut xml_content = String::new();
                    if file.read_to_string(&mut xml_content).is_ok() {
                        tracing::debug!("[Word Mask] 处理页脚: {}", name);
                        xml_files_to_process.push((name.clone(), xml_content));
                    }
                }
            }
        }
        
        // 处理脚注 (word/footnotes.xml)
        if let Ok(mut file) = archive.by_name("word/footnotes.xml") {
            let mut xml_content = String::new();
            if file.read_to_string(&mut xml_content).is_ok() {
                tracing::debug!("[Word Mask] 处理脚注");
                xml_files_to_process.push(("word/footnotes.xml".to_string(), xml_content));
            }
        }
        
        // 处理尾注 (word/endnotes.xml)
        if let Ok(mut file) = archive.by_name("word/endnotes.xml") {
            let mut xml_content = String::new();
            if file.read_to_string(&mut xml_content).is_ok() {
                tracing::debug!("[Word Mask] 处理尾注");
                xml_files_to_process.push(("word/endnotes.xml".to_string(), xml_content));
            }
        }
        
        // 执行替换
        for (original, masked) in replacements {
            // 安全检查：验证内容不包含 XML 注入攻击
            if Self::contains_xml_injection(original) || Self::contains_xml_injection(masked) {
                tracing::warn!("检测到潜在的 XML 注入内容，跳过该替换");
                continue;
            }
            
            // 对所有 XML 文件执行替换
            for (_, xml_content) in xml_files_to_process.iter_mut() {
                *xml_content = xml_content.replace(original, masked);
            }
        }
        
        // 构建替换后的 XML 文件映射
        let xml_replacements: std::collections::HashMap<String, String> = 
            xml_files_to_process.into_iter().collect();
        
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
        
        // 复制所有文件，替换相关的 XML 文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| format!("读取 ZIP 条目失败: {}", e))?;
            let file_name = file.name().to_string();
            
            if let Some(modified_content) = xml_replacements.get(&file_name) {
                // 写入修改后的内容
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入 ZIP 条目失败: {:?}", e))?;
                zip_writer.write_all(modified_content.as_bytes())
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
