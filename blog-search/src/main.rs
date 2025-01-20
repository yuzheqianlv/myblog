use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    path: String,
    title: String,
    excerpt: String,
    tags: Vec<String>,
}

async fn search(Query(params): Query<SearchQuery>) -> Result<Json<Vec<SearchResult>>, StatusCode> {
    let query = params.q;
    
    // 使用 ripgrep 搜索内容
    let output = Command::new("rg")
        .args([
            "--json",           // 输出 JSON 格式
            "--type", "md",     // 只搜索 Markdown 文件
            "--max-depth", "3", // 限制搜索深度
            "-i",              // 忽略大小写
            &query,
            "content"          // 搜索 content 目录
        ])
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !output.status.success() {
        return Ok(Json(vec![]));
    }

    // 解析搜索结果
    let results = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let value: serde_json::Value = serde_json::from_str(line).ok()?;
            if let Some(data) = value.get("data") {
                let path = data.get("path")?.as_str()?.to_string();
                let text = data.get("lines")?.get("text")?.as_str()?.to_string();
                
                // 提取文章标题和标签
                let (title, tags) = extract_metadata(&path)?;
                
                Some(SearchResult {
                    path,
                    title,
                    excerpt: text,
                    tags,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(results))
}

// 从 Markdown 文件中提取元数据
fn extract_metadata(path: &str) -> Option<(String, Vec<String>)> {
    let content = std::fs::read_to_string(path).ok()?;
    let mut title = String::new();
    let mut tags = Vec::new();
    
    let mut in_frontmatter = false;
    for line in content.lines() {
        if line == "+++" {
            in_frontmatter = !in_frontmatter;
            continue;
        }
        
        if in_frontmatter {
            if line.starts_with("title = ") {
                title = line.trim_start_matches("title = ")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("tags = ") {
                tags = line.trim_start_matches("tags = [")
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim_matches('"').to_string())
                    .collect();
            }
        }
    }
    
    Some((title, tags))
}

// 修改主函数为 Shuttle 格式
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 创建路由
    let app = Router::new()
        .route("/api/search", get(search))
        .layer(CorsLayer::permissive());
    
    Ok(app.into())
} 