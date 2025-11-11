// 代码质量评分系统
use anyhow::Result;
use serde::{Serialize, Deserialize};

/// 代码质量评分结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    /// 总分 (0-100)
    pub total: u8,
    
    /// 各维度得分
    pub dimensions: ScoreDimensions,
    
    /// 发现的缺陷
    pub flaws: Vec<CodeFlaw>,
    
    /// 改进建议
    pub suggestions: Vec<String>,
}

/// 评分维度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDimensions {
    /// 正确性 (0-100)
    pub correctness: u8,
    
    /// 可读性 (0-100)
    pub readability: u8,
    
    /// 可维护性 (0-100)
    pub maintainability: u8,
    
    /// 性能 (0-100)
    pub performance: u8,
    
    /// 安全性 (0-100)
    pub security: u8,
    
    /// 测试覆盖 (0-100)
    pub test_coverage: u8,
}

/// 代码缺陷
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFlaw {
    /// 缺陷类型
    pub flaw_type: FlawType,
    
    /// 严重程度
    pub severity: Severity,
    
    /// 描述
    pub description: String,
    
    /// 位置（可选）
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlawType {
    Syntax,           // 语法错误
    Logic,            // 逻辑错误
    Performance,      // 性能问题
    Security,         // 安全问题
    Style,            // 代码风格
    Documentation,    // 文档缺失
    TestCoverage,     // 测试覆盖不足
    ErrorHandling,    // 错误处理不当
    ResourceLeak,     // 资源泄漏
    Complexity,       // 复杂度过高
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,       // 信息
    Low,        // 低
    Medium,     // 中
    High,       // 高
    Critical,   // 严重
}

impl FlawType {
    pub fn as_str(&self) -> &str {
        match self {
            FlawType::Syntax => "语法错误",
            FlawType::Logic => "逻辑错误",
            FlawType::Performance => "性能问题",
            FlawType::Security => "安全问题",
            FlawType::Style => "代码风格",
            FlawType::Documentation => "文档缺失",
            FlawType::TestCoverage => "测试覆盖不足",
            FlawType::ErrorHandling => "错误处理不当",
            FlawType::ResourceLeak => "资源泄漏",
            FlawType::Complexity => "复杂度过高",
        }
    }
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Info => "信息",
            Severity::Low => "低",
            Severity::Medium => "中",
            Severity::High => "高",
            Severity::Critical => "严重",
        }
    }
    
    pub fn score_impact(&self) -> u8 {
        match self {
            Severity::Info => 2,
            Severity::Low => 5,
            Severity::Medium => 10,
            Severity::High => 20,
            Severity::Critical => 30,
        }
    }
}

/// 评估代码质量
pub fn evaluate_code_quality(code: &str, tests: &str) -> Result<QualityScore> {
    let mut flaws = Vec::new();
    let mut suggestions = Vec::new();
    
    // 1. 检查基本语法和结构
    let correctness = check_correctness(code, &mut flaws, &mut suggestions);
    
    // 2. 检查可读性
    let readability = check_readability(code, &mut flaws, &mut suggestions);
    
    // 3. 检查可维护性
    let maintainability = check_maintainability(code, &mut flaws, &mut suggestions);
    
    // 4. 检查性能
    let performance = check_performance(code, &mut flaws, &mut suggestions);
    
    // 5. 检查安全性
    let security = check_security(code, &mut flaws, &mut suggestions);
    
    // 6. 检查测试覆盖
    let test_coverage = check_test_coverage(code, tests, &mut flaws, &mut suggestions);
    
    // 计算总分（加权平均）
    let total = calculate_total_score(&ScoreDimensions {
        correctness,
        readability,
        maintainability,
        performance,
        security,
        test_coverage,
    });
    
    Ok(QualityScore {
        total,
        dimensions: ScoreDimensions {
            correctness,
            readability,
            maintainability,
            performance,
            security,
            test_coverage,
        },
        flaws,
        suggestions,
    })
}

fn calculate_total_score(dimensions: &ScoreDimensions) -> u8 {
    // 加权计算：正确性>安全性>可维护性>可读性>性能>测试覆盖
    let weighted_sum = 
        dimensions.correctness as f32 * 0.25 +
        dimensions.security as f32 * 0.20 +
        dimensions.maintainability as f32 * 0.20 +
        dimensions.readability as f32 * 0.15 +
        dimensions.performance as f32 * 0.10 +
        dimensions.test_coverage as f32 * 0.10;
    
    weighted_sum.round() as u8
}

