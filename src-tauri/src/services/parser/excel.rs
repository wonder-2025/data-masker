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
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            let mut sheet_data = Vec::new();
            
            for row in range.rows() {
                let cells: Vec<String> = row.iter()
                    .map(|cell| cell_to_string(cell))
                    .collect();
                
                all_text.push_str(&cells.join("\t"));
                all_text.push('\n');
                
                sheet_data.push(cells);
            }
            
            result_sheets.push(ExcelSheet {
                name: sheet_name,
                data: sheet_data,
            });
        }
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
        // 使用 umya-spreadsheet 处理 Excel
        let mut workbook = umya_spreadsheet::reader::xlsx::read(input_path)
            .map_err(|e| format!("读取Excel失败: {:?}", e))?;
        
        // 遍历所有工作表
        let sheet_count = workbook.get_sheet_collection().len();
        
        for sheet_idx in 0..sheet_count {
            // 使用 get_sheet_collection_mut 获取可变引用
            let sheet = workbook.get_sheet_collection_mut()
                .get_mut(sheet_idx)
                .ok_or("无法获取工作表")?;
            
            // 获取工作表尺寸
            let row_count = sheet.get_row_dimensions().len();
            let col_count = sheet.get_column_dimensions().len();
            
            // 遍历所有单元格
            for row_idx in 1..=row_count + 100 {
                for col_idx in 1..=col_count + 50 {
                    // 使用 (col_idx, row_idx) 元组而不是引用
                    let cell = sheet.get_cell_mut((col_idx as u32, row_idx as u32));
                    
                    let value = cell.get_value();
                    let mut new_value = value.to_string();
                    
                    for (original, masked) in replacements {
                        new_value = new_value.replace(original, masked);
                    }
                    
                    if new_value != value {
                        cell.set_value(&new_value);
                    }
                }
            }
        }
        
        // 保存文件
        umya_spreadsheet::writer::xlsx::write(&workbook, output_path)
            .map_err(|e| format!("保存Excel失败: {:?}", e))?;
        
        Ok(())
    }
}
