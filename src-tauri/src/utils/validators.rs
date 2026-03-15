// 产品设计: wonder-宏
// 架构设计/开发实现: JARVIS AI Assistant
// 
// 核心特性:
// - 本地处理，零上传（解决云端泄露痛点）
// - 格式无损保持（解决格式破坏痛点）
// - 预览确认机制（解决无预览痛点）
// - 自定义规则系统（解决规则固化痛点）

//! 校验工具
//! 
//! 包含身份证、银行卡号、统一社会信用代码等校验功能

/// 身份证号校验
#[allow(dead_code)]
pub struct IdCardValidator;

impl IdCardValidator {
    /// 校验身份证号
    #[allow(dead_code)]
    pub fn validate(id: &str) -> bool {
        let id = id.to_uppercase();
        
        // 15位身份证
        if id.len() == 15 {
            return id.chars().all(|c| c.is_ascii_digit());
        }
        
        // 18位身份证
        if id.len() == 18 {
            if !id[..17].chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
            
            let last_char = id.chars().last().unwrap();
            if !last_char.is_ascii_digit() && last_char != 'X' {
                return false;
            }
            
            // 校验码验证
            let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
            let check_codes = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
            
            let mut sum: u32 = 0;
            for (i, c) in id[..17].chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    sum += d * weights[i];
                }
            }
            
            let check_index = (sum % 11) as usize;
            let expected_check = check_codes[check_index];
            
            return id.chars().last().unwrap() == expected_check;
        }
        
        false
    }
    
    /// 获取身份证信息
    #[allow(dead_code)]
    pub fn get_info(id: &str) -> Option<IdCardInfo> {
        if !Self::validate(id) {
            return None;
        }
        
        let id = id.to_uppercase();
        
        // 提取地区码
        let area_code = id[..6].to_string();
        
        // 提取出生日期
        let birth_date = if id.len() == 18 {
            format!("{}-{}-{}", &id[6..10], &id[10..12], &id[12..14])
        } else {
            format!("19{}-{}-{}", &id[6..8], &id[8..10], &id[10..12])
        };
        
        // 性别
        let gender_digit = if id.len() == 18 {
            id.chars().nth(16).unwrap()
        } else {
            id.chars().nth(14).unwrap()
        };
        let gender = if gender_digit.to_digit(10).unwrap() % 2 == 1 {
            "男"
        } else {
            "女"
        };
        
        Some(IdCardInfo {
            area_code,
            birth_date,
            gender: gender.to_string(),
        })
    }
}

/// 身份证信息
#[allow(dead_code)]
pub struct IdCardInfo {
    pub area_code: String,
    pub birth_date: String,
    pub gender: String,
}

/// 银行卡号校验（Luhn算法）
#[allow(dead_code)]
pub struct BankCardValidator;

