use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

use search_engine::{SearchEngine, BlogPost};
use crate::server::SearchServer;

mod server;

fn get_content_dir() -> PathBuf {
    let current_dir = std::env::current_dir().expect("无法获取当前目录");
    current_dir
        .ancestors()
        .find(|p| p.join("content").join("blog").is_dir())
        .map(|p| p.join("content").join("blog"))
        .unwrap_or_else(|| {
            // 如果找不到，尝试相对路径
            Path::new("../../content/blog").to_path_buf()
        })
}

fn should_index_file(path: &Path) -> bool {
    // 跳过特殊文件
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.starts_with('_') || file_name.starts_with('.') {
            return false;
        }
    }
    
    // 检查扩展名
    path.extension()
        .map_or(false, |ext| ext == "md")
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化搜索引擎
    let search_engine = SearchEngine::new("./data/search_index")?;
    let mut writer = search_engine.create_writer(50_000_000)?;

    // 检查索引健康状态
    if !search_engine.check_index_health()? {
        println!("检测到索引异常，将重建索引");
        search_engine.rebuild_index(&mut writer)?;
    }

    // 遍历博客文章目录
    let content_path = get_content_dir();
    println!("开始索引文章目录: {:?}", content_path);
    
    if !content_path.exists() {
        println!("警告: 文章目录不存在: {:?}", content_path);
        println!("当前工作目录: {:?}", std::env::current_dir()?);
        return Err(anyhow::anyhow!("文章目录不存在"));
    }
    
    // 收集所有文章
    let mut posts = Vec::new();
    for entry in WalkDir::new(&content_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_index_file(e.path()))
    {
        println!("处理文件: {:?}", entry.path());
        match BlogPost::from_file(entry.path()) {
            Ok(post) => {
                posts.push(post);
            }
            Err(err) => {
                println!("警告: 解析文章失败 {:?}: {}", entry.path(), err);
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    println!("文件内容:\n{}", content);
                }
            }
        }
    }

    // 更新索引
    search_engine.update_index(&mut writer, &posts)?;
    println!("索引更新完成，共处理 {} 篇文章", posts.len());

    // 启动搜索服务器
    println!("启动搜索服务器...");
    let server = SearchServer::new(search_engine);
    server.run().await;

    Ok(())
} 