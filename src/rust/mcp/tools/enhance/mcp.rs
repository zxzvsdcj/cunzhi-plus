// 提示词增强MCP工具实现
use anyhow::Result;
use rmcp::{ErrorData as McpError, model::*};
use chrono::Utc;
use std::time::Instant;

use super::types::*;
use super::vision::{extract_image_info, integrate_image_context};
use super::pipeline::*;
use crate::mcp::utils::generate_request_id;

/// 提示词增强工具
#[derive(Clone)]
pub struct EnhanceTool;

impl EnhanceTool {
    /// 增强用户提示词
    /// 
    /// 支持:
    /// - 自动提示词润色
    /// - 多模态图片理解
    /// - 四阶增强管线
    /// - 寸止评分闭环
    pub async fn enhance(
        request: EnhanceRequest,
    ) -> Result<CallToolResult, McpError> {
        let start_time = Instant::now();
        let request_id = generate_request_id();
        let mut enabled_features = vec!["基础增强".to_string()];
        
        // 1. 基础提示词增强
        let enhanced_prompt = enhance_prompt_basic(&request.prompt);
        
        // 2. 处理图片(如果有)
        let image_descriptions = if !request.images.is_empty() {
            enabled_features.push("多模态处理".to_string());
            extract_image_info(&request.images)
                .await
                .map_err(|e| McpError::internal_error(format!("图片处理失败: {}", e), None))?
        } else {
            Vec::new()
        };
        
        let image_context = image_descriptions.join("\n");
        let full_prompt = integrate_image_context(&enhanced_prompt, &image_descriptions);
        
        // 3. 四阶增强管线(可选)
        let (analysis, task_spec, code_result) = if request.enable_pipeline {
            enabled_features.push("四阶管线".to_string());
            
            // 意图分类
            let _ = classify_intent(&full_prompt).await
                .map_err(|e| McpError::internal_error(format!("意图分类失败: {}", e), None))?;
            
            // 需求分析
            let analysis = analyze_requirements(&full_prompt, &image_context).await
                .map_err(|e| McpError::internal_error(format!("需求分析失败: {}", e), None))?;
            
            // 任务单生成
            let task_spec = generate_task_spec(&analysis).await
                .map_err(|e| McpError::internal_error(format!("任务单生成失败: {}", e), None))?;
            
            // 代码生成
            let mut code_result = generate_code_with_tests(&task_spec).await
                .map_err(|e| McpError::internal_error(format!("代码生成失败: {}", e), None))?;
            
            // 评分闭环(可选)
            if request.enable_scoring {
                enabled_features.push("评分闭环".to_string());
                code_result = scoring_loop(code_result, request.target_score).await
                    .map_err(|e| McpError::internal_error(format!("评分闭环失败: {}", e), None))?;
            }
            
            (Some(analysis), Some(task_spec), Some(code_result))
        } else {
            (None, None, None)
        };
        
        // 4. 构建结果
        let duration_ms = start_time.elapsed().as_millis() as u64;
        
        let result = EnhanceResult {
            enhanced_prompt: full_prompt,
            image_descriptions,
            analysis,
            task_spec,
            code_result,
            metadata: EnhanceMetadata {
                request_id,
                timestamp: Utc::now().to_rfc3339(),
                duration_ms,
                enabled_features,
            },
        };
        
        // 5. 格式化输出
        let response_text = format_enhance_result(&result);
        
        Ok(CallToolResult::success(vec![Content::text(response_text)]))
    }
}

/// 基础提示词增强
fn enhance_prompt_basic(prompt: &str) -> String {
    // 检查是否以 /e 开头
    let cleaned_prompt = if prompt.starts_with("/e ") {
        &prompt[3..]
    } else {
        prompt
    };
    
    // 应用增强模板
    format!(
        "请按以下要求优化和执行任务:\n\n\
        **原始需求**: {}\n\n\
        **优化要求**:\n\
        1. 明确性 - 消除歧义,使需求更具体\n\
        2. 完整性 - 补充隐含需求和边界条件\n\
        3. 可执行性 - 提供清晰的实现路径\n\
        4. 质量标准 - 遵循DRY/KISS/SOLID原则\n\n\
        请深入理解需求的字面、意图、场景和补全四个层次,然后给出最佳方案。",
        cleaned_prompt
    )
}

/// 格式化增强结果
fn format_enhance_result(result: &EnhanceResult) -> String {
    let mut output = String::new();
    
    // 1. 增强后的提示词
    output.push_str("## 🚀 增强后的提示词\n\n");
    output.push_str(&result.enhanced_prompt);
    output.push_str("\n\n");
    
    // 2. 图片描述
    if !result.image_descriptions.is_empty() {
        output.push_str("## 🖼️ 图片理解\n\n");
        for desc in &result.image_descriptions {
            output.push_str(&format!("- {}\n", desc));
        }
        output.push_str("\n");
    }
    
    // 3. 需求分析
    if let Some(analysis) = &result.analysis {
        output.push_str("## 📊 需求分析\n\n");
        output.push_str(&format!("**字面理解**: {}\n\n", analysis.literal));
        output.push_str(&format!("**意图推理**: {}\n\n", analysis.intent));
        output.push_str(&format!("**场景还原**: {}\n\n", analysis.context));
        
        if !analysis.completion.is_empty() {
            output.push_str("**需求补全**:\n");
            for item in &analysis.completion {
                output.push_str(&format!("- {}\n", item));
            }
            output.push_str("\n");
        }
        
        if !analysis.questions.is_empty() {
            output.push_str("**待澄清问题**:\n");
            for q in &analysis.questions {
                output.push_str(&format!("- {}\n", q));
            }
            output.push_str("\n");
        }
    }
    
    // 4. 任务单
    if let Some(task) = &result.task_spec {
        output.push_str("## 📋 任务单\n\n");
        output.push_str(&format!("- **场景**: {}\n", task.scene));
        output.push_str(&format!("- **输入**: {}\n", task.input));
        output.push_str(&format!("- **输出**: {}\n", task.output));
        output.push_str(&format!("- **性能**: {}\n", task.performance));
        output.push_str(&format!("- **技术栈**: {}\n\n", task.tech_stack));
        
        output.push_str("**验收标准**:\n");
        for (idx, ac) in task.acceptance_criteria.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", idx + 1, ac));
        }
        output.push_str("\n");
    }
    
    // 5. 代码结果
    if let Some(code) = &result.code_result {
        output.push_str(&format!("## 💻 代码生成 (得分: {}/100)\n\n", code.score));
        
        if !code.flaws.is_empty() {
            output.push_str("**发现的问题**:\n");
            for flaw in &code.flaws {
                output.push_str(&format!("- {}\n", flaw));
            }
            output.push_str("\n");
        }
        
        output.push_str("**代码**:\n```rust\n");
        output.push_str(&code.code);
        output.push_str("\n```\n\n");
        
        output.push_str("**测试**:\n```rust\n");
        output.push_str(&code.tests);
        output.push_str("\n```\n\n");
    }
    
    // 6. 元数据
    output.push_str("---\n\n");
    output.push_str(&format!(
        "*请求ID*: {} | *耗时*: {}ms | *功能*: {}",
        result.metadata.request_id,
        result.metadata.duration_ms,
        result.metadata.enabled_features.join(", ")
    ));
    
    output
}

