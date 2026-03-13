// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// IP映射命令

//! IP映射相关Tauri命令

use crate::services::{IP_MAPPER, IPMappingRecord, MappingStrategy};
use std::sync::PoisonError;
use std::sync::MutexGuard;

/// 映射单个IP地址
#[tauri::command]
pub async fn map_ip(ip: String) -> Result<String, String> {
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    Ok(mapper.map(&ip))
}

/// 批量映射IP地址
#[tauri::command]
pub async fn map_ip_batch(ips: Vec<String>) -> Result<Vec<(String, String)>, String> {
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    Ok(mapper.map_batch(&ips))
}

/// 获取IP映射表
#[tauri::command]
pub async fn get_ip_mappings() -> Result<Vec<IPMappingRecord>, String> {
    let mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    Ok(mapper.get_mapping_records())
}

/// 导入IP映射表
#[tauri::command]
pub async fn import_ip_mappings(records: Vec<IPMappingRecord>) -> Result<(), String> {
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    mapper.import_mappings(records);
    Ok(())
}

/// 导出IP映射表
#[tauri::command]
pub async fn export_ip_mappings() -> Result<Vec<IPMappingRecord>, String> {
    let mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    Ok(mapper.export_mappings())
}

/// 清空IP映射表
#[tauri::command]
pub async fn clear_ip_mappings() -> Result<(), String> {
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    mapper.clear();
    Ok(())
}

/// 设置映射策略
#[tauri::command]
pub async fn set_mapping_strategy(
    internal_prefix: Option<String>,
    public_strategy: Option<String>
) -> Result<(), String> {
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    
    let strategy = match public_strategy.as_deref() {
        Some("hide") => MappingStrategy::FullHide,
        Some("mask") => MappingStrategy::PartialMask,
        _ => MappingStrategy::InternalToRFC1918 {
            target_prefix: internal_prefix.unwrap_or_else(|| "10.10".to_string()),
        },
    };
    
    mapper.set_strategy(strategy);
    Ok(())
}

/// 获取映射数量
#[tauri::command]
pub async fn get_mapping_count() -> Result<usize, String> {
    let mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    Ok(mapper.count())
}
