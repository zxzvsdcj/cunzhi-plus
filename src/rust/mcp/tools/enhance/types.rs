use serde::{Deserialize, Serialize};

/// 增强请求
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EnhanceRequest {
    #[schemars(description = "用户原始输入")]
    pub prompt: String,
    
    #[schemars(description = "图片附件(base64编码,可选)")]
    #[serde(default)]
    pub images: Vec<ImageInput>,
    
    #[schemars(description = "是否启用四阶管线,默认true")]
    #[serde(default = "default_enable_pipeline")]
    pub enable_pipeline: bool,
    
    #[schemars(description = "是否启用寸止评分闭环,默认true")]
    #[serde(default = "default_enable_scoring")]
    pub enable_scoring: bool,
    
    #[schemars(description = "目标质量分数(0-100),默认90")]
    #[serde(default = "default_target_score")]
    pub target_score: u8,
}

fn default_enable_pipeline() -> bool {
    true
}

fn default_enable_scoring() -> bool {
    true
}

fn default_target_score() -> u8 {
    90
}

/// 图片输入
#[derive(Debug, Clone, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ImageInput {
    #[schemars(description = "base64编码的图片数据")]
    pub data: String,
    
    #[schemars(description = "MIME类型,如image/png")]
    pub media_type: String,
    
    #[schemars(description = "文件名(可选)")]
    pub filename: Option<String>,
}

/// 四层需求分析结果
#[derive(Debug, Serialize, Deserialize)]
pub struct RequirementAnalysis {
    /// 字面理解层
    pub literal: String,
    
    /// 意图推理层
    pub intent: String,
    
    /// 场景还原层
    pub context: String,
    
    /// 需求补全层
    pub completion: Vec<String>,
    
    /// 疑问点列表
    pub questions: Vec<String>,
}

/// 任务单
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSpec {
    /// 业务场景
    pub scene: String,
    
    /// 输入格式
    pub input: String,
    
    /// 输出格式
    pub output: String,
    
    /// 性能要求
    pub performance: String,
    
    /// 技术栈
    pub tech_stack: String,
    
    /// 验收标准
    pub acceptance_criteria: Vec<String>,
}

/// 代码生成结果
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResult {
    /// 生成的代码
    pub code: String,
    
    /// 测试用例
    pub tests: String,
    
    /// 质量分数(0-100)
    pub score: u8,
    
    /// 缺陷列表
    pub flaws: Vec<String>,
}

/// 增强结果
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhanceResult {
    /// 增强后的提示词
    pub enhanced_prompt: String,
    
    /// 图片描述(如果有)
    pub image_descriptions: Vec<String>,
    
    /// 需求分析(如果启用四阶管线)
    pub analysis: Option<RequirementAnalysis>,
    
    /// 任务单(如果启用四阶管线)
    pub task_spec: Option<TaskSpec>,
    
    /// 代码结果(如果启用四阶管线)
    pub code_result: Option<CodeResult>,
    
    /// 元数据
    pub metadata: EnhanceMetadata,
}

/// 增强元数据
#[derive(Debug, Serialize, Deserialize)]
pub struct EnhanceMetadata {
    /// 请求ID
    pub request_id: String,
    
    /// 时间戳
    pub timestamp: String,
    
    /// 处理耗时(毫秒)
    pub duration_ms: u64,
    
    /// 启用的功能
    pub enabled_features: Vec<String>,
}

