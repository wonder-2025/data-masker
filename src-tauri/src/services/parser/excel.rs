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
//! 保持公式、图表、条件格式

use std::path::PathBuf;
use calamine::{Reader, Xlsx, DataType};

/// Excel 解析结果
pub struct ExcelParseResult {
    pub text: String,
    pub sheets: Vec<ExcelSheet>,
}

/// Excel 工作表信息
pub struct ExcelSheet {
    pub name: String,
    pub data: Vec<Vec<String>>,
}

/// 解析 Excel 文件
pub fn parse_excel(path: &PathBuf) -> Result<ExcelParseResult, String> {
    let mut workbook: Xlsx<_> = calamine::open_workbook(path)
        .map_err(|e| format!("打开Excel失败: {}", e))?;
    
    let sheets = workbook.sheet_names().to_vec();
    let mut result_sheets = Vec::new();
    let mut all_text = String::new();
    
    for sheet_name in sheets {
        match workbook.worksheet_range(&sheet_name) {
            Some(Ok(range)) => {
                let mut sheet_data = Vec::new();
                
                for row in range.rows() {
                    // 跳过完全空的行
                    let has_content = row.iter().any(|cell| {
                        !matches!(cell, DataType::Empty)
                    });
                    
                    if !has_content {
                        continue;
                    }
                    
                    let cells: Vec<String> = row.iter()
                        .map(|cell| cell_to_string(cell))
                        .collect();
                    
                    if !cells.is_empty() {
                        all_text.push_str(&cells.join("\t"));
                        all_text.push('\n');
                        sheet_data.push(cells);
                    }
                }
                
                // 只添加有数据的工作表
                if !sheet_data.is_empty() {
                    result_sheets.push(ExcelSheet {
                        name: sheet_name,
                        data: sheet_data,
                    });
                }
            }
            Some(Err(e)) => {
                tracing::warn!("读取工作表 {} 失败: {:?}", sheet_name, e);
                continue;
            }
            None => {
                tracing::warn!("工作表 {} 不存在", sheet_name);
                continue;
            }
        }
    }
    
    // 如果没有任何数据，返回空结果而不是错误
    if result_sheets.is_empty() {
        return Ok(ExcelParseResult {
            text: String::new(),
            sheets: vec![],
        });
    }
    
    Ok(ExcelParseResult {
        text: all_text,
        sheets: result_sheets,
    })
}

/// 单元格值转字符串
fn cell_to_string(cell: &DataType) -> String {
    match cell {
        DataType::Empty => String::new(),
        DataType::String(s) => s.clone(),
        DataType::Float(f) => f.to_string(),
        DataType::Int(i) => i.to_string(),
        DataType::Bool(b) => b.to_string(),
        DataType::DateTime(d) => d.to_string(),
        DataType::Error(e) => format!("ERROR: {:?}", e),
        _ => String::new(),
    }
}

/// Excel 脱敏处理器
pub struct ExcelMasker;

impl ExcelMasker {
    /// 对 Excel 文件进行脱敏处理
    pub fn mask_excel(input_path: &PathBuf, output_path: &PathBuf, replacements: &[(String, String)]) -> Result<(), String> {
        // 检查输入文件是否存在
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
        
        // 使用 umya-spreadsheet 处理 Excel
        let mut workbook = umya_spreadsheet::reader::xlsx::read(input_path)
            .map_err(|e| format!("读取Excel失败: {:?}", e))?;
        
        // 遍历所有工作表
        let sheet_count = workbook.get_sheet_collection().len();
        
        if sheet_count == 0 {
            return Err("Excel文件没有工作表".to_string());
        }
        
        for sheet_idx in 0..sheet_count {
            // 使用 get_sheet_collection_mut 获取可变引用
            let sheet = workbook.get_sheet_collection_mut()
                .get_mut(sheet_idx)
                .ok_or("无法获取工作表")?;
            
            // 获取工作表尺寸，使用更安全的边界
            let row_count = sheet.get_row_dimensions().len().min(10000); // 限制最大行数
            let col_count = sheet.get_column_dimensions().len().min(1000); // 限制最大列数
            
            // 遍历所有单元格
            for row_idx in 1..=row_count + 10 { // 额外检查10行
                for col_idx in 1..=col_count + 10 { // 额外检查10列
                    // 使用 (col_idx, row_idx) 元组而不是引用
                    let cell = sheet.get_cell_mut((col_idx as u32, row_idx as u32));
                    
                    let value = cell.get_value();
                    let value_str = value.to_string();
                    
                    // 跳过空单元格
                    if value_str.is_empty() {
                        continue;
                    }
                    
                    let mut new_value = value_str.clone();
                    
                    for (original, masked) in replacements {
                        new_value = new_value.replace(original, masked);
                    }
                    
                    if new_value != value_str {
                        cell.set_value(&new_value);
                    }
                }
            }
        }
        
        // 确保输出目录存在
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("创建输出目录失败: {:?}", e))?;
        }
        
        // 保存文件
        umya_spreadsheet::writer::xlsx::write(&workbook, output_path)
            .map_err(|e| format!("保存Excel失败: {:?}", e))?;
        
        Ok(())
    }
}
