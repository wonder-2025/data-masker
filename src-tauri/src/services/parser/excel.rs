// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! Excel 文件解析器
//! 
//! 支持解析和脱敏 .xlsx 和 .xls 格式 Excel 文件

use std::path::PathBuf;
use calamine::{Reader, Xlsx, Xls, Data};

/// Excel 解析结果
pub struct ExcelParseResult {
    pub text: String,
    pub sheet_count: usize,
    pub row_count: usize,
}

/// 解析 Excel 文件
pub fn parse_excel(path: &PathBuf) -> Result<ExcelParseResult, String> {
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
    
    // 获取文件扩展名
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // 根据扩展名选择解析器
    let result = match extension.as_str() {
        "xlsx" => parse_xlsx(path)?,
        "xls" => parse_xls(path)?,
        _ => return Err(format!("不支持的文件格式: {}", extension)),
    };
    
    Ok(result)
}

/// 解析 .xlsx 文件
fn parse_xlsx(path: &PathBuf) -> Result<ExcelParseResult, String> {
    let mut workbook: Xlsx<_> = calamine::open_workbook(path)
        .map_err(|e| format!("打开Excel文件失败: {}。文件可能已损坏或不是有效的 .xlsx 格式。", e))?;
    
    parse_workbook(&mut workbook)
}

/// 解析 .xls 文件
fn parse_xls(path: &PathBuf) -> Result<ExcelParseResult, String> {
    let mut workbook: Xls<_> = calamine::open_workbook(path)
        .map_err(|e| format!("打开Excel文件失败: {}。文件可能已损坏或不是有效的 .xls 格式。", e))?;
    
    parse_workbook(&mut workbook)
}

/// 解析工作簿内容
fn parse_workbook<R: std::io::Read + std::io::Seek>(workbook: &mut impl Reader<R>) -> Result<ExcelParseResult, String> {
    let sheets = workbook.sheet_names().to_vec();
    let sheet_count = sheets.len();
    
    let mut text = String::new();
    let mut total_rows = 0;
    
    for sheet_name in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            text.push_str(&format!("=== 工作表: {} ===\n", sheet_name));
            
            for row in range.rows() {
                let cells: Vec<String> = row.iter()
                    .map(|cell| format_cell_value(cell))
                    .collect();
                
                if !cells.iter().all(|c| c.is_empty()) {
                    text.push_str(&cells.join("\t"));
                    text.push('\n');
                    total_rows += 1;
                }
            }
            
            text.push('\n');
        }
    }
    
    if text.trim().is_empty() {
        return Err("Excel 文件为空或无法读取内容".to_string());
    }
    
    Ok(ExcelParseResult {
        text,
        sheet_count,
        row_count: total_rows,
    })
}

/// 格式化单元格值
fn format_cell_value(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.to_string(),
        Data::Float(f) => format_float(*f),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(dt) => {
            // Excel 日期时间格式
            if *dt >= 1.0 {
                let days = (*dt - 25569.0) as i64;
                let timestamp = days * 86400;
                if let Some(dt) = chrono::DateTime::from_timestamp(timestamp, 0) {
                    dt.format("%Y-%m-%d").to_string()
                } else {
                    format!("{:.5}", dt)
                }
            } else {
                format!("{:.5}", dt)
            }
        }
        Data::Error(e) => format!("#ERROR: {:?}", e),
        _ => cell.to_string(),
    }
}

/// 格式化浮点数（去除不必要的精度）
fn format_float(f: f64) -> String {
    if f.fract() == 0.0 && f.abs() < i64::MAX as f64 {
        format!("{:.0}", f)
    } else {
        // 限制小数位数
        let formatted = format!("{:.10}", f);
        // 移除末尾的零
        let trimmed = formatted.trim_end_matches('0');
        if trimmed.ends_with('.') {
            format!("{}0", trimmed)
        } else {
            trimmed.to_string()
        }
    }
}

