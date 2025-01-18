# Post Manager

博客文章管理工具。

## 功能

- 创建新文章模板
- 检查文章格式
- 修复常见问题

## 使用方法

### 创建新文章

```bash
cargo run -- new "文章标题" -d "文章描述" -t tag1,tag2
```

### 检查文章格式

```bash
# 检查单个文章
cargo run -- check -p path/to/post.md

# 检查所有文章
cargo run -- check
```

### 修复文章问题

```bash
# 修复单个文章
cargo run -- fix -p path/to/post.md

# 修复所有文章
cargo run -- fix
```

## 检查项目

- 前置元数据格式
- 标题格式
- 日期格式
- 标签格式
- Markdown 语法
- 图片引用
- 链接有效性

## 开发计划

- [ ] 支持更多文章检查项
- [ ] 添加批量操作功能
- [ ] 改进错误报告
- [ ] 添加自动修复建议 