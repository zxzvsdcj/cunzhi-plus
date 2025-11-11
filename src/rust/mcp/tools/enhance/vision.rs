// 多模态视觉处理模块
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;

use super::types::ImageInput;

#[derive(Debug, Clone)]
pub enum VisionProvider {
    OpenAI,
    Claude,
    Mock,
}

impl VisionProvider {
    pub fn from_env() -> Self {
        match std::env::var("VISION_PROVIDER").as_deref() {
            Ok("openai") => VisionProvider::OpenAI,
            Ok("claude") => VisionProvider::Claude,
            _ => VisionProvider::Mock,
        }
    }
}

/// 提取图片语义信息
/// 
/// 支持多种视觉API提供商:
/// - OpenAI GPT-4 Vision (VISION_PROVIDER=openai)
/// - Claude 3 Vision (VISION_PROVIDER=claude)
/// - Mock模式（默认，用于测试）
pub async fn extract_image_info(images: &[ImageInput]) -> Result<Vec<String>> {
    let provider = VisionProvider::from_env();
    
    match provider {
        VisionProvider::OpenAI => extract_with_openai(images).await,
        VisionProvider::Claude => extract_with_claude(images).await,
        VisionProvider::Mock => extract_mock(images).await,
    }
}

async fn extract_with_openai(images: &[ImageInput]) -> Result<Vec<String>> {
    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| anyhow!("未设置 OPENAI_API_KEY 环境变量"))?;
    
    let mut descriptions = Vec::new();
    
    for image in images {
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "gpt-4-vision-preview",
                "messages": [{
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "请详细描述这张图片的内容，包括主要元素、布局、颜色等信息。如果是UI设计图或技术图表，请特别注意其中的文字、结构和功能说明。"
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": format!("data:{};base64,{}", 
                                    image.media_type, 
                                    image.data
                                )
                            }
                        }
                    ]
                }],
                "max_tokens": 500
            }))
            .send()
            .await
            .map_err(|e| anyhow!("OpenAI API 请求失败: {}", e))?;
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| anyhow!("解析 OpenAI 响应失败: {}", e))?;
        
        let description = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("无法识别图片内容")
            .to_string();
        
        descriptions.push(description);
    }
    
    Ok(descriptions)
}

async fn extract_with_claude(images: &[ImageInput]) -> Result<Vec<String>> {
    let client = Client::new();
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow!("未设置 ANTHROPIC_API_KEY 环境变量"))?;
    
    let mut descriptions = Vec::new();
    
    for image in images {
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key.clone())
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-3-opus-20240229",
                "max_tokens": 1024,
                "messages": [{
                    "role": "user",
                    "content": [
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": &image.media_type,
                                "data": &image.data
                            }
                        },
                        {
                            "type": "text",
                            "text": "请详细描述这张图片的内容，包括主要元素、布局、颜色等信息。如果是UI设计图或技术图表，请特别注意其中的文字、结构和功能说明。"
                        }
                    ]
                }]
            }))
            .send()
            .await
            .map_err(|e| anyhow!("Claude API 请求失败: {}", e))?;
        
        let result: serde_json::Value = response.json().await
            .map_err(|e| anyhow!("解析 Claude 响应失败: {}", e))?;
        
        let description = result["content"][0]["text"]
            .as_str()
            .unwrap_or("无法识别图片内容")
            .to_string();
        
        descriptions.push(description);
    }
    
    Ok(descriptions)
}

async fn extract_mock(images: &[ImageInput]) -> Result<Vec<String>> {
    let mut descriptions = Vec::new();
    
    for (idx, image) in images.iter().enumerate() {
        let desc = format!(
            "图片 {} ({}): {} KB数据 [Mock模式 - 如需真实视觉识别，请设置 VISION_PROVIDER=openai 或 claude]",
            idx + 1,
            image.filename.as_deref().unwrap_or("未命名"),
            image.data.len() / 1024
        );
        descriptions.push(desc);
    }
    
    Ok(descriptions)
}

/// 将图片描述整合到提示词中
pub fn integrate_image_context(original_prompt: &str, image_descriptions: &[String]) -> String {
    if image_descriptions.is_empty() {
        return original_prompt.to_string();
    }
    
    let images_context = image_descriptions.join("\n");
    format!(
        "{}\n\n**附加图片上下文:**\n{}",
        original_prompt,
        images_context
    )
}
