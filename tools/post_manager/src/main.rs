use clap::{Parser, Subcommand};
use anyhow::Result;
use chrono::Local;
use colored::*;
use std::path::{Path, PathBuf};
use std::env;
use std::fmt;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 创建新文章
    New {
        /// 文章标题
        title: String,
        /// 文章描述
        #[arg(short, long)]
        description: Option<String>,
        /// 文章标签
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// 检查文章格式
    Check {
        /// 文章路径，不指定则检查所有文章
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// 修复文章格式问题
    Fix {
        /// 文章路径，不指定则修复所有文章
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[derive(Debug)]
enum PostError {
    MissingFrontmatter,
    InvalidTitleFormat,
    MissingDate,
    MissingDescription,
    InvalidTagsFormat,
    EmptyContent,
    InvalidDateFormat,
}

// 实现 Display trait
impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

// 实现 Error trait
impl Error for PostError {}

impl PostError {
    fn message(&self) -> &str {
        match self {
            Self::MissingFrontmatter => "缺少前置元数据",
            Self::InvalidTitleFormat => "标题格式无效",
            Self::MissingDate => "缺少日期",
            Self::MissingDescription => "缺少描述",
            Self::InvalidTagsFormat => "标签格式无效",
            Self::EmptyContent => "文章内容为空",
            Self::InvalidDateFormat => "日期格式无效",
        }
    }

    fn fix_suggestion(&self) -> &str {
        match self {
            Self::MissingFrontmatter => "添加前置元数据:\n+++\ntitle = \"标题\"\ndate = YYYY-MM-DD\ndescription = \"描述\"\n+++",
            Self::InvalidTitleFormat => "修改为: title = \"文章标题\"",
            Self::MissingDate => "添加: date = YYYY-MM-DD",
            Self::MissingDescription => "添加: description = \"文章描述\"",
            Self::InvalidTagsFormat => "修改为: [taxonomies]\ntags = [\"标签1\", \"标签2\"]",
            Self::EmptyContent => "在 +++ 后添加文章内容",
            Self::InvalidDateFormat => "使用正确的日期格式: YYYY-MM-DD",
        }
    }

    fn can_auto_fix(&self) -> bool {
        matches!(self, 
            Self::MissingDate | 
            Self::MissingDescription | 
            Self::InvalidDateFormat |
            Self::EmptyContent
        )
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title, description, tags } => {
            create_new_post(title, description.as_deref(), tags.as_ref())?;
        }
        Commands::Check { path } => {
            check_posts(path.as_ref().map(|p| p.as_path()))?;
        }
        Commands::Fix { path } => {
            fix_posts(path.as_ref().map(|p| p.as_path()))?;
        }
    }

    Ok(())
}

fn get_project_root() -> PathBuf {
    env::current_dir()
        .expect("Failed to get current directory")
        .parent()
        .expect("Failed to get parent directory")
        .parent()
        .expect("Failed to get project root")
        .to_path_buf()
}

fn create_new_post(title: &str, description: Option<&str>, tags: Option<&Vec<String>>) -> Result<()> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = title.to_lowercase()
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
    
    let project_root = get_project_root();
    let path = project_root
        .join("content")
        .join("blog")
        .join(format!("{}.md", filename));

    let content = format!(
        r#"+++
title = "{}"
date = {}
description = "{}"
{}
+++

在这里写文章内容...
"#,
        title,
        date,
        description.unwrap_or(""),
        tags.map(|t| format!("\n[taxonomies]\ntags = {:?}", t)).unwrap_or_default()
    );

    std::fs::write(&path, content)?;
    
    println!("{} Created new post: {}", "✓".green(), path.display());
    Ok(())
}

fn check_posts(path: Option<&Path>) -> Result<()> {
    let project_root = get_project_root();
    let posts_dir = project_root.join("content").join("blog");
    
    let paths: Vec<_> = if let Some(p) = path {
        vec![p.to_path_buf()]
    } else {
        std::fs::read_dir(&posts_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |ext| ext == "md"))
            .collect()
    };

    // 对路径进行排序和去重
    let mut unique_paths = paths;
    unique_paths.sort();
    unique_paths.dedup();

    for path in unique_paths {
        check_post(&path)?;
    }
    Ok(())
}

