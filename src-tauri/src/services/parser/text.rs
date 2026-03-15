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
#[derive(Debug)]
#[allow(dead_code)]
pub struct TextParseResult {
    pub text: String,
    pub encoding: String,
    pub line_count: usize,
}

/// 解析文本文件
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct TextMasker;

impl TextMasker {
    /// 对文本文件进行脱敏处理
    #[allow(dead_code)]
    pub fn mask_text(content: &str, replacements: &[(String, String)]) -> String {
        let mut result = content.to_string();
        
        for (original, masked) in replacements {
            result = result.replace(original, masked);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_parse_utf8_text() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "这是UTF-8编码的测试文本\n第二行内容\n第三行";
        temp_file.write_all(content.as_bytes()).unwrap();
        
        let path = temp_file.path().to_path_buf();
        let result = parse_text(&path).unwrap();
        
        assert_eq!(result.encoding, "utf-8");
        assert_eq!(result.line_count, 3);
        assert!(result.text.contains("UTF-8"));
    }
    
    #[test]
    fn test_parse_gbk_text() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // GBK 编码的中文文本
        let gbk_bytes = encoding_rs::GBK.encode("这是GBK编码的测试").0;
        temp_file.write_all(&gbk_bytes).unwrap();
        
        let path = temp_file.path().to_path_buf();
        let result = parse_text(&path).unwrap();
        
        assert_eq!(result.encoding, "gbk");
        assert!(result.text.contains("GBK"));
    }
    
    #[test]
    fn test_parse_large_file_performance() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // 生成大约 1MB 的文本
        let line = "这是一行测试数据，用于测试大文件解析性能。".repeat(10);
        let content = (line + "\n").repeat(5000);
        temp_file.write_all(content.as_bytes()).unwrap();
        
        let path = temp_file.path().to_path_buf();
        let start = std::time::Instant::now();
        let result = parse_text(&path).unwrap();
        let elapsed = start.elapsed();
        
        assert_eq!(result.line_count, 5000);
        // 性能要求：1MB 文件解析应在 1 秒内完成
        assert!(elapsed.as_millis() < 1000, "大文件解析耗时: {:?}", elapsed);
    }
    
    #[test]
    fn test_parse_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"").unwrap();
        
        let path = temp_file.path().to_path_buf();
        let result = parse_text(&path).unwrap();
        
        assert_eq!(result.encoding, "utf-8");
        assert_eq!(result.line_count, 0);
        assert!(result.text.is_empty());
    }
    
    #[test]
    fn test_parse_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/path/file.txt");
        let result = parse_text(&path);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("读取文件失败"));
    }
    
    #[test]
    fn test_text_masker() {
        let content = "手机号13812345678需要脱敏";
        let replacements = vec![
            ("13812345678".to_string(), "138****5678".to_string()),
        ];
        
        let result = TextMasker::mask_text(content, &replacements);
        assert_eq!(result, "手机号138****5678需要脱敏");
    }
}
