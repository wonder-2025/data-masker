// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

pub mod detector;
pub mod masker;
pub mod parser;
pub mod crypto;
pub mod logger;
pub mod ip_mapper;

pub use ip_mapper::{IP_MAPPER, IPMappingRecord, MappingStrategy};