fn check_post(path: &Path) -> Result<()> {
    // 跳过 _index.md 的某些检查
    let is_index = path.file_name()
        .and_then(|n| n.to_str())
        .map_or(false, |n| n == "_index.md");

    let content = std::fs::read_to_string(path)?;
    let mut issues = vec![
        check_frontmatter(&content),
        check_title_format(&content),
    ];

    // 只对非索引文件进行完整检查
    if !is_index {
        issues.extend(vec![
            check_date_format(&content),
            check_description(&content),
            check_tags_format(&content),
            check_content_length(&content),
        ]);
    }

    let errors: Vec<_> = issues.into_iter()
        .filter_map(|r| r.err())
        .collect();

    if !errors.is_empty() {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
            
        println!("\n{} {} 存在以下问题:", "✗".red(), file_name);
        for err in &errors {
            let post_err = err.downcast_ref::<PostError>()
                .expect("Should be PostError");
            
            println!("\n  {} {}", "•".yellow(), post_err.message());
            println!("    {} {}", "💡".blue(), post_err.fix_suggestion());
            
            if post_err.can_auto_fix() {
                println!("    {} 可以使用 `cargo run -- fix -p \"{}\"` 自动修复", 
                    "🔧".green(), 
                    path.display()
                );
            }
        }
        println!();
    } else {
        println!("{} {} 格式正确", "✓".green(), 
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
        );
    }
    Ok(())
}

fn fix_posts(path: Option<&Path>) -> Result<()> {
    let project_root = get_project_root();
    let posts_dir = project_root.join("content").join("blog");
    
    let paths = if let Some(p) = path {
        vec![p.to_path_buf()]
    } else {
        std::fs::read_dir(&posts_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |ext| ext == "md"))
            .collect()
    };

    for path in paths {
        fix_post(&path)?;
    }
    Ok(())
}

fn fix_post(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)?;
    let mut fixed_content = content.clone();

    // 修复日期
    if let Err(_) = check_date_format(&content) {
        let date = Local::now().format("%Y-%m-%d").to_string();
        fixed_content = fixed_content.replace(
            "+++",
            &format!("+++\ndate = {}", date)
        );
    }

    // 修复描述
    if let Err(_) = check_description(&content) {
        fixed_content = fixed_content.replace(
            "+++",
            "+++\ndescription = \"待补充\""
        );
    }

    // 修复空内容
    if let Err(_) = check_content_length(&content) {
        fixed_content.push_str("\n\n这里是文章内容...\n");
    }

    if fixed_content != content {
        std::fs::write(path, fixed_content)?;
        println!("{} Fixed {}", "✓".green(), path.display());
    } else {
        println!("{} No fixes needed for {}", "ℹ".blue(), path.display());
    }

    Ok(())
}

// 检查函数实现...
fn check_frontmatter(content: &str) -> Result<()> {
    if !content.starts_with("+++") {
        return Err(PostError::MissingFrontmatter.into());
    }
    Ok(())
}

fn check_title_format(content: &str) -> Result<()> {
    if !content.contains("title = \"") {
        return Err(PostError::InvalidTitleFormat.into());
    }
    Ok(())
}

fn check_date_format(content: &str) -> Result<()> {
    if !content.contains("date = ") {
        return Err(PostError::MissingDate.into());
    }
    Ok(())
}

fn check_description(content: &str) -> Result<()> {
    if !content.contains("description = \"") {
        return Err(PostError::MissingDescription.into());
    }
    Ok(())
}

fn check_tags_format(content: &str) -> Result<()> {
    if content.contains("[taxonomies]") && !content.contains("tags = [") {
        return Err(PostError::InvalidTagsFormat.into());
    }
    Ok(())
}

fn check_content_length(content: &str) -> Result<()> {
    if let Some(start) = content.find("+++\n\n") {
        let actual_content = &content[start + 5..];
        if actual_content.trim().is_empty() {
            return Err(PostError::EmptyContent.into());
        }
    }
    Ok(())
} 