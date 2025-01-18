# 搜索引擎实现

这是一个基于 Tantivy 的全文搜索引擎实现，支持中文分词、多字段搜索、上下文展示等功能。

## 目录结构

```
tools/search_engine/
├── src/
│   ├── lib.rs          # 核心搜索引擎实现
│   ├── server.rs       # HTTP 服务器实现
│   ├── tokenizer.rs    # 中文分词器实现
│   └── parser.rs       # Markdown 文档解析器
├── static/
│   └── index.html      # 搜索界面
└── README.md           # 本文档
```

## 核心功能

### 1. 索引管理 (lib.rs)

- **Schema 定义**
  - title: 文章标题，可搜索和存储
  - line_number: 行号，仅存储
  - line_content: 行内容，可搜索和存储
  - content: 完整内容，可搜索
  - tags: 标签，可搜索和存储
  - path: 文件路径，可搜索和存储

- **索引创建与更新**
  ```rust
  pub fn index_document(&self, writer: &mut IndexWriter, 
      title: &str, content: &str, tags: &[String], path: &str) -> Result<()>
  ```
  - 为每篇文章创建主文档
  - 为每一行创建独立索引
  - 标题行权重提升（1-3级标题分别权重 3.0/2.5/2.0）

- **索引健康检查**
  ```rust
  pub fn check_index_health(&self) -> Result<bool>
  ```
  - 检查索引是否为空
  - 验证索引完整性
  - 自动重建损坏的索引

### 2. 搜索实现 (lib.rs)

- **多级搜索策略**
  ```rust
  pub fn search(&self, query_text: &str) -> Result<Vec<SearchResult>>
  ```
  1. 优先搜索标题和行内容
  2. 如无结果，搜索完整内容
  3. 最后尝试搜索标题

- **上下文展示**
  ```rust
  fn get_line_context(&self, path: &str, target_line_number: u64) -> Result<(String, u64)>
  ```
  - 显示匹配行前后各5行
  - 行号标记
  - 匹配行高亮（➤ 标记）

### 3. 中文分词 (tokenizer.rs)

```rust
pub struct ChineseTokenizer;
impl TokenizerImpl for ChineseTokenizer { ... }
```
- 集成结巴分词
- 自定义分词流实现
- 支持中文搜索

### 4. 文档解析 (parser.rs)

```rust
pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub path: String,
    pub tags: Vec<String>,
}
```
- Markdown 文档解析
- 提取文章元数据
- 标签处理

### 5. Web 界面 (static/index.html)

- **搜索结果展示**
  - 文章标题和路径
  - 相关度得分
  - 匹配位置上下文
  - 标签展示

- **样式特性**
  - 响应式设计
  - 代码块样式
  - 高亮匹配文本
  - 行号显示

## 使用方法

1. 启动服务器：
```bash
cargo run
```

2. 访问搜索界面：
```
http://localhost:3000
```

3. 输入搜索词：
- 支持中文和英文
- 支持标题、内容搜索
- 实时展示匹配结果

## 特色功能

1. **精确定位**：
   - 显示匹配行的实际行号
   - 提供上下文预览
   - 支持直接跳转到匹配位置

2. **智能排序**：
   - 标题匹配权重更高
   - 考虑标题层级
   - 相关度评分

3. **用户体验**：
   - 实时搜索
   - 高亮显示
   - 清晰的结果展示

## 性能优化

1. **索引优化**：
   - 增量更新
   - 文档去重
   - 内存管理

2. **搜索优化**：
   - 多字段并行搜索
   - 结果缓存
   - 相关度优化

## 后续改进

1. 支持更多搜索特性：
   - 模糊搜索
   - 正则表达式
   - 高级过滤

2. 性能优化：
   - 索引压缩
   - 缓存优化
   - 并行处理

3. 用户界面：
   - 搜索建议
   - 结果预览
   - 快捷键支持 