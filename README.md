# 个人博客

基于 Zola 静态站点生成器构建的个人博客，主要包含文章和图库功能。

## 文件结构

```
.
├── config.toml                # Zola 配置文件
├── content/                   # 内容目录
│   ├── about.md              # 关于页面
│   ├── blog/                 # 博客文章目录
│   ├── friends.md            # 友链页面
│   ├── gallery/              # 图库目录
│   └── hello-world.md        # 示例文章
├── static/                    # 静态资源
│   ├── favicon.ico           # 网站图标
│   ├── images/               # 图片资源
│   ├── js/                   # JavaScript 文件
│   ├── logo.jpeg             # 网站 Logo
│   └── style.css             # 全局样式
├── scripts/                   # 脚本工具
│   └── update_gallery.sh     # 图库更新脚本
└── templates/                 # 模板文件
    ├── about.html            # 关于页面模板
    ├── base.html             # 基础模板
    ├── blog-page.html        # 文章页面模板
    ├── blog.html             # 博客列表模板
    ├── friends.html          # 友链页面模板
    ├── gallery-page.html     # 图片详情模板
    ├── gallery.html          # 图库列表模板
    ├── index.html            # 首页模板
    ├── shortcodes/           # 短代码模板
    ├── taxonomy_list.html    # 分类列表模板
    └── taxonomy_single.html  # 分类详情模板
```

## 主要文件说明

### 1. 模板文件

#### templates/base.html
基础模板，定义网站的整体结构：
- 导航栏和页脚
- 主题切换功能
- 全局样式和脚本引入
- SEO 相关配置

#### templates/blog-page.html
文章页面模板，负责渲染单篇文章：
```html
{% extends "base.html" %}
```
- 文章内容渲染
- 代码高亮
- 响应式布局适配
- 阅读进度指示

#### templates/gallery.html
图库页面模板，实现图片展示功能：
```html
{% extends "base.html" %}
```
- 图片网格布局
- 懒加载优化
- 渐进式加载
- 悬停效果
- 错误处理
- 触摸设备支持

#### templates/shortcodes/markdown.html
Markdown 渲染增强：
- 代码块样式
- 图片优化
- 表格美化
- 引用样式
- 链接样式

### 2. 内容组织

#### content/blog/
博客文章目录，每篇文章包含：
```toml
+++
title = "文章标题"
date = 2024-01-01
description = "文章描述"
[taxonomies]
tags = ["标签1", "标签2"]
+++
```
- front matter 配置
- Markdown 内容
- 标签分类

#### content/gallery/
图片集目录：
```toml
+++
title = "图片标题"
description = "图片描述"
date = 2024-01-01
[extra]
image = "images/example.jpg"
+++
```
- 图片元数据
- 展示配置
- 分类信息

### 3. 配置文件

#### config.toml
Zola 站点配置：
```toml
base_url = "/"
title = "博客标题"
description = "博客描述"

[markdown]
highlight_code = true
render_emoji = true

[extra]
author = "作者名"
```
- 站点信息
- 构建配置
- 主题设置
- 自定义变量

## 功能实现细节

### 图库功能
- 使用 CSS Grid 实现响应式布局
- IntersectionObserver 实现图片懒加载
- CSS transforms 和 filters 实现过渡效果
- 使用 will-change 优化性能

### 性能优化
- 图片懒加载
- 代码分割
- 资源预加载
- 缓存策略
- 响应式图片

### 主题支持
- CSS 变量实现主题切换
- 媒体查询适配暗色模式
- 平滑过渡效果
- 主题持久化存储

## 开发和部署流程

### 环境要求

- [Zola](https://www.getzola.org/) 0.17.0 或更高版本

### 本地开发

1. 克隆仓库：
```bash
git clone https://github.com/yourusername/blog.git
cd blog
```

2. 启动 Zola 开发服务器：
```bash
zola serve
```

### 构建

```bash
# 构建站点
zola build
```

## 部署

本项目已配置为可直接部署到 Vercel：

1. Fork 本仓库
2. 在 Vercel 中导入项目
3. 部署

## 许可证

MIT License

## 致谢

- [Zola](https://www.getzola.org/)

## SEO 优化

本项目已实现以下 SEO 优化：

1. Meta 标签优化
   - 标题、描述、关键词
   - Open Graph 协议支持
   - Twitter Cards 支持

2. 结构化数据
   - 文章页面 Schema.org 标记
   - 面包屑导航标记

3. 技术优化
   - 响应式设计
   - 移动设备友好
   - 页面加载优化
   - 图片优化

4. 内容优化
   - 语义化 HTML
   - 合理的标题层级
   - 内部链接优化
   - 图片 ALT 属性

5. 站点地图
   - XML 站点地图
   - RSS Feed
   - robots.txt

6. 搜索引擎验证
   - Google Search Console
   - 百度站长平台
   - Bing Webmaster Tools