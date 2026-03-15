// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! PDF 文件解析器
//! 
//! 支持解析和脱敏 PDF 文档

use std::path::PathBuf;
use std::io::Cursor;

/// PDF 解析结果
pub struct PdfParseResult {
    pub text: String,
    pub page_count: usize,
}

/// 解析 PDF 文档
pub fn parse_pdf(path: &PathBuf) -> Result<PdfParseResult, String> {
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
    
    // 读取文件
    let bytes = std::fs::read(path)
        .map_err(|e| format!("读取PDF失败: {}", e))?;
    
    // 尝试提取文本
    let text = extract_text_from_pdf(&bytes)?;
    
    // 计算页数
    let page_count = count_pdf_pages(&bytes);
    
    if text.trim().is_empty() {
        return Err("PDF 文件可能包含扫描图像或加密内容，无法提取文本。\n\n建议:\n1. 如果是扫描件，请先进行 OCR 处理\n2. 如果是加密文件，请先解密\n3. 尝试将 PDF 转换为 Word 格式后处理".to_string());
    }
    
    Ok(PdfParseResult { text, page_count })
}

/// 从 PDF 中提取文本
fn extract_text_from_pdf(bytes: &[u8]) -> Result<String, String> {
    // 尝试使用 pdf_extract
    match pdf_extract::extract_text_from_mem(bytes) {
        Ok(text) if !text.trim().is_empty() => {
            return Ok(text);
        }
        Ok(_) => {
            // 提取的文本为空，尝试其他方法
        }
        Err(e) => {
            tracing::debug!("pdf_extract 失败: {}", e);
        }
    }
    
    // 备用方案：使用 lopdf 直接解析
    match lopdf::Document::load_mem(bytes) {
        Ok(doc) => {
            let mut text = String::new();
            let pages: std::collections::BTreeMap<u32, lopdf::ObjectId> = doc.get_pages();
            
            for (_page_num, page_id) in pages {
                if let Ok(page_obj) = doc.get_object(page_id) {
                    if let lopdf::Object::Dictionary(dict) = page_obj {
                        // 尝试获取 Contents
                        if let Ok(contents) = dict.get(b"Contents") {
                            match contents {
                                lopdf::Object::Reference(stream_id) => {
                                    if let Ok(stream_obj) = doc.get_object(*stream_id) {
                                        if let lopdf::Object::Stream(stream) = stream_obj {
                                            if let Ok(content) = stream.decompressed_content() {
                                                // 解析 PDF 内容流
                                                let page_text = parse_pdf_content_stream(&content);
                                                if !page_text.is_empty() {
                                                    text.push_str(&page_text);
                                                    text.push('\n');
                                                }
                                            }
                                        }
                                    }
                                }
                                lopdf::Object::Array(arr) => {
                                    for obj in arr {
                                        if let lopdf::Object::Reference(stream_id) = obj {
                                            if let Ok(stream_obj) = doc.get_object(*stream_id) {
                                                if let lopdf::Object::Stream(stream) = stream_obj {
                                                    if let Ok(content) = stream.decompressed_content() {
                                                        let page_text = parse_pdf_content_stream(&content);
                                                        text.push_str(&page_text);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    text.push('\n');
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            
            if text.trim().is_empty() {
                Err("无法从 PDF 中提取文本内容。文件可能是扫描件或加密文件。".to_string())
            } else {
                Ok(text)
            }
        }
        Err(e) => Err(format!("解析 PDF 失败: {}。文件可能已损坏或使用了不支持的格式。", e)),
    }
}

/// 解析 PDF 内容流，提取文本
fn parse_pdf_content_stream(data: &[u8]) -> String {
    let mut text = String::new();
    let content = String::from_utf8_lossy(data);
    
    // 简单的 Tj 和 TJ 操作符解析
    // Tj: 显示字符串
    // TJ: 显示字符串数组
    
    // 匹配 (...) Tj 格式
    let re_tj = regex::Regex::new(r"\(([^)]*)\)\s*Tj").unwrap();
    for cap in re_tj.captures_iter(&content) {
        if let Some(m) = cap.get(1) {
            text.push_str(m.as_str());
        }
    }
    
    // 匹配 <...> Tj 格式（十六进制字符串）
    let re_hex = regex::Regex::new(r"<([0-9a-fA-F]*)>\s*Tj").unwrap();
    for cap in re_hex.captures_iter(&content) {
        if let Some(m) = cap.get(1) {
            // 简单的十六进制解码
            let hex = m.as_str();
            if hex.len() % 2 == 0 {
                for i in (0..hex.len()).step_by(2) {
                    if let Ok(byte) = u8::from_str_radix(&hex[i..i+2], 16) {
                        if byte >= 32 && byte < 127 {
                            text.push(byte as char);
                        } else if byte >= 0xE4 && byte <= 0xE9 {
                            // 可能是中文字符的开头，简单处理
                            text.push('?');
                        }
                    }
                }
            }
        }
    }
    
    text
}

/// 计算 PDF 页数
fn count_pdf_pages(bytes: &[u8]) -> usize {
    match lopdf::Document::load_mem(bytes) {
        Ok(doc) => doc.get_pages().len(),
        Err(_) => {
            // 备用方案：查找 /Pages 和 /Count
            let content = String::from_utf8_lossy(bytes);
            let re = regex::Regex::new(r"/Count\s+(\d+)").unwrap();
            if let Some(cap) = re.captures(&content) {
                if let Some(m) = cap.get(1) {
                    return m.as_str().parse().unwrap_or(1);
                }
            }
            1
        }
    }
}

/// PDF 脱敏处理器
pub struct PdfMasker;

impl PdfMasker {
    /// 对 PDF 文档进行脱敏处理
    /// 
    /// 注意：PDF 的结构复杂，直接修改可能破坏文件
    /// 当前实现仅支持简单的文本替换
    pub fn mask_pdf(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 验证输入文件
        if !input_path.exists() {
            return Err(format!("输入文件不存在: {:?}", input_path));
        }
        
        // 检查是否有替换内容
        if replacements.is_empty() {
            // 没有需要替换的内容，直接复制文件
            std::fs::copy(input_path, output_path)
                .map_err(|e| format!("复制文件失败: {:?}", e))?;
            return Ok(());
        }
        
        // 读取 PDF 文件
        let bytes = std::fs::read(input_path)
            .map_err(|e| format!("读取PDF失败: {}", e))?;
        
        // 尝试使用 lopdf 进行修改
        match lopdf::Document::load_mem(&bytes) {
            Ok(mut doc) => {
                // 获取所有页面
                let pages: std::collections::BTreeMap<u32, lopdf::ObjectId> = doc.get_pages().clone();
                
                for (_page_num, page_id) in pages {
                    if let Ok(page_obj) = doc.get_object_mut(page_id) {
                        if let lopdf::Object::Dictionary(dict) = page_obj {
                            // 获取 Contents
                            if let Ok(contents) = dict.get(b"Contents") {
                                match contents.clone() {
                                    lopdf::Object::Reference(stream_id) => {
                                        if let Ok(stream_obj) = doc.get_object_mut(stream_id) {
                                            if let lopdf::Object::Stream(stream) = stream_obj {
                                                if let Ok(content) = stream.decompressed_content() {
                                                    let mut content_str = String::from_utf8_lossy(&content).to_string();
                                                    
                                                    // 执行替换
                                                    for (original, masked) in replacements {
                                                        content_str = content_str.replace(original, masked);
                                                    }
                                                    
                                                    // 更新流内容
                                                    stream.set_content(content_str.into_bytes());
                                                }
                                            }
                                        }
                                    }
                                    lopdf::Object::Array(arr) => {
                                        for obj in arr.clone() {
                                            if let lopdf::Object::Reference(stream_id) = obj {
                                                if let Ok(stream_obj) = doc.get_object_mut(stream_id) {
                                                    if let lopdf::Object::Stream(stream) = stream_obj {
                                                        if let Ok(content) = stream.decompressed_content() {
                                                            let mut content_str = String::from_utf8_lossy(&content).to_string();
                                                            
                                                            for (original, masked) in replacements {
                                                                content_str = content_str.replace(original, masked);
                                                            }
                                                            
                                                            stream.set_content(content_str.into_bytes());
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
                
                // 确保输出目录存在
                if let Some(parent) = output_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("创建输出目录失败: {:?}", e))?;
                }
                
                // 保存修改后的 PDF
                doc.save(output_path)
                    .map_err(|e| format!("保存PDF失败: {:?}", e))?;
                
                Ok(())
            }
            Err(e) => {
                // 如果无法解析 PDF，直接复制文件并给出警告
                tracing::warn!("无法解析PDF进行修改，直接复制文件: {}", e);
                std::fs::copy(input_path, output_path)
                    .map_err(|e| format!("复制文件失败: {:?}", e))?;
                
                Err(format!("无法修改 PDF 文件内容（可能包含加密或特殊格式），已复制原文件。\n错误: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_pdf_content() {
        let data = b"(Hello World) Tj";
        let text = parse_pdf_content_stream(data);
        assert!(text.contains("Hello World"));
    }
}