impl BankCardValidator {
    /// 校验银行卡号
    #[allow(dead_code)]
    pub fn validate(number: &str) -> bool {
        let digits: Vec<u32> = number.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() < 13 || digits.len() > 19 {
            return false;
        }
        
        let mut sum = 0;
        let mut alternate = false;
        
        for &digit in digits.iter().rev() {
            let mut n = digit;
            
            if alternate {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            
            sum += n;
            alternate = !alternate;
        }
        
        sum % 10 == 0
    }
    
    /// 获取银行信息（基于BIN号）
    #[allow(dead_code)]
    pub fn get_bank_info(number: &str) -> Option<BankInfo> {
        let bin = &number[..6.min(number.len())];
        
        // 常见银行BIN号
        let banks = [
            ("621700", "中国银行", "借记卡"),
            ("622280", "工商银行", "借记卡"),
            ("622848", "农业银行", "借记卡"),
            ("622700", "建设银行", "借记卡"),
            ("622588", "招商银行", "借记卡"),
            ("621284", "工商银行", "信用卡"),
            ("622580", "招商银行", "信用卡"),
        ];
        
        for (prefix, bank, card_type) in banks {
            if bin.starts_with(prefix) {
                return Some(BankInfo {
                    bank: bank.to_string(),
                    card_type: card_type.to_string(),
                });
            }
        }
        
        None
    }
}

/// 银行信息
#[allow(dead_code)]
pub struct BankInfo {
    pub bank: String,
    pub card_type: String,
}

/// 统一社会信用代码校验
#[allow(dead_code)]
pub struct CreditCodeValidator;

impl CreditCodeValidator {
    /// 校验统一社会信用代码
    #[allow(dead_code)]
    pub fn validate(code: &str) -> bool {
        if code.len() != 18 {
            return false;
        }
        
        let code = code.to_uppercase();
        
        // 检查字符是否合法
        let valid_chars = "0123456789ABCDEFGHJKLMNPQRTUWXY";
        if !code.chars().all(|c| valid_chars.contains(c)) {
            return false;
        }
        
        // 校验码验证
        let weights = [1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28];
        let check_chars = "0123456789ABCDEFGHJKLMNPQRTUWXY";
        
        let mut sum: u32 = 0;
        for (i, c) in code[..17].chars().enumerate() {
            let value = check_chars.find(c).unwrap() as u32;
            sum += value * weights[i];
        }
        
        let check_index = (31 - (sum % 31)) % 31;
        let expected_check = check_chars.chars().nth(check_index as usize).unwrap();
        
        code.chars().last().unwrap() == expected_check
    }
}

/// 文件路径安全验证
#[allow(dead_code)]
pub struct PathValidator;

impl PathValidator {
    /// 检测路径遍历攻击
    #[allow(dead_code)]
    pub fn is_path_traversal(path: &str) -> bool {
        let normalized = path.replace('\\', "/");
        
        // 检测 ../ 或 ..\
        if normalized.contains("../") {
            return true;
        }
        
        // 检测以 .. 开头
        if normalized.starts_with("..") {
            return true;
        }
        
        // 检测过多的目录层级跳跃
        let depth = normalized.split('/').fold(0i32, |acc, part| {
            if part == ".." { acc - 1 } else if !part.is_empty() && part != "." { acc + 1 } else { acc }
        });
        
        depth < 0
    }
    
    /// 验证文件名安全
    #[allow(dead_code)]
    pub fn is_safe_filename(filename: &str) -> bool {
        // 空文件名不安全
        if filename.is_empty() {
            return false;
        }
        
        // 检测危险字符
        let dangerous_chars = ['<', '>', ':', '"', '|', '?', '*', '\0'];
        if filename.chars().any(|c| dangerous_chars.contains(&c)) {
            return false;
        }
        
        // 检测保留名称（Windows）
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];
        
        let upper_name = filename.to_uppercase();
        let name_without_ext = upper_name.split('.').next().unwrap_or("");
        
        if reserved_names.contains(&name_without_ext) {
            return false;
        }
        
        // 检测以点开头的隐藏文件（可选，视安全策略而定）
        // 这里允许隐藏文件，但不允许只包含点的文件
        if filename.chars().all(|c| c == '.') {
            return false;
        }
        
        true
    }
}

/// 命令注入检测
#[allow(dead_code)]
pub struct CommandInjectionValidator;

impl CommandInjectionValidator {
    /// 检测命令注入攻击
    #[allow(dead_code)]
    pub fn has_command_injection(input: &str) -> bool {
        // Shell 命令分隔符
        let dangerous_patterns = [
            "&&", "||", "|", ";", "\n", "\r",
            "$(", "${", "`",
            ">", ">>", "<",
            "$(", "$[", "${",
        ];
        
        for pattern in dangerous_patterns {
            if input.contains(pattern) {
                return true;
            }
        }
        
        // 检测潜在的命令注入关键字
        let dangerous_commands = [
            "rm ", "del ", "format ", "shutdown", "reboot",
            "eval ", "exec ", "system(", "popen(",
            "/bin/sh", "/bin/bash", "cmd.exe", "powershell",
        ];
        
        let lower_input = input.to_lowercase();
        for cmd in dangerous_commands {
            if lower_input.contains(cmd) {
                return true;
            }
        }
        
        false
    }
    
