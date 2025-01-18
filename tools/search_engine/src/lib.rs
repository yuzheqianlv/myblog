mod tokenizer;
mod parser;

use anyhow::Result;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, Document, Term};
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, TermQuery};
use tantivy::Score;

pub use crate::tokenizer::{ChineseTokenizer, VecTokenStream};
pub use crate::parser::BlogPost;

pub struct SearchEngine {
    index: Index,
    schema: Schema,
}

impl SearchEngine {
    pub fn new<P: AsRef<Path>>(index_path: P) -> Result<Self> {
        let schema = Self::create_schema();
        
        // 确保索引目录存在
        if let Some(parent) = index_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 尝试打开已存在的索引
        let index = if index_path.as_ref().exists() {
            println!("打开已存在的索引: {:?}", index_path.as_ref());
            match Index::open_in_dir(&index_path) {
                Ok(index) => {
                    // 验证索引 schema 是否匹配
                    if index.schema() != schema {
                        println!("索引结构已变更，需要重建索引");
                        fs::remove_dir_all(&index_path)?;
                        fs::create_dir_all(&index_path)?;
                        Index::create_in_dir(&index_path, schema.clone())?
                    } else {
                        index
                    }
                },
                Err(err) => {
                    println!("打开索引失败: {}，将重建索引", err);
                    fs::remove_dir_all(&index_path)?;
                    fs::create_dir_all(&index_path)?;
                    Index::create_in_dir(&index_path, schema.clone())?
                }
            }
        } else {
            println!("创建新索引: {:?}", index_path.as_ref());
            fs::create_dir_all(&index_path)?;
            Index::create_in_dir(&index_path, schema.clone())?
        };
        
        // 注册中文分词器
        let tokenizer = ChineseTokenizer {};
        index.tokenizers().register("jieba", tokenizer);
        
        Ok(Self { index, schema })
    }

    fn create_schema() -> Schema {
        let mut builder = Schema::builder();
        
        // 标题字段 - 可搜索和存储
        builder.add_text_field("title", TextOptions::default()
            .set_indexing_options(TextFieldIndexing::default()
                .set_tokenizer("jieba")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions))
            .set_stored());
        
        // 行号字段 - 仅存储
        builder.add_u64_field("line_number", STORED);
        
