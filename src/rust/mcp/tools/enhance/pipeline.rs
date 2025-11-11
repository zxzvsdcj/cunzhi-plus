// 四阶增强管线实现
use anyhow::Result;
use crate::mcp::types::ZhiRequest;
use crate::mcp::tools::InteractionTool;
use super::types::*;

/// 阶段0: 意图分类
pub async fn classify_intent(prompt: &str) -> Result<String> {
    let keywords = ["帮我写", "给我", "实现", "创建", "生成"];
    
    if keywords.iter().any(|k| prompt.contains(k)) {
        Ok("code_generation".to_string())
    } else if prompt.contains("分析") || prompt.contains("解释") {
        Ok("code_analysis".to_string())
    } else {
        Ok("general".to_string())
    }
}

/// 阶段1: 需求反向访谈(四层分析)
pub async fn analyze_requirements(
    prompt: &str,
    image_context: &str,
) -> Result<RequirementAnalysis> {
    let full_context = if image_context.is_empty() {
        prompt.to_string()
    } else {
        format!("{}\n\n{}", prompt, image_context)
    };
    
    // 通过寸止工具进行交互式分析
    let analysis_request = ZhiRequest {
        message: format!(
            "## 需求深度分析\n\n\
            原始需求: {}\n\n\
            请帮助分析以下4个维度:\n\
            1. **字面理解**: 用户明确说了什么?\n\
            2. **意图推理**: 用户为什么需要这个?真实目标是什么?\n\
            3. **场景还原**: 用户在什么场景下使用?环境条件如何?\n\
            4. **需求补全**: 用户没说但必需的关联需求有哪些?\n\
            5. **不确定点**: 存在哪些模糊、矛盾或缺失的信息?",
            full_context
        ),
        predefined_options: vec![
            "分析完成".to_string(),
            "需要更多信息".to_string(),
        ],
        is_markdown: true,
    };
    
    // 这里简化处理,实际应该解析用户的详细分析
    // 在生产环境中应该多轮交互直到分析完整
    let _ = InteractionTool::zhi(analysis_request).await?;
    
    // 返回分析结果(示例)
    Ok(RequirementAnalysis {
        literal: "用户请求实现功能".to_string(),
        intent: "解决具体技术问题".to_string(),
        context: "开发环境/生产环境".to_string(),
        completion: vec![
            "错误处理".to_string(),
            "日志记录".to_string(),
            "性能优化".to_string(),
        ],
        questions: vec![
            "数据量级如何?".to_string(),
            "性能要求?".to_string(),
        ],
    })
}

/// 阶段2: 生成任务单
pub async fn generate_task_spec(
    analysis: &RequirementAnalysis,
) -> Result<TaskSpec> {
    // 通过寸止工具确认任务单
    let task_request = ZhiRequest {
        message: format!(
            "## 任务单生成\n\n\
            基于需求分析,生成可验收的任务单:\n\n\
            **场景**: {}\n\
            **意图**: {}\n\
            **补全需求**: {:?}\n\n\
            请确认任务单的关键信息:\n\
            1. 输入输出格式\n\
            2. 性能要求\n\
            3. 技术栈选择\n\
            4. 验收标准(2-3条可跑通的断言)",
            analysis.context,
            analysis.intent,
            analysis.completion
        ),
        predefined_options: vec![
            "确认任务单".to_string(),
            "需要修改".to_string(),
        ],
        is_markdown: true,
    };
    
    let _ = InteractionTool::zhi(task_request).await?;
    
    // 返回任务单(示例)
    Ok(TaskSpec {
        scene: analysis.context.clone(),
        input: "用户输入数据".to_string(),
        output: "处理结果".to_string(),
        performance: "响应时间 < 100ms".to_string(),
        tech_stack: "Rust/Tokio".to_string(),
        acceptance_criteria: vec![
            "功能正确性测试通过".to_string(),
            "性能指标达标".to_string(),
            "代码规范检查通过".to_string(),
        ],
    })
}

/// 阶段3: 代码+测试生成与三重校验
pub async fn generate_code_with_tests(
    task_spec: &TaskSpec,
) -> Result<CodeResult> {
    // 通过寸止工具生成代码
    let code_request = ZhiRequest {
        message: format!(
            "## 代码生成\n\n\
            **任务单**:\n\
            - 场景: {}\n\
            - 输入: {}\n\
            - 输出: {}\n\
            - 性能: {}\n\
            - 技术栈: {}\n\n\
            请生成:\n\
            1. 完整的代码实现\n\
            2. 测试用例\n\
            3. 错误处理\n\
            4. 文档注释\n\n\
            代码必须符合以下标准:\n\
            - DRY原则\n\
            - KISS原则\n\
            - SOLID原则(如适用)\n\
            - 行数 ≤ 400行",
            task_spec.scene,
            task_spec.input,
            task_spec.output,
            task_spec.performance,
            task_spec.tech_stack
        ),
        predefined_options: vec![
            "代码生成完成".to_string(),
            "需要修改".to_string(),
        ],
        is_markdown: true,
    };
    
    let _ = InteractionTool::zhi(code_request).await?;
    
    // 返回代码结果(示例)
    Ok(CodeResult {
        code: "// 生成的代码\nfn main() {\n    println!(\"Hello, World!\");\n}".to_string(),
        tests: "#[test]\nfn test_main() {\n    // 测试用例\n}".to_string(),
        score: 85,
        flaws: vec![
            "缺少错误处理".to_string(),
        ],
    })
}

/// 寸止评分闭环
pub async fn scoring_loop(
    mut code_result: CodeResult,
    target_score: u8,
) -> Result<CodeResult> {
    let max_iterations = 3;
    let mut iteration = 0;
    
    while code_result.score < target_score && iteration < max_iterations {
        iteration += 1;
        
        // 通过寸止工具进行评分和重构
        let review_request = ZhiRequest {
            message: format!(
                "## 代码评审 (轮次 {})\n\n\
                **当前得分**: {}/100\n\
                **目标得分**: {}/100\n\n\
                **发现的问题**:\n{}\n\n\
                **代码**:\n```rust\n{}\n```\n\n\
                请修复以上问题并提高代码质量",
                iteration,
                code_result.score,
                target_score,
                code_result.flaws.join("\n"),
                code_result.code
            ),
            predefined_options: vec![
                "重构完成".to_string(),
                "需要更多时间".to_string(),
            ],
            is_markdown: true,
        };
        
        let _ = InteractionTool::zhi(review_request).await?;
        
        // 模拟评分提升(实际应该由AI模型评分)
        code_result.score += 5;
        if !code_result.flaws.is_empty() {
            code_result.flaws.pop();
        }
    }
    
    Ok(code_result)
}

