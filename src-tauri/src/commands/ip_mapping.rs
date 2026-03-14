// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// IP映射命令

//! IP映射相关Tauri命令

use crate::services::{IP_MAPPER, IPMappingRecord, MappingStrategy};
use crate::services::crypto::Encryptor;
use std::sync::PoisonError;
use std::sync::MutexGuard;
use base64::Engine;

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
pub async fn import_ip_mappings(
    records: Vec<IPMappingRecord>,
    encrypted: Option<bool>,
    password: Option<String>,
    encrypted_data: Option<String>,
    salt: Option<String>
) -> Result<(), String> {
    let final_records = if encrypted.unwrap_or(false) {
        // 解密数据
        let pwd = password.ok_or("加密导入需要提供密码")?;
        let encrypted = encrypted_data.ok_or("加密导入需要提供加密数据")?;
        let salt_b64 = salt.ok_or("加密导入需要提供盐值")?;
        
        // 解码盐值
        let salt_bytes = base64::engine::general_purpose::STANDARD
            .decode(&salt_b64)
            .map_err(|e| format!("盐值解码失败: {}", e))?;
        
        // 创建加密器并解密
        let encryptor = Encryptor::from_password(&pwd, &salt_bytes);
        let decrypted = encryptor.decrypt(&encrypted)?;
        
        // 解析记录
        serde_json::from_str(&decrypted)
            .map_err(|e| format!("解析解密数据失败: {}", e))?
    } else {
        records
    };
    
    let mut mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    mapper.import_mappings(final_records);
    Ok(())
}

/// 导出IP映射表
#[tauri::command]
pub async fn export_ip_mappings(
    encrypt: Option<bool>,
    password: Option<String>
) -> Result<serde_json::Value, String> {
    let mapper: MutexGuard<crate::services::ip_mapper::IPMapper> = 
        IP_MAPPER.lock().map_err(|e: PoisonError<MutexGuard<crate::services::ip_mapper::IPMapper>>| e.to_string())?;
    let records = mapper.export_mappings();
    
    if encrypt.unwrap_or(false) {
        // 加密导出
        let pwd = password.ok_or("加密导出需要提供密码")?;
        let salt = Encryptor::generate_salt();
        let encryptor = Encryptor::from_password(&pwd, &salt);
        
        let json_data = serde_json::to_string(&records)
            .map_err(|e| format!("序列化失败: {}", e))?;
        let encrypted = encryptor.encrypt(&json_data)?;
        
        // 返回加密数据和盐值
        Ok(serde_json::json!({
            "encrypted": true,
            "salt": base64::engine::general_purpose::STANDARD.encode(&salt),
            "data": encrypted
        }))
    } else {
        // 普通导出
        Ok(serde_json::to_value(records)
            .map_err(|e| format!("序列化失败: {}", e))?)
    }
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