    /// 安全地转义输入
    #[allow(dead_code)]
    pub fn escape_shell_arg(input: &str) -> String {
        // 使用单引号包裹，并转义内部的单引号
        format!("'{}'", input.replace("'", "'\\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_id_card_validation() {
        // 有效身份证号（测试用）
        assert!(IdCardValidator::validate("11010519491231002X"));
        
        // 无效身份证号
        assert!(!IdCardValidator::validate("110105194912310021"));
        assert!(!IdCardValidator::validate("123456789012345678"));
    }
    
    #[test]
    fn test_bank_card_validation() {
        // 有效银行卡号（测试用）
        assert!(BankCardValidator::validate("4532015112830366"));
        
        // 无效银行卡号
        assert!(!BankCardValidator::validate("4532015112830367"));
    }
    
    #[test]
    fn test_credit_code_validation() {
        // 有效统一社会信用代码（校验码已验证）
        assert!(CreditCodeValidator::validate("91110000600007336U"));
        assert!(CreditCodeValidator::validate("91110000600007333J"));
        
        // 无效代码（校验码错误）
        assert!(!CreditCodeValidator::validate("91110000600007333X"));
        assert!(!CreditCodeValidator::validate("91110000600007336C")); // 校验码错误
        assert!(!CreditCodeValidator::validate("12345")); // 长度不对
        assert!(!CreditCodeValidator::validate("91110000600007336")); // 少一位
    }
    
    // ==================== 路径遍历攻击测试 ====================
    
    #[test]
    fn test_path_traversal_basic() {
        // 基本的路径遍历攻击
        assert!(PathValidator::is_path_traversal("../../../etc/passwd"));
        assert!(PathValidator::is_path_traversal("..\\..\\windows\\system32"));
        assert!(PathValidator::is_path_traversal("/safe/path/../../../etc/passwd"));
    }
    
    #[test]
    fn test_path_traversal_safe_paths() {
        // 安全路径
        assert!(!PathValidator::is_path_traversal("/var/data/file.txt"));
        assert!(!PathValidator::is_path_traversal("documents/report.pdf"));
        assert!(!PathValidator::is_path_traversal("./current/dir"));
    }
    
    #[test]
    fn test_path_traversal_encoded() {
        // URL 编码的路径遍历（当前实现不解码，标记为安全）
        // 如果需要，可以扩展支持 URL 解码后的检测
        assert!(!PathValidator::is_path_traversal("%2e%2e%2f"));
    }
    
    #[test]
    fn test_path_traversal_deep() {
        // 深层目录跳跃
        assert!(PathValidator::is_path_traversal("../../../../../../etc/passwd"));
        assert!(PathValidator::is_path_traversal("a/b/c/../../../d")); // 跳出根目录
    }
    
    // ==================== 文件名安全测试 ====================
    
    #[test]
    fn test_safe_filename_valid() {
        // 安全的文件名
        assert!(PathValidator::is_safe_filename("document.pdf"));
        assert!(PathValidator::is_safe_filename("report_2024.xlsx"));
        assert!(PathValidator::is_safe_filename("数据文件.docx"));
        assert!(PathValidator::is_safe_filename(".hidden_file"));
    }
    
    #[test]
    fn test_safe_filename_invalid() {
        // 不安全的文件名
        assert!(!PathValidator::is_safe_filename("")); // 空
        assert!(!PathValidator::is_safe_filename("file<name>.txt")); // 危险字符
        assert!(!PathValidator::is_safe_filename("file|name.txt"));
        assert!(!PathValidator::is_safe_filename("file:name.txt"));
        assert!(!PathValidator::is_safe_filename("file\"name.txt"));
        assert!(!PathValidator::is_safe_filename("file?name.txt"));
        assert!(!PathValidator::is_safe_filename("file*name.txt"));
        assert!(!PathValidator::is_safe_filename("...")); // 只有点
    }
    
    #[test]
    fn test_safe_filename_reserved() {
        // Windows 保留名称
        assert!(!PathValidator::is_safe_filename("CON"));
        assert!(!PathValidator::is_safe_filename("PRN.txt"));
        assert!(!PathValidator::is_safe_filename("AUX.log"));
        assert!(!PathValidator::is_safe_filename("NUL"));
        assert!(!PathValidator::is_safe_filename("COM1"));
        assert!(!PathValidator::is_safe_filename("LPT1.txt"));
    }
    
    // ==================== 命令注入检测测试 ====================
    
    #[test]
    fn test_command_injection_basic() {
        // 基本命令注入
        assert!(CommandInjectionValidator::has_command_injection("file.txt; rm -rf /"));
        assert!(CommandInjectionValidator::has_command_injection("test && cat /etc/passwd"));
        assert!(CommandInjectionValidator::has_command_injection("data || echo hacked"));
        assert!(CommandInjectionValidator::has_command_injection("input | grep password"));
    }
    
    #[test]
    fn test_command_injection_safe() {
        // 安全输入
        assert!(!CommandInjectionValidator::has_command_injection("normal_filename.txt"));
        assert!(!CommandInjectionValidator::has_command_injection("report-2024-12.pdf"));
        assert!(!CommandInjectionValidator::has_command_injection("数据文件_最终版.docx"));
    }
    
    #[test]
    fn test_command_injection_substitution() {
        // 命令替换
        assert!(CommandInjectionValidator::has_command_injection("$(whoami)"));
        assert!(CommandInjectionValidator::has_command_injection("${PATH}"));
        assert!(CommandInjectionValidator::has_command_injection("`id`"));
    }
    
    #[test]
    fn test_command_injection_redirection() {
        // 重定向攻击
        assert!(CommandInjectionValidator::has_command_injection("file > /etc/passwd"));
        assert!(CommandInjectionValidator::has_command_injection("data >> ~/.bashrc"));
        assert!(CommandInjectionValidator::has_command_injection("input < /etc/shadow"));
    }
    
    #[test]
    fn test_command_injection_dangerous_commands() {
        // 危险命令关键字
        assert!(CommandInjectionValidator::has_command_injection("script.sh rm -rf /"));
        assert!(CommandInjectionValidator::has_command_injection("shutdown -h now"));
        assert!(CommandInjectionValidator::has_command_injection("/bin/bash -c 'evil'"));
        assert!(CommandInjectionValidator::has_command_injection("cmd.exe /c dir"));
    }
    
    #[test]
    fn test_shell_arg_escaping() {
        // 测试参数转义
        let escaped = CommandInjectionValidator::escape_shell_arg("file; rm -rf /");
        // 转义后应该用单引号包裹
        assert!(escaped.starts_with("'"));
        assert!(escaped.ends_with("'"));
        // 整个字符串被引号包裹，shell 不会执行其中的命令
        assert_eq!(escaped, "'file; rm -rf /'");
        
        // 测试包含单引号的输入
        let escaped_quote = CommandInjectionValidator::escape_shell_arg("it's a test");
        // 单引号应该被转义
        assert!(escaped_quote.contains("'\\''"));
        assert_eq!(escaped_quote, "'it'\\''s a test'");
    }
    
    // ==================== 边界条件测试 ====================
    
    #[test]
    fn test_empty_input_safety() {
        // 空输入
        assert!(!PathValidator::is_path_traversal(""));
        assert!(!CommandInjectionValidator::has_command_injection(""));
    }
    
    #[test]
    fn test_unicode_input() {
        // Unicode 输入
        assert!(PathValidator::is_safe_filename("中文文件名.txt"));
        assert!(!CommandInjectionValidator::has_command_injection("正常的中文输入"));
    }
    
    #[test]
    fn test_special_chars_in_path() {
        // 路径中的特殊字符
        assert!(!PathValidator::is_path_traversal("/safe/path/file.txt"));
        assert!(PathValidator::is_path_traversal("/safe/../other/file.txt"));
    }
}
