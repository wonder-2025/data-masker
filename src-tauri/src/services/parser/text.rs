// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 纯文本文件解析器

use std::path::PathBuf;

/// 文本解析结果
pub struct TextParseResult {
    pub text: String,
    pub encoding: String,
    pub line_count: usize,
}

/// 解析文本文件
pub fn parse_text(path: &PathBuf) -> Result<TextParseResult, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 尝试 UTF-8 解码
    let (text, encoding) = match String::from_utf8(bytes.clone()) {
        Ok(content) => (content, "utf-8".to_string()),
        Err(_) => {
            // 尝试 GBK 解码
            let (content, _, _) = encoding_rs::GBK.decode(&bytes);
            (content.to_string(), "gbk".to_string())
        }
    };
    
    let line_count = text.lines().count();
    
    Ok(TextParseResult {
        text,
        encoding,
        line_count,
    })
}

/// 文本文件脱敏处理器
pub struct TextMasker;

impl TextMasker {
    /// 对文本文件进行脱敏处理
    pub fn mask_text(content: &str, replacements: &[(String, String)]) -> String {
        let mut result = content.to_string();
        
        for (original, masked) in replacements {
            result = result.replace(original, masked);
        }
        
        result
    }
}