/// Excel 脱敏处理器
pub struct ExcelMasker;

impl ExcelMasker {
    /// 对 Excel 文件进行脱敏处理
    /// 
    /// 注意：Excel 的复杂格式（公式、图表等）可能无法完全保留
    pub fn mask_excel(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
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
        
        // 获取文件扩展名
        let extension = input_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // 根据格式选择处理方法
        match extension.as_str() {
            "xlsx" => Self::mask_xlsx(input_path, output_path, replacements),
            "xls" => Self::mask_xls(input_path, output_path, replacements),
            _ => Err(format!("不支持的文件格式: {}", extension)),
        }
    }
    
    /// 处理 .xlsx 文件
    fn mask_xlsx(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 对于 xlsx 文件，我们可以修改 XML 内容
        // xlsx 是一个 ZIP 文件，包含 XML 文件
        
        use std::io::{Read, Cursor};
        use zip::ZipArchive;
        
        let file = std::fs::File::open(input_path)
            .map_err(|e| format!("打开文件失败: {}", e))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("解析ZIP失败: {}", e))?;
        
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
        
        // 处理每个文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("读取ZIP条目失败: {}", e))?;
            let file_name = file.name().to_string();
            
            // 检查是否是工作表文件
            if file_name.starts_with("xl/worksheets/") && file_name.ends_with(".xml") {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| format!("读取工作表内容失败: {}", e))?;
                
                // 执行替换
                for (original, masked) in replacements {
                    content = content.replace(original, masked);
                }
                
                // 写入修改后的内容
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入ZIP条目失败: {:?}", e))?;
                zip_writer.write_all(content.as_bytes())
                    .map_err(|e| format!("写入内容失败: {:?}", e))?;
            } else if file_name == "xl/sharedStrings.xml" {
                // 共享字符串表
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| format!("读取共享字符串失败: {}", e))?;
                
                // 执行替换
                for (original, masked) in replacements {
                    content = content.replace(original, masked);
                }
                
                // 写入修改后的内容
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入ZIP条目失败: {:?}", e))?;
                zip_writer.write_all(content.as_bytes())
                    .map_err(|e| format!("写入内容失败: {:?}", e))?;
            } else {
                // 直接复制其他文件
                zip_writer.start_file(&file_name, options)
                    .map_err(|e| format!("写入ZIP条目失败: {:?}", e))?;
                
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| format!("读取ZIP条目失败: {:?}", e))?;
                zip_writer.write_all(&buffer)
                    .map_err(|e| format!("写入内容失败: {:?}", e))?;
            }
        }
        
        zip_writer.finish()
            .map_err(|e| format!("完成ZIP写入失败: {:?}", e))?;
        
        Ok(())
    }
    
    /// 处理 .xls 文件
    fn mask_xls(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 对于 .xls 文件（OLE 格式），处理更加复杂
        // 当前实现：读取内容，生成警告，直接复制文件
        
        tracing::warn!("xls 格式的脱敏处理有限，建议转换为 xlsx 格式");
        
        // 检查是否有敏感信息需要处理
        let parse_result = parse_xls(input_path)?;
        let has_sensitive = replacements.iter()
            .any(|(original, _)| parse_result.text.contains(original));
        
        if has_sensitive {
            // 直接复制文件并给出警告
            std::fs::copy(input_path, output_path)
                .map_err(|e| format!("复制文件失败: {:?}", e))?;
            
            return Err("xls 格式的脱敏处理有限。\n\n建议:\n1. 将文件转换为 .xlsx 格式后处理\n2. 或导出为 CSV 格式后处理\n\n已复制原文件到输出目录。".to_string());
        } else {
            // 没有敏感信息，直接复制
            std::fs::copy(input_path, output_path)
                .map_err(|e| format!("复制文件失败: {:?}", e))?;
            
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_float() {
        assert_eq!(format_float(123.0), "123");
        assert_eq!(format_float(123.456), "123.456");
    }
}
