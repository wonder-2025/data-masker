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
pub struct IdCardValidator;

impl IdCardValidator {
    /// 校验身份证号
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
pub struct IdCardInfo {
    pub area_code: String,
    pub birth_date: String,
    pub gender: String,
}

/// 银行卡号校验（Luhn算法）
pub struct BankCardValidator;

impl BankCardValidator {
    /// 校验银行卡号
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
pub struct BankInfo {
    pub bank: String,
    pub card_type: String,
}

/// 统一社会信用代码校验
pub struct CreditCodeValidator;

impl CreditCodeValidator {
    /// 校验统一社会信用代码
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
        // 有效统一社会信用代码
        assert!(CreditCodeValidator::validate("91110000600007333F"));
        
        // 无效代码
        assert!(!CreditCodeValidator::validate("91110000600007333X"));
    }
}