fn check_correctness(code: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    // 检查基本语法结构
    let has_main_or_function = code.contains("fn ") || code.contains("function") || code.contains("def ");
    if !has_main_or_function {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Syntax,
            severity: Severity::High,
            description: "未找到函数定义".to_string(),
            location: None,
        });
        score = score.saturating_sub(20);
    }
    
    // 检查错误处理
    if !code.contains("Result") && !code.contains("?") && !code.contains("unwrap") && !code.contains("expect") {
        if !code.contains("try") && !code.contains("catch") && !code.contains("except") {
            flaws.push(CodeFlaw {
                flaw_type: FlawType::ErrorHandling,
                severity: Severity::Medium,
                description: "缺少错误处理机制".to_string(),
                location: None,
            });
            score = score.saturating_sub(10);
            suggestions.push("添加适当的错误处理（Result/Option/try-catch）".to_string());
        }
    }
    
    // 检查是否有未处理的unwrap/panic
    if code.contains(".unwrap()") && !code.contains("// SAFETY:") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::ErrorHandling,
            severity: Severity::Low,
            description: "使用了 unwrap() 可能导致panic".to_string(),
            location: None,
        });
        score = score.saturating_sub(5);
        suggestions.push("考虑使用 ? 或更安全的错误处理方式".to_string());
    }
    
    score
}

fn check_readability(code: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    let lines: Vec<&str> = code.lines().collect();
    
    // 检查代码长度
    if lines.len() > 100 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Style,
            severity: Severity::Low,
            description: format!("代码过长 ({} 行)，建议拆分", lines.len()),
            location: None,
        });
        score = score.saturating_sub(5);
        suggestions.push("将大型函数拆分为更小的功能单元".to_string());
    }
    
    // 检查单行长度
    for (i, line) in lines.iter().enumerate() {
        if line.len() > 120 {
            flaws.push(CodeFlaw {
                flaw_type: FlawType::Style,
                severity: Severity::Info,
                description: format!("第 {} 行过长 ({} 字符)", i + 1, line.len()),
                location: Some(format!("line {}", i + 1)),
            });
            score = score.saturating_sub(2);
        }
    }
    
    // 检查注释密度
    let comment_lines = lines.iter().filter(|l| {
        let trimmed = l.trim();
        trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("#")
    }).count();
    
    let comment_ratio = comment_lines as f32 / lines.len().max(1) as f32;
    if comment_ratio < 0.1 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Documentation,
            severity: Severity::Low,
            description: format!("注释比例较低 ({:.1}%)", comment_ratio * 100.0),
            location: None,
        });
        score = score.saturating_sub(8);
        suggestions.push("添加必要的注释说明复杂逻辑".to_string());
    }
    
    // 检查命名规范
    if code.contains("var1") || code.contains("temp") || code.contains("x1") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Style,
            severity: Severity::Low,
            description: "存在不具描述性的变量名".to_string(),
            location: None,
        });
        score = score.saturating_sub(5);
        suggestions.push("使用有意义的变量名".to_string());
    }
    
    score
}

fn check_maintainability(code: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    // 检查嵌套深度
    let max_nesting = calculate_max_nesting(code);
    if max_nesting > 4 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Complexity,
            severity: Severity::Medium,
            description: format!("嵌套层级过深 ({}层)", max_nesting),
            location: None,
        });
        score = score.saturating_sub(15);
        suggestions.push("降低嵌套层级，考虑提前返回或拆分函数".to_string());
    }
    
    // 检查函数复杂度（基于if/for/while/match数量）
    let complexity_indicators = [
        code.matches(" if ").count(),
        code.matches(" for ").count(),
        code.matches(" while ").count(),
        code.matches(" match ").count(),
        code.matches(" case ").count(),
    ].iter().sum::<usize>();
    
    if complexity_indicators > 10 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Complexity,
            severity: Severity::Medium,
            description: format!("圈复杂度较高 (约{})", complexity_indicators),
            location: None,
        });
        score = score.saturating_sub(10);
        suggestions.push("简化逻辑，减少分支数量".to_string());
    }
    
    // 检查魔法数字
    let has_magic_numbers = code.contains("100") || code.contains("200") || 
                           code.contains("1000") || code.contains("0.5");
    if has_magic_numbers && !code.contains("const") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Style,
            severity: Severity::Low,
            description: "存在魔法数字，建议使用常量".to_string(),
            location: None,
        });
        score = score.saturating_sub(5);
        suggestions.push("将魔法数字提取为命名常量".to_string());
    }
    
    score
}

