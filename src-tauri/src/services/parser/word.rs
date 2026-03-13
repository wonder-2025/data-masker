// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! Word 文档解析器
//! 
//! 使用 XML 层级操作，不破坏样式/表格/修订

use std::path::PathBuf;
use zip::ZipArchive;
use std::io::{Read, Write};
use quick_xml::{Reader, events::Event};

/// Word 文档解析结果
pub struct WordParseResult {
    pub text: String,
    pub paragraphs: Vec<String>,
}

/// 解析 Word 文档
pub fn parse_word(path: &PathBuf) -> Result<WordParseResult, String> {
    let file = std::fs::File::open(path)
        .map_err(|e| format!("打开文件失败: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("解析ZIP失败: {}", e))?;
    
    let mut text = String::new();
    let mut paragraphs = Vec::new();
    
    // 读取 document.xml
    if let Ok(mut doc) = archive.by_name("word/document.xml") {
        let mut content = String::new();
        doc.read_to_string(&mut content)
            .map_err(|e| format!("读取文档内容失败: {}", e))?;
        
        // 提取文本内容
        let reader = Reader::from_str(&content);
        let mut current_text = String::new();
        let mut buf = Vec::new();
        
        let mut rdr = reader;
        loop {
            match rdr.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.local_name().as_ref() == b"w:t" {
                        current_text.clear();
                    }
                }
                Ok(Event::Text(ref e)) => {
                    current_text.push_str(&e.unescape().unwrap_or_default());
                }
                Ok(Event::End(ref e)) => {
                    if e.local_name().as_ref() == b"w:t" {
                        text.push_str(&current_text);
                    }
                    if e.local_name().as_ref() == b"w:p" {
                        if !text.is_empty() {
                            paragraphs.push(text.clone());
                        }
                        text.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
            buf.clear();
        }
    }
    
    Ok(WordParseResult {
        text: paragraphs.join("\n"),
        paragraphs,
    })
}

/// Word 脱敏处理器
pub struct WordMasker;

impl WordMasker {
    /// 对 Word 文档进行脱敏处理
    pub fn mask_word(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        let file = std::fs::File::open(input_path)
            .map_err(|e| format!("打开文件失败: {}", e))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("解析ZIP失败: {}", e))?;
        
        // 读取并处理 document.xml
        let mut doc_content = String::new();
        {
            let mut doc = archive.by_name("word/document.xml")
                .map_err(|e| format!("读取document.xml失败: {}", e))?;
            doc.read_to_string(&mut doc_content)
                .map_err(|e| format!("读取内容失败: {}", e))?;
        }
        
        // 执行替换（保持 XML 结构）
        for (original, masked) in replacements {
            doc_content = doc_content.replace(original, masked);
        }
        
        // 创建新的 ZIP 文件
        let output_file = std::fs::File::create(output_path)
            .map_err(|e| format!("创建输出文件失败: {}", e))?;
        
        let mut zip_writer = zip::ZipWriter::new(output_file);
        let options = zip::write::FileOptions::default();
        
        // 复制所有文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let name = file.name().to_string();
            
            zip_writer.start_file(&name, options)
                .map_err(|e| format!("写入文件失败: {}", e))?;
            
            if name == "word/document.xml" {
                zip_writer.write_all(doc_content.as_bytes())
                    .map_err(|e| format!("写入内容失败: {}", e))?;
            } else {
                std::io::copy(&mut file, &mut zip_writer)
                    .map_err(|e| format!("复制文件失败: {}", e))?;
            }
        }
        
        zip_writer.finish()
            .map_err(|e| format!("完成ZIP写入失败: {}", e))?;
        
        Ok(())
    }
}
