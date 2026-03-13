// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! Data Masker - 文件脱敏工具
//! 
//! 所有处理100%本地完成，敏感数据不上传云端

mod commands;
mod services;
mod models;
mod utils;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化日志系统
            init_logger();
            
            // 获取应用数据目录（Tauri v2 API）
            let app_data_dir = app.path().app_data_dir()
                .expect("无法获取应用数据目录");
            std::fs::create_dir_all(&app_data_dir)
                .expect("无法创建应用数据目录");
            
            // 创建临时文件目录
            let temp_dir = app_data_dir.join("temp");
            std::fs::create_dir_all(&temp_dir)
                .expect("无法创建临时文件目录");
            
            // 创建输出目录
            let output_dir = app_data_dir.join("output");
            std::fs::create_dir_all(&output_dir)
                .expect("无法创建输出目录");
            
            tracing::info!("Data Masker 应用启动成功");
            tracing::info!("数据目录: {:?}", app_data_dir);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 文件操作
            commands::file::select_files,
            commands::file::select_directory,
            commands::file::read_file_content,
            commands::file::read_file_preview,
            commands::file::save_file,
            commands::file::open_file_location,
            commands::file::open_output_directory,
            commands::file::clear_temp_files,
            
            // 脱敏处理
            commands::mask::detect_sensitive,
            commands::mask::apply_mask,
            commands::mask::process_file,
            
            // 规则管理
            commands::rule::get_builtin_rules,
            commands::rule::test_rule,
            commands::rule::validate_regex,
            
            // 预览
            commands::preview::generate_preview,
            
            // 导出
            commands::export::export_result,
            commands::export::export_all_results,
            commands::export::export_report,
            
            // 设置
            commands::settings::get_app_info,
            commands::settings::get_output_dir,
            
            // IP映射
            commands::ip_mapping::map_ip,
            commands::ip_mapping::map_ip_batch,
            commands::ip_mapping::get_ip_mappings,
            commands::ip_mapping::import_ip_mappings,
            commands::ip_mapping::export_ip_mappings,
            commands::ip_mapping::clear_ip_mappings,
            commands::ip_mapping::set_mapping_strategy,
            commands::ip_mapping::get_mapping_count,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}

/// 初始化日志系统
fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .init();
}
