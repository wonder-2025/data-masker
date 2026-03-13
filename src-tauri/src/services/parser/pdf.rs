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
//! 支持文本层提取和定位替换，保持格式无损

use std::path::PathBuf;

/// PDF 解析结果
pub struct PdfParseResult {
    pub text: String,
    pub pages: Vec<PdfPage>,
}

/// PDF 页面信息
pub struct PdfPage {
    pub page_num: usize,
    pub text: String,
    pub width: f32,
    pub height: f32,
}

/// 解析 PDF 文件
pub fn parse_pdf(path: &PathBuf) -> Result<PdfParseResult, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("读取PDF失败: {}", e))?;
    
    // 提取文本
    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| format!("解析PDF失败: {}", e))?;
    
    Ok(PdfParseResult {
        text: text.clone(),
        pages: vec![PdfPage {
            page_num: 1,
            text,
            width: 595.0,  // A4 默认宽度
            height: 842.0, // A4 默认高度
        }],
    })
}

/// PDF 脱敏处理器
pub struct PdfMasker;

impl PdfMasker {
    /// 对 PDF 进行脱敏处理
    pub fn mask_pdf(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 读取原始 PDF
        let bytes = std::fs::read(input_path)
            .map_err(|e| format!("读取PDF失败: {}", e))?;
        
        // 使用 lopdf 进行处理
        let mut doc = lopdf::Document::load_mem(&bytes)
            .map_err(|e| format!("解析PDF失败: {}", e))?;
        
        // 遍历所有页面
        let pages: Vec<(u32, lopdf::ObjectId)> = doc.get_pages().into_iter().collect();
        
        for (_page_num, page_id) in pages {
            if let Ok(page) = doc.get_object(page_id) {
                if let lopdf::Object::Dictionary(dict) = page {
                    // 获取内容流
                    if let Ok(contents) = dict.get(b"Contents") {
                        match contents {
                            lopdf::Object::Reference(stream_id) => {
                                // 解引用：*stream_id 将 &(u32, u16) 转换为 (u32, u16)
                                let obj_id = *stream_id;
                                if let Ok(stream_obj) = doc.get_object(obj_id) {
                                    if let lopdf::Object::Stream(s) = stream_obj.clone() {
                                        if let Ok(content) = s.decompressed_content() {
                                            let mut text = String::from_utf8_lossy(&content).to_string();
                                            
                                            // 执行替换
                                            for (original, masked) in replacements {
                                                text = text.replace(original, masked);
                                            }
                                            
                                            // 更新流内容
                                            if let Ok(stream) = doc.get_object_mut(obj_id) {
                                                if let lopdf::Object::Stream(ref mut s) = stream {
                                                    s.set_content(text.into_bytes());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            lopdf::Object::Array(arr) => {
                                // 处理多个内容流 - 先收集所有 stream_id
                                let stream_ids: Vec<lopdf::ObjectId> = arr.iter()
                                    .filter_map(|obj| {
                                        if let lopdf::Object::Reference(id) = obj {
                                            Some(*id)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect();
                                
                                for obj_id in stream_ids {
                                    if let Ok(stream_obj) = doc.get_object(obj_id) {
                                        if let lopdf::Object::Stream(s) = stream_obj.clone() {
                                            if let Ok(content) = s.decompressed_content() {
                                                let mut text = String::from_utf8_lossy(&content).to_string();
                                                
                                                // 执行替换
                                                for (original, masked) in replacements {
                                                    text = text.replace(original, masked);
                                                }
                                                
                                                // 更新流内容
                                                if let Ok(stream) = doc.get_object_mut(obj_id) {
                                                    if let lopdf::Object::Stream(ref mut s) = stream {
                                                        s.set_content(text.into_bytes());
                                                    }
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
        
        // 保存修改后的 PDF
        doc.save(output_path)
            .map_err(|e| format!("保存PDF失败: {}", e))?;
        
        Ok(())
    }
}
