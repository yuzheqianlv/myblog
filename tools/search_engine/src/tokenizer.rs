use std::sync::Arc;
use tantivy::tokenizer::*;
use jieba_rs::Jieba;
use once_cell::sync::Lazy;

pub(crate) static JIEBA: Lazy<Arc<Jieba>> = Lazy::new(|| {
    let mut jieba = Jieba::new();
    
    // 添加技术词汇
    let tech_words = [
        ("Rust", None, None), 
        ("Cargo", None, None),
        ("crate", None, None),
        ("async", None, None),
        ("await", None, None),
        ("tokio", None, None),
        ("axum", None, None),
        ("tantivy", None, None),
        ("PostgreSQL", None, None),
        ("数据库", None, None),
        ("索引", None, None),
        ("并发", None, None),
        ("异步", None, None),
        ("线程", None, None),
        ("智能指针", None, None),
        ("所有权", None, None),
        ("借用", None, None),
        ("生命周期", None, None),
    ];

    for (word, freq, tag) in tech_words {
        jieba.add_word(word, freq, tag);
    }
    
    Arc::new(jieba)
});

#[derive(Clone)]
pub struct ChineseTokenizer;

impl Tokenizer for ChineseTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let tokens = JIEBA.cut_for_search(text, true)  // 启用搜索模式，更细粒度的分词
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                Token {
                    offset_from: 0,  // 简化处理，不计算具体偏移
                    offset_to: 0,
                    position: i,  // 使用 usize
                    text: word.to_string(),
                    position_length: 1,
                }
            })
            .collect::<Vec<_>>();

        BoxTokenStream::from(VecTokenStream::new(tokens))
    }
}

pub struct VecTokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl VecTokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }
}

impl TokenStream for VecTokenStream {
    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn token(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.tokens[self.index - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_tokenizer() {
        let tokenizer = ChineseTokenizer;
        let mut stream = tokenizer.token_stream("Rust异步编程和智能指针的最佳实践");
        
        let mut tokens = Vec::new();
        while stream.advance() {
            tokens.push(stream.token().text.clone());
        }
        
        assert!(tokens.contains(&"Rust".to_string()));
        assert!(tokens.contains(&"异步".to_string()));
        assert!(tokens.contains(&"智能指针".to_string()));
    }
} 