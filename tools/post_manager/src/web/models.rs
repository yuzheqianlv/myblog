use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            content: String::new(),
            description: None,
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub description: String,
    pub tags: Vec<String>,
    pub categories: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub weight: Option<i32>,
    pub author: Option<String>,
    pub cover_image: Option<String>,
    pub series: Option<String>,
    pub language: Option<String>,
}

impl Default for PostMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            description: String::new(),
            tags: Vec::new(),
            categories: None,
            draft: Some(true),
            weight: None,
            author: None,
            cover_image: None,
            series: None,
            language: None,
        }
    }
}

impl PostMetadata {
    pub fn to_toml(&self) -> String {
        format!(
            r#"+++
title = "{}"
date = "{}"
description = "{}"
tags = {:?}
{}{}{}{}{}{}{}
+++"#,
            self.title,
            self.date,
            self.description,
            self.tags,
            self.categories.as_ref().map_or(String::new(), |c| format!("categories = {:?}\n", c)),
            self.draft.map_or(String::new(), |d| format!("draft = {}\n", d)),
            self.weight.map_or(String::new(), |w| format!("weight = {}\n", w)),
            self.author.as_ref().map_or(String::new(), |a| format!("author = \"{}\"\n", a)),
            self.cover_image.as_ref().map_or(String::new(), |i| format!("cover_image = \"{}\"\n", i)),
            self.series.as_ref().map_or(String::new(), |s| format!("series = \"{}\"\n", s)),
            self.language.as_ref().map_or(String::new(), |l| format!("language = \"{}\"\n", l)),
        )
    }
} 