// 配置模块

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm_api_key: String,
    pub llm_api_url: String,
    pub llm_model: String,
}

impl Config {
    /// 创建默认配置
    pub fn default() -> Self {
        Self {
            llm_api_key: String::new(),
            llm_api_url: String::new(),
            llm_model: String::new(),
        }
    }

    /// 创建新的配置实例
    pub fn new(api_url: String, api_key: String, model: String) -> Self {
        Self {
            llm_api_key: api_key,
            llm_api_url: api_url,
            llm_model: model,
        }
    }
    
    /// 从配置文件加载
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))
    }
    
    /// 保存到配置文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))
    }
    
    /// 加载配置（优先从文件，文件不存在则使用默认配置）
    pub fn load() -> Result<Self, String> {
        let config_path = "config.json";
        
        if Path::new(config_path).exists() {
            Self::load_from_file(config_path)
        } else {
            Ok(Self::default())
        }
    }
    
    /// 检查配置是否完整
    pub fn is_configured(&self) -> bool {
        !self.llm_api_key.is_empty() 
            && !self.llm_api_url.is_empty() 
            && !self.llm_model.is_empty()
            && self.llm_api_key != "your-api-key-here"
    }
    
    /// 获取大模型 API Key
    pub fn api_key(&self) -> &str {
        &self.llm_api_key
    }
    
    /// 获取大模型 API URL
    pub fn api_url(&self) -> &str {
        &self.llm_api_url
    }
    
    /// 获取大模型名称
    pub fn model(&self) -> &str {
        &self.llm_model
    }
}