fn check_performance(code: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    // 检查字符串连接
    if code.contains("+\"") || code.contains("+ \"") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Performance,
            severity: Severity::Info,
            description: "使用 + 进行字符串连接，性能较差".to_string(),
            location: None,
        });
        score = score.saturating_sub(3);
        suggestions.push("考虑使用 format! 或 String::push_str".to_string());
    }
    
    // 检查克隆使用
    let clone_count = code.matches(".clone()").count();
    if clone_count > 3 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Performance,
            severity: Severity::Low,
            description: format!("过多使用 clone() ({}次)", clone_count),
            location: None,
        });
        score = score.saturating_sub(5);
        suggestions.push("检查是否可以使用引用代替克隆".to_string());
    }
    
    // 检查潜在的循环性能问题
    if code.contains("for") && (code.contains(".push(") || code.contains(".insert(")) {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Performance,
            severity: Severity::Info,
            description: "循环中修改集合，考虑性能影响".to_string(),
            location: None,
        });
        score = score.saturating_sub(3);
        suggestions.push("考虑预分配容量或使用迭代器".to_string());
    }
    
    score
}

fn check_security(code: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    // 检查不安全代码块
    if code.contains("unsafe") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Security,
            severity: Severity::High,
            description: "使用了 unsafe 代码块".to_string(),
            location: None,
        });
        score = score.saturating_sub(20);
        suggestions.push("仔细review unsafe代码并添加SAFETY注释".to_string());
    }
    
    // 检查SQL注入风险
    if (code.contains("execute") || code.contains("query")) && 
       (code.contains("format!") || code.contains("+")) {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Security,
            severity: Severity::Critical,
            description: "可能存在SQL注入风险".to_string(),
            location: None,
        });
        score = score.saturating_sub(30);
        suggestions.push("使用参数化查询代替字符串拼接".to_string());
    }
    
    // 检查命令注入风险
    if code.contains("Command::new") && code.contains("format!") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Security,
            severity: Severity::High,
            description: "可能存在命令注入风险".to_string(),
            location: None,
        });
        score = score.saturating_sub(20);
        suggestions.push("验证和转义外部输入".to_string());
    }
    
    // 检查硬编码密钥/密码
    if code.contains("password") || code.contains("secret") || code.contains("api_key") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::Security,
            severity: Severity::High,
            description: "可能存在硬编码敏感信息".to_string(),
            location: None,
        });
        score = score.saturating_sub(20);
        suggestions.push("使用环境变量或密钥管理系统".to_string());
    }
    
    score
}

fn check_test_coverage(code: &str, tests: &str, flaws: &mut Vec<CodeFlaw>, suggestions: &mut Vec<String>) -> u8 {
    let mut score = 100u8;
    
    // 检查是否有测试
    if tests.trim().is_empty() || tests.len() < 50 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::TestCoverage,
            severity: Severity::High,
            description: "缺少测试代码".to_string(),
            location: None,
        });
        score = score.saturating_sub(30);
        suggestions.push("为主要功能添加单元测试".to_string());
        return score;
    }
    
    // 统计函数数量
    let function_count = code.matches("fn ").count().saturating_sub(1); // 减去可能的 fn main
    let test_count = tests.matches("#[test]").count() + 
                     tests.matches("test_").count() +
                     tests.matches("def test").count();
    
    let coverage_ratio = if function_count > 0 {
        (test_count as f32 / function_count as f32).min(1.0)
    } else {
        0.5
    };
    
    if coverage_ratio < 0.5 {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::TestCoverage,
            severity: Severity::Medium,
            description: format!("测试覆盖率较低 ({:.0}%)", coverage_ratio * 100.0),
            location: None,
        });
        score = score.saturating_sub(15);
        suggestions.push("增加测试用例覆盖更多场景".to_string());
    }
    
    // 检查边界测试
    if !tests.contains("empty") && !tests.contains("null") && !tests.contains("zero") {
        flaws.push(CodeFlaw {
            flaw_type: FlawType::TestCoverage,
            severity: Severity::Low,
            description: "缺少边界条件测试".to_string(),
            location: None,
        });
        score = score.saturating_sub(10);
        suggestions.push("添加边界条件和异常情况的测试".to_string());
    }
    
    score
}

fn calculate_max_nesting(code: &str) -> usize {
    let mut max_depth = 0;
    let mut current_depth = 0;
    
    for line in code.lines() {
        let open_braces = line.matches('{').count();
        let close_braces = line.matches('}').count();
        
        current_depth += open_braces;
        max_depth = max_depth.max(current_depth);
        current_depth = current_depth.saturating_sub(close_braces);
    }
    
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scoring_basic() {
        let code = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        let tests = r#"
            #[test]
            fn test_add() {
                assert_eq!(add(1, 2), 3);
            }
        "#;
        
        let score = evaluate_code_quality(code, tests).unwrap();
        assert!(score.total > 70, "Basic code should score > 70, got {}", score.total);
    }
}

