use chrono::Local;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let static_dir = "../../static/images";
    let gallery_dir = "../../content/gallery";
    
    // 确保目录存在
    create_dir_all(gallery_dir).expect("Failed to create gallery directory");
    
    // 获取当前日期
    let date = Local::now().format("%Y-%m-%d").to_string();
    
    // 遍历 static/images 目录
    for entry in WalkDir::new(static_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_image_file(e.path()))
    {
        let image_path = entry.path();
        let file_name = image_path.file_name().unwrap().to_str().unwrap();
        let md_file_name = format!(
            "{}/{}",
            gallery_dir,
            image_path.file_stem().unwrap().to_str().unwrap()
        );
        
        // 创建 Markdown 文件内容
        let content = format!(
            r#"+++
title = "{}"
date = {}
description = "图片描述"

[taxonomies]
tags = ["图片"]

[extra]
image = "/images/{}"
+++

这是一张图片的描述...
"#,
            file_name,
            date,
            file_name
        );
        
        // 写入 Markdown 文件
        let md_path = format!("{}.md", md_file_name);
        fs::write(&md_path, content).expect("Failed to write markdown file");
        
        println!("Created: {}", md_path);
    }
}

fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap().to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "webp"
        )
    } else {
        false
    }
} 