        // 行内容字段 - 可搜索和存储
        builder.add_text_field("line_content", TextOptions::default()
            .set_indexing_options(TextFieldIndexing::default()
                .set_tokenizer("jieba")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions))
            .set_stored());
        
        // 完整内容字段 - 可搜索
        builder.add_text_field("content", TextOptions::default()
            .set_indexing_options(TextFieldIndexing::default()
                .set_tokenizer("jieba")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions))
            .set_stored());
        
        // 标签字段 - 可搜索和存储
        builder.add_text_field("tags", TEXT | STORED);
        
        // 路径字段 - 可搜索和存储
        builder.add_text_field("path", TEXT | STORED);
        
        builder.build()
    }

    pub fn index_document(&self, writer: &mut IndexWriter, 
        title: &str, content: &str, tags: &[String], path: &str) -> Result<()> {
        
        // 创建主文档
        let main_doc = doc!(
            self.schema.get_field("title").unwrap() => title,
            self.schema.get_field("content").unwrap() => content,
            self.schema.get_field("tags").unwrap() => tags.join(","),
            self.schema.get_field("path").unwrap() => path,
            self.schema.get_field("line_number").unwrap() => 1u64
        );
        writer.add_document(main_doc)?;

        // 为每一行创建单独的索引
        for (file_line_number, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let actual_line_number = file_line_number + 1;  // 实际文件行号
                
                if trimmed.starts_with('#') {
                    let (level, heading) = if let Some((prefix, heading)) = trimmed.split_once(' ') {
                        let level = prefix.chars().filter(|&c| c == '#').count();
                        (level, heading.trim())
                    } else {
                        continue;
                    };

                    // 为标题创建索引，根据级别赋予不同的权重
                    let weight: f32 = match level {
                        1 => 3.0,
                        2 => 2.5,
                        3 => 2.0,
                        _ => 1.5,
                    };

                    // 创建标题文档，同时索引原始标题和处理后的标题
                    let line_doc = doc!(
                        self.schema.get_field("title").unwrap() => title,
                        self.schema.get_field("path").unwrap() => path,
                        self.schema.get_field("line_number").unwrap() => actual_line_number as u64,  // 使用实际行号
                        self.schema.get_field("line_content").unwrap() => trimmed,  // 原始标题（包含 #）
                        self.schema.get_field("content").unwrap() => format!("{} ({}级标题)", heading, level),  // 处理后的标题
                        self.schema.get_field("tags").unwrap() => tags.join(",")
                    );
                    
                    // 添加多个副本来提高权重
                    for _ in 0..weight.round() as i32 {
                        writer.add_document(line_doc.clone())?;
                    }
                } else if !trimmed.starts_with('>') &&    // 过滤引用
                          !trimmed.starts_with('|') &&    // 过滤表格
                          !trimmed.starts_with('-') &&    // 过滤列表
                          !trimmed.starts_with('*') &&    // 过滤列表
                          !trimmed.starts_with("```") &&  // 过滤代码块
                          !trimmed.starts_with("---")     // 过滤分隔线
                {
                    let line_doc = doc!(
                        self.schema.get_field("title").unwrap() => title,
                        self.schema.get_field("path").unwrap() => path,
                        self.schema.get_field("line_number").unwrap() => actual_line_number as u64,  // 使用实际行号
                        self.schema.get_field("line_content").unwrap() => trimmed,
                        self.schema.get_field("content").unwrap() => trimmed,
                        self.schema.get_field("tags").unwrap() => tags.join(",")
                    );
                    writer.add_document(line_doc)?;
                }
            }
        }

        Ok(())
    }

    pub fn search(&self, query_text: &str) -> Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        let mut results = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();
        
        // 获取所需字段
        let title_field = self.schema.get_field("title").unwrap();
        let path_field = self.schema.get_field("path").unwrap();
        let tags_field = self.schema.get_field("tags").unwrap();
        let line_number_field = self.schema.get_field("line_number").unwrap();
        let line_content_field = self.schema.get_field("line_content").unwrap();
        let content_field = self.schema.get_field("content").unwrap();

        // 第一步：同时搜索标题和行内容
        {
            // 创建多字段查询解析器，给标题更高的权重
            let mut query_parser = QueryParser::for_index(&self.index, vec![title_field, line_content_field]);
            // 设置标题字段的权重为2.0（默认是1.0）
            query_parser.set_field_boost(title_field, 2.0);
            
            let query = query_parser.parse_query(query_text)?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(100))?;

            // 按文件路径分组的结果
            let mut grouped_results: HashMap<String, Vec<(Score, u64, String)>> = HashMap::new();

            for (score, doc_address) in top_docs {
                let doc = searcher.doc(doc_address)?;
                let path = self.get_text(&doc, path_field);
                
                // 检查是否匹配标题
                let title = self.get_text(&doc, title_field);
                if title.to_lowercase().contains(&query_text.to_lowercase()) {
                    let line_number = 1;
                    if let Ok((context, actual_line)) = self.get_line_context(&path, line_number) {
                        grouped_results.entry(path.clone())
                            .or_default()
                            .push((score * 2.0, actual_line, context));
                    }
                }
                
                // 检查行内容
                if let Some(line_content) = doc.get_first(line_content_field).and_then(|f| f.as_text()) {
                    if line_content.to_lowercase().contains(&query_text.to_lowercase()) {
                        let line_number = doc.get_first(line_number_field)
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1);

                        if let Ok((context, actual_line)) = self.get_line_context(&path, line_number) {
                            grouped_results.entry(path)
                                .or_default()
                                .push((score, actual_line, context));
                        }
                    }
                }
            }

            // 处理分组结果
            for (path, matches) in grouped_results {
                // 按分数排序，取最高分的匹配
                if let Some((best_score, line_number, context)) = matches.iter()
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)) {
                    
                    // 使用 term query 来获取文档
                    let term = Term::from_field_text(path_field, &path);
                    let query = TermQuery::new(term, IndexRecordOption::Basic);
                    let top_docs = searcher.search(&query, &TopDocs::with_limit(1))?;
                    
                    if let Some((_score, doc_address)) = top_docs.first() {
                        let doc = searcher.doc(*doc_address)?;
                        let title = self.get_text(&doc, title_field);
                        let tags = self.get_tags(&doc, tags_field);

                        results.push(SearchResult {
                            title,
                            excerpt: context.clone(),
                            path,
                            tags,
                            score: normalize_score(*best_score),
                            anchor: format!("L{}", line_number),
                        });
                    }
                }
            }
        }

        // 第二步：如果没有找到结果，搜索完整内容
        if results.is_empty() {
            let query_parser = QueryParser::for_index(&self.index, vec![content_field]);
            let query = query_parser.parse_query(query_text)?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(20))?;

            for (_score, doc_address) in top_docs {
                let doc = searcher.doc(doc_address)?;
                let content = self.get_text(&doc, content_field);
                let path = self.get_text(&doc, path_field);

                // 避免重复结果
                if seen_paths.contains(&path) {
                    continue;
                }
                seen_paths.insert(path.clone());

                // 在内容中查找匹配的段落
                if let Some((excerpt, line_number)) = self.find_matching_excerpt(&content, query_text) {
                    results.push(SearchResult {
                        title: self.get_text(&doc, title_field),
                        excerpt,
                        path,
                        tags: self.get_tags(&doc, tags_field),
                        score: normalize_score(_score),
                        anchor: format!("L{}", line_number),
                    });
                }
            }
        }

        // 第三步：如果还是没有结果，尝试搜索标题
        if results.is_empty() {
            let query_parser = QueryParser::for_index(&self.index, vec![title_field]);
            let query = query_parser.parse_query(query_text)?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

            for (_score, doc_address) in top_docs {
                let doc = searcher.doc(doc_address)?;
                let path = self.get_text(&doc, path_field);

                // 避免重复结果
                if seen_paths.contains(&path) {
                    continue;
                }
                seen_paths.insert(path.clone());

                let content = self.get_text(&doc, content_field);
                
                // 获取文章的第一段作为摘要
                let excerpt = content.lines()
                    .find(|line| !line.trim().is_empty())
                    .unwrap_or("")
                    .to_string();

                results.push(SearchResult {
                    title: self.get_text(&doc, title_field),
                    excerpt: format!("  {}", excerpt),
                    path,
                    tags: self.get_tags(&doc, tags_field),
                    score: normalize_score(_score),
                    anchor: "L1".to_string(),
                });
            }
        }

        // 按相关度排序
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
    }
    
    fn get_text(&self, doc: &Document, field: Field) -> String {
        doc.get_first(field)
            .and_then(|v| v.as_text())
            .unwrap_or_default()
            .to_string()
    }
    
    fn get_tags(&self, doc: &Document, field: Field) -> Vec<String> {
        self.get_text(doc, field)
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }
    
    fn get_line_context(&self, path: &str, target_line_number: u64) -> Result<(String, u64)> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let current_line = target_line_number as usize - 1;
        
        if current_line >= lines.len() {
            return Ok((String::new(), target_line_number));
        }
        
        // 获取前后五行
        let start = current_line.saturating_sub(5);
        let end = (current_line + 6).min(lines.len());
        
        // 构建带行号的上下文
        let mut context_lines = Vec::new();
        for (i, &line) in lines[start..end].iter().enumerate() {
            let line_num = start + i + 1;  // 实际文件行号
            let formatted_line = if line_num == target_line_number as usize {
                format!("➤ {: >3} | {}", line_num, line)
            } else {
                format!("  {: >3} | {}", line_num, line)
            };
            context_lines.push(formatted_line);
        }
        
        Ok((context_lines.join("\n"), target_line_number))
    }

    pub fn create_writer(&self, memory_arena_size: usize) -> Result<IndexWriter> {
        Ok(self.index.writer(memory_arena_size)?)
    }

    // 添加清理索引的方法
    pub fn clear_index(&self) -> Result<()> {
        let mut writer = self.create_writer(50_000_000)?;
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }

    pub fn update_index(&self, writer: &mut IndexWriter, posts: &[BlogPost]) -> Result<()> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        let path_field = self.schema.get_field("path").unwrap();
        
        // 获取现有文档的路径
        let mut existing_docs = HashMap::new();
        let term_path = Term::from_field_text(path_field, "");
        let path_query = TermQuery::new(term_path, IndexRecordOption::Basic);
        let top_docs = searcher.search(&path_query, &TopDocs::with_limit(1000))?;

        for (_score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            if let Some(path) = doc.get_first(path_field).and_then(|f| f.as_text()) {
                existing_docs.insert(path.to_string(), doc_address);
            }
        }

        // 更新或添加新文档
        for post in posts {
            // 为每篇文章创建索引
            println!("添加新文章索引: {}", post.title);
            
            // 删除旧的文档（如果存在）
            writer.delete_term(Term::from_field_text(path_field, &post.path));
            
            // 添加主文档
            let main_doc = doc!(
                self.schema.get_field("title").unwrap() => post.title.clone(),
                self.schema.get_field("content").unwrap() => post.content.clone(),
                self.schema.get_field("tags").unwrap() => post.tags.join(","),
                self.schema.get_field("path").unwrap() => post.path.clone(),
                self.schema.get_field("line_number").unwrap() => 1u64
            );
            writer.add_document(main_doc)?;

            // 为每一行创建单独的索引
            for (file_line_number, line) in post.content.lines().enumerate() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    let actual_line_number = file_line_number + 1;  // 实际文件行号
                    
                    if trimmed.starts_with('#') {
                        let (level, heading) = if let Some((prefix, heading)) = trimmed.split_once(' ') {
                            let level = prefix.chars().filter(|&c| c == '#').count();
                            (level, heading.trim())
                        } else {
                            continue;
                        };

                        // 为标题创建索引，根据级别赋予不同的权重
                        let weight: f32 = match level {
                            1 => 3.0,
                            2 => 2.5,
                            3 => 2.0,
                            _ => 1.5,
                        };

                        // 创建标题文档，同时索引原始标题和处理后的标题
                        let line_doc = doc!(
                            self.schema.get_field("title").unwrap() => post.title.clone(),
                            self.schema.get_field("path").unwrap() => post.path.clone(),
                            self.schema.get_field("line_number").unwrap() => actual_line_number as u64,  // 使用实际行号
                            self.schema.get_field("line_content").unwrap() => trimmed,  // 原始标题（包含 #）
                            self.schema.get_field("content").unwrap() => format!("{} ({}级标题)", heading, level),  // 处理后的标题
                            self.schema.get_field("tags").unwrap() => post.tags.join(",")
                        );
                        
                        // 添加多个副本来提高权重
                        for _ in 0..weight.round() as i32 {
                            writer.add_document(line_doc.clone())?;
                        }
                    } else if !trimmed.starts_with('>') &&    // 过滤引用
                              !trimmed.starts_with('|') &&    // 过滤表格
                              !trimmed.starts_with('-') &&    // 过滤列表
                              !trimmed.starts_with('*') &&    // 过滤列表
                              !trimmed.starts_with("```") &&  // 过滤代码块
                              !trimmed.starts_with("---")     // 过滤分隔线
                    {
                        let line_doc = doc!(
                            self.schema.get_field("title").unwrap() => post.title.clone(),
                            self.schema.get_field("path").unwrap() => post.path.clone(),
                            self.schema.get_field("line_number").unwrap() => actual_line_number as u64,  // 使用实际行号
                            self.schema.get_field("line_content").unwrap() => trimmed,
                            self.schema.get_field("content").unwrap() => trimmed,
                            self.schema.get_field("tags").unwrap() => post.tags.join(",")
                        );
                        writer.add_document(line_doc)?;
                    }
                }
            }
            
            existing_docs.remove(&post.path);
        }

        // 删除不再存在的文档
        for (path, _) in existing_docs {
            writer.delete_term(Term::from_field_text(path_field, &path));
            println!("删除过期索引: {}", path);
        }

        writer.commit()?;
        Ok(())
    }

    // 添加索引管理方法
    pub fn rebuild_index(&self, writer: &mut IndexWriter) -> Result<()> {
        println!("重建索引...");
        writer.delete_all_documents()?;
        writer.commit()?;
        Ok(())
    }

    pub fn check_index_health(&self) -> Result<bool> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        
        // 检查索引是否为空
        let doc_count = searcher.num_docs();
        if doc_count == 0 {
            println!("索引为空");
            return Ok(false);
        }

        // 检查索引完整性
        let path_field = self.schema.get_field("path").unwrap();
        let term_path = Term::from_field_text(path_field, "");
        let path_query = TermQuery::new(term_path, IndexRecordOption::Basic);
        let top_docs = searcher.search(&path_query, &TopDocs::with_limit(1))?;
        
        if top_docs.is_empty() {
            println!("索引可能损坏");
            return Ok(false);
        }

        println!("索引状态正常，包含 {} 篇文章", doc_count);
        Ok(true)
    }

    // 添加辅助方法来查找匹配的段落
    fn find_matching_excerpt(&self, content: &str, query_text: &str) -> Option<(String, usize)> {
        let lines: Vec<&str> = content.lines().collect();
        let query_lower = query_text.to_lowercase();
        
        // 查找包含搜索词的行
        for (i, &line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&query_lower) {
                let actual_line_num = i + 1;
                
                // 获取前后五行
                let start = i.saturating_sub(5);
                let end = (i + 6).min(lines.len());
                
                // 构建带行号的上下文
                let mut context_lines = Vec::new();
                for (j, &context_line) in lines[start..end].iter().enumerate() {
                    let line_num = start + j + 1;
                    let formatted_line = if line_num == actual_line_num {
                        format!("➤ {: >3} | {}", line_num, context_line)
                    } else {
                        format!("  {: >3} | {}", line_num, context_line)
                    };
                    context_lines.push(formatted_line);
                }
                
                return Some((context_lines.join("\n"), actual_line_num));
            }
        }
        
        None
    }
}

// 归一化分数到 0-100 范围
fn normalize_score(score: Score) -> f32 {
    // 调整归一化系数以获得更合理的分数范围
    (score * 25.0).min(100.0)
}

#[derive(Debug, serde::Serialize)]
pub struct SearchResult {
    pub title: String,
    pub excerpt: String,
    pub path: String,
    pub tags: Vec<String>,
    pub score: f32,
    pub anchor: String,
} 