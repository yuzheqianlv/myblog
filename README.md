# 千虑者的博客

基于 Zola 静态网站生成器构建的个人博客。

## 项目结构

```
.
├── config.toml            # 网站配置文件
├── content/              # 内容目录
│   ├── blog/            # 博客文章目录
│   │   ├── _index.md   # 博客列表页配置
│   │   └── *.md        # 文章文件
│   ├── about.md        # 关于页面
│   └── friends.md      # 友链页面
├── static/              # 静态资源目录
│   ├── css/            # 样式文件目录
│   │   ├── style.css   # 主样式文件
│   │   ├── code.css    # 代码高亮样式
│   │   └── mobile.css  # 移动端适配样式
│   ├── js/             # JavaScript 文件目录
│   │   ├── main.js     # 主要脚本
│   │   └── search.js   # 搜索功能脚本
│   ├── favicon.ico     # 网站图标
│   └── images/         # 图片目录
├── templates/          # 模板文件目录
│   ├── base.html      # 基础模板
│   ├── index.html     # 首页模板
│   ├── blog.html      # 博客列表模板
│   ├── blog-page.html # 文章页面模板
│   ├── tags.html      # 标签页面模板
│   └── search.html    # 搜索页面模板
└── tools/             # 工具目录
    ├── gallery_generator/  # 图库生成工具
    │   ├── src/          # 源代码
    │   └── templates/    # 图库模板
    └── search_engine/    # 搜索引擎工具
        ├── src/          # 源代码
        └── static/       # 静态资源
```

## 目录功能说明

### content/ 目录

博客内容的核心目录,包含所有文章和页面:

- `blog/`: 存放所有博客文章
  - 文章使用 Markdown 格式
  - 支持 front matter 配置(标题、日期、标签等)
  - 支持数学公式、代码高亮
- `_index.md`: 用于配置博客列表页面
- `about.md`: 关于页面内容
- `friends.md`: 友情链接页面

### static/ 目录

存放所有静态资源:

- `css/`: 样式文件
  - `style.css`: 定义全局样式、布局、颜色主题
  - `code.css`: 代码高亮样式配置
  - `mobile.css`: 移动端适配样式
- `js/`: JavaScript 脚本
  - `main.js`: 实现交互功能
  - `search.js`: 搜索功能实现
- `images/`: 图片资源
  - 支持 jpg、png、webp 等格式
  - 按文章分类存放

### templates/ 目录

Zola 模板文件:

- `base.html`: 定义网站整体结构
  - 包含导航栏、页脚
  - 引入 CSS 和 JavaScript
- `index.html`: 首页模板
  - 展示最新文章
  - 特色内容展示
- `blog.html`: 文章列表页
  - 分页功能
  - 标签筛选
- `blog-page.html`: 文章详情页
  - 目录导航
  - 代码高亮
  - 评论系统

### tools/ 目录

开发工具集:

- `gallery_generator/`: 图库生成工具
  - 自动处理图片尺寸
  - 生成缩略图
  - 创建图库页面
- `search_engine/`: 搜索引擎实现
  - 基于 tantivy 搜索引擎
  - 支持中英文搜索
  - 实现相关文章推荐

## 样式设计

### 颜色主题

```css
:root {
  /* 主题色 */
  --primary-color: #3498db;
  --secondary-color: #2ecc71;
  
  /* 文字颜色 */
  --text-primary: #2c3e50;
  --text-secondary: #7f8c8d;
  
  /* 背景色 */
  --bg-primary: #ffffff;
  --bg-secondary: #f5f6fa;
  
  /* 代码块 */
  --code-bg: #282c34;
  --code-color: #abb2bf;
}
```

### 响应式布局

- 桌面端(>= 1024px):
  - 内容区域最大宽度 1200px
  - 双栏布局
  - 固定侧边栏

- 平板端(768px - 1023px):
  - 内容区域最大宽度 90%
  - 单栏布局
  - 可折叠侧边栏

- 移动端(<= 767px):
  - 内容区域宽度 95%
  - 单栏布局
  - 隐藏侧边栏
  - 底部导航栏

### 排版

- 标题
  - H1: 2rem, 主标题
  - H2: 1.5rem, 章节标题
  - H3: 1.25rem, 小节标题
  
- 正文
  - 字体: system-ui
  - 字号: 16px
  - 行高: 1.6

- 代码
  - 等宽字体: Fira Code
  - 行内代码背景色
  - 代码块圆角边框

## 开发指南

### 本地开发

1. 安装依赖
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Zola
brew install zola
```

2. 克隆项目
```bash
git clone <repository-url>
cd blog
```

3. 运行开发服务器
```bash
zola serve
```

### 新建文章

1. 创建 Markdown 文件
```bash
touch content/blog/new-post.md
```

2. 添加 front matter
```markdown
+++
title = "文章标题"
date = 2024-01-18
description = "文章描述"
[taxonomies]
tags = ["标签1", "标签2"]
+++
```

3. 编写文章内容

### 构建部署

1. 构建静态文件
```bash
zola build
```

2. 部署到 Vercel
```bash
vercel deploy
```

## 许可证

MIT

## 作者

千虑者