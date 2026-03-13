// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::Manager;

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,
    pub is_local_processing: bool,
    pub security_note: String,
}

/// 获取应用信息
#[command]
pub async fn get_app_info() -> Result<AppInfo, String> {
    Ok(AppInfo {
        name: "Data Masker".to_string(),
        version: "1.0.0".to_string(),
        description: "文件脱敏工具 - 本地桌面应用，敏感数据不上传云端".to_string(),
        authors: vec![
            "wonder-宏 (产品设计)".to_string(),
            "JARVIS AI Assistant (架构设计/开发实现)".to_string(),
        ],
        license: "MIT".to_string(),
        is_local_processing: true,
        security_note: "所有数据处理100%在本地完成，不会上传到任何服务器".to_string(),
    })
}

/// 获取输出目录
#[command]
pub async fn get_output_dir(app: tauri::AppHandle) -> Result<String, String> {
    let output_dir = app.path()
        .app_data_dir()
        .map(|p| p.join("output"))
        .map_err(|e| format!("无法获取输出目录: {}", e))?;
    
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    Ok(output_dir.to_string_lossy().to_string())
}
