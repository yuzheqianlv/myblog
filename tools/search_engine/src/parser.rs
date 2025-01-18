use anyhow::{Result, Context};
use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    #[allow(dead_code)]
    #[serde(deserialize_with = "deserialize_date")]
    pub date: String,
    #[allow(dead_code)]
    pub description: Option<String>,
    pub taxonomies: Option<Taxonomies>,
}

// 自定义日期反序列化
fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value = toml::Value::deserialize(deserializer)?;
    
    match value {
        toml::Value::String(s) => Ok(s),
        toml::Value::Datetime(dt) => Ok(dt.to_string()),
        toml::Value::Integer(i) => {
            // 处理整数形式的日期 (如 20240118)
            Ok(i.to_string())
        }
        _ => {
            // 尝试解析未加引号的日期
            if let toml::Value::Table(table) = value {
                if let Some(date_str) = table.get("date") {
                    return Ok(date_str.to_string());
                }
            }
            Err(D::Error::custom("日期格式不正确"))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Taxonomies {
    pub tags: Option<Vec<String>>,
}

pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub path: String,
    pub content_hash: u64,
}

impl BlogPost {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("读取文件失败: {:?}", path.as_ref()))?;
        
        // 获取文件名作为备用标题
        let default_title = path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("未命名文章")
            .to_string();
        
        // 解析 front matter
        let (front_matter, content) = parse_front_matter(&content, &default_title)
            .with_context(|| format!("解析 front matter 失败: {:?}", path.as_ref()))?;
        
        // 获取文件名（不包含扩展名）作为 URL 路径
        let file_name = path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        
        // 计算内容哈希
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let content_hash = hasher.finish();
        
        Ok(Self {
            title: front_matter.title,
            content,
            tags: front_matter.taxonomies
                .and_then(|t| t.tags)
                .unwrap_or_default(),
            path: format!("/blog/{}", file_name),  // 不添加结尾的斜杠，让服务器处理
            content_hash,
        })
    }
}

fn parse_front_matter(content: &str, default_title: &str) -> Result<(FrontMatter, String)> {
    let mut lines = content.lines().peekable();
    let mut front_matter_str = String::new();
    let mut content = String::new();
    let mut found_front_matter = false;

    // 跳过开头的空行
    while let Some(line) = lines.peek() {
        if line.trim().is_empty() {
            lines.next();
        } else {
            break;
        }
    }

    // 检查是否以 +++ 开始
    if let Some("+++") = lines.peek().map(|s| s.trim()) {
        found_front_matter = true;
        lines.next(); // 跳过开始的 +++

        // 收集 front matter 内容
        while let Some(line) = lines.next() {
            if line.trim() == "+++" {
                break;
            }
            front_matter_str.push_str(line);
            front_matter_str.push('\n');
        }
    }

    if !found_front_matter {
        // 如果没有 front matter，使用默认值
        return Ok((FrontMatter {
            title: default_title.to_string(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            description: None,
            taxonomies: None,
        }, content));
    }

    // 解析剩余内容
    for line in lines {
        content.push_str(line);
        content.push('\n');
    }

    // 解析 front matter
    let front_matter = toml::from_str(&front_matter_str)
        .with_context(|| format!("解析 TOML 失败:\n{}", front_matter_str))?;

    Ok((front_matter, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_front_matter() {
        let content = r#"
+++
title = "测试文章"
date = "2024-01-01"
description = "这是一篇测试文章"

[taxonomies]
tags = ["rust", "test"]
+++

这是文章内容。
"#;

        let result = parse_front_matter(content);
        assert!(result.is_ok());
        
        let (front_matter, content) = result.unwrap();
        assert_eq!(front_matter.title, "测试文章");
        assert_eq!(front_matter.date, "2024-01-01");
        assert_eq!(front_matter.description, Some("这是一篇测试文章".to_string()));
        assert_eq!(
            front_matter.taxonomies.unwrap().tags.unwrap(),
            vec!["rust".to_string(), "test".to_string()]
        );
        assert!(content.contains("这是文章内容。"));
    }

    #[test]
    fn test_parse_front_matter_with_unquoted_date() {
        let content = r#"
+++
title = "测试文章"
date = 2024-01-18
description = "这是一篇测试文章"

[taxonomies]
tags = ["rust", "test"]
+++

这是文章内容。
"#;

        let result = parse_front_matter(content, "default");
        assert!(result.is_ok());
        
        let (front_matter, _) = result.unwrap();
        assert_eq!(front_matter.title, "测试文章");
        assert_eq!(front_matter.date, "2024-01-18");
    }
} 