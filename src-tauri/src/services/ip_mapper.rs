// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// IP智能映射引擎 - 核心特性：
// - 保持网络拓扑关系
// - 同一IP多次映射结果一致
// - 内网IP映射到RFC 1918私有地址
// - 公网IP映射到RFC 5737文档地址

//! IP智能映射引擎

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// 全局IP映射器实例
pub static IP_MAPPER: Lazy<Arc<Mutex<IPMapper>>> = Lazy::new(|| {
    Arc::new(Mutex::new(IPMapper::new()))
});

/// IP映射策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MappingStrategy {
    /// 内网IP映射到RFC 1918私有地址
    InternalToRFC1918 {
        target_prefix: String,
    },
    /// 公网IP映射到RFC 5737文档地址
    PublicToRFC5737,
    /// 完全隐藏
    FullHide,
    /// 部分掩码
    PartialMask,
}

/// IP映射记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPMappingRecord {
    pub original_ip: String,
    pub mapped_ip: String,
    pub ip_type: String,
    pub subnet: Option<String>,
    pub timestamp: i64,
}

/// IP映射器 - 保持网络拓扑关系
#[derive(Debug)]
pub struct IPMapper {
    /// 已建立的映射关系
    mapping: HashMap<String, String>,
    /// 映射策略
    strategy: MappingStrategy,
}

impl IPMapper {
    /// 创建新的IP映射器
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            strategy: MappingStrategy::InternalToRFC1918 {
                target_prefix: "10.10".to_string(),
            },
        }
    }
    
    /// 设置映射策略
    pub fn set_strategy(&mut self, strategy: MappingStrategy) {
        self.strategy = strategy;
    }
    
    /// 映射单个IP地址
    pub fn map(&mut self, ip: &str) -> String {
        // 已映射则返回缓存结果
        if let Some(mapped) = self.mapping.get(ip) {
            return mapped.clone();
        }
        
        // 解析IP地址
        let parsed: Option<IpAddr> = ip.parse().ok();
        
        let mapped = if let Some(IpAddr::V4(v4)) = parsed {
            if self.is_internal_ipv4(&v4) {
                self.map_internal_ipv4(&v4)
            } else {
                self.map_public_ipv4(&v4)
            }
        } else {
            ip.to_string()
        };
        
        self.mapping.insert(ip.to_string(), mapped.clone());
        mapped
    }
    
    /// 判断是否为内网IPv4地址
    fn is_internal_ipv4(&self, ip: &Ipv4Addr) -> bool {
        let octets = ip.octets();
        octets[0] == 10 ||
        (octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31) ||
        (octets[0] == 192 && octets[1] == 168) ||
        octets[0] == 127
    }
    
    /// 映射内网IPv4地址
    fn map_internal_ipv4(&mut self, ip: &Ipv4Addr) -> String {
        let octets = ip.octets();
        
        match &self.strategy {
            MappingStrategy::InternalToRFC1918 { target_prefix } => {
                let prefix_parts: Vec<u8> = target_prefix
                    .split('.')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                let mapped = match prefix_parts.len() {
                    2 => Ipv4Addr::new(prefix_parts[0], prefix_parts[1], octets[2], octets[3]),
                    1 => Ipv4Addr::new(prefix_parts[0], 10, octets[2], octets[3]),
                    _ => Ipv4Addr::new(10, 10, octets[2], octets[3]),
                };
                mapped.to_string()
            }
            MappingStrategy::FullHide => "[IP_HIDDEN]".to_string(),
            MappingStrategy::PartialMask => format!("{}.{}.x.x", octets[0], octets[1]),
            _ => Ipv4Addr::new(10, 10, octets[2], octets[3]).to_string(),
        }
    }
    
    /// 映射公网IPv4地址到RFC 5737
    fn map_public_ipv4(&mut self, ip: &Ipv4Addr) -> String {
        let octets = ip.octets();
        
        match &self.strategy {
            MappingStrategy::PublicToRFC5737 => Ipv4Addr::new(203, 0, 113, octets[3]).to_string(),
            MappingStrategy::FullHide => "[PUBLIC_IP_HIDDEN]".to_string(),
            MappingStrategy::PartialMask => format!("x.x.x.{}", octets[3]),
            _ => Ipv4Addr::new(203, 0, 113, octets[3]).to_string(),
        }
    }
    
    /// 批量映射IP地址
    pub fn map_batch(&mut self, ips: &[String]) -> Vec<(String, String)> {
        ips.iter().map(|ip| (ip.clone(), self.map(ip))).collect()
    }
    
    /// 获取映射记录列表
    pub fn get_mapping_records(&self) -> Vec<IPMappingRecord> {
        self.mapping.iter().map(|(original, mapped)| {
            let ip_type = if let Ok(IpAddr::V4(v4)) = original.parse::<IpAddr>() {
                if self.is_internal_ipv4(&v4) { "internal" } else { "public" }
            } else { "unknown" };
            
            IPMappingRecord {
                original_ip: original.clone(),
                mapped_ip: mapped.clone(),
                ip_type: ip_type.to_string(),
                subnet: None,
                timestamp: chrono::Utc::now().timestamp(),
            }
        }).collect()
    }
    
    /// 导入映射表
    pub fn import_mappings(&mut self, records: Vec<IPMappingRecord>) {
        for record in records {
            self.mapping.insert(record.original_ip, record.mapped_ip);
        }
    }
    
    /// 导出映射表
    pub fn export_mappings(&self) -> Vec<IPMappingRecord> {
        self.get_mapping_records()
    }
    
    /// 清空映射表
    pub fn clear(&mut self) {
        self.mapping.clear();
    }
    
    /// 获取映射数量
    pub fn count(&self) -> usize {
        self.mapping.len()
    }
}

impl Default for IPMapper {
    fn default() -> Self {
        Self::new()
    }
}
