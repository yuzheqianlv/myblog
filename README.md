# 个人博客

基于 Zola 静态站点生成器构建的个人博客，主要包含文章和图库功能。

## 功能特点

1. 文章
   - Markdown 支持
   - 代码高亮
   - 标签分类
   - 文章目录
   - 行号显示

2. 图库
   - 图片预览
   - 懒加载优化
   - 响应式布局

3. 搜索功能
   - 全文搜索
   - 实时搜索
   - 搜索词高亮
   - 上下文预览
   - 行号定位

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
│   ├── js/                   # JavaScript 文件
│   │   └── search.js         # 搜索功能脚本
│   ├── css/                  # 样式文件
│   └── images/               # 图片资源
├── templates/                 # 模板目录
│   ├── base.html             # 基础模板
│   ├── blog.html             # 博客列表模板
│   ├── blog-page.html        # 博客文章模板
│   ├── gallery.html          # 图库模板
│   └── search.html           # 搜索页面模板
└── vercel.json               # Vercel 部署配置
```

## 搜索服务

博客使用独立的搜索服务提供全文搜索功能：

- 基于 Rust + Axum 构建
- 使用 ripgrep 进行全文搜索
- 部署在 Shuttle 平台
- 支持实时搜索和结果高亮
- 提供上下文预览

## 部署说明

1. 博客部署
   ```bash
   # 安装 Zola
   brew install zola

   # 本地预览
   zola serve

   # 构建静态文件
   zola build
   ```

2. 搜索服务部署
   ```bash
   # 部署到 Shuttle
   cargo shuttle deploy
   ```

## SEO 优化

1. 基础优化
   - 语义化 HTML
   - 合理的标题层级
   - 内部链接优化
   - 图片 ALT 属性

2. 性能优化
   - 静态资源压缩
   - 图片懒加载
   - 代码分割
   - 缓存策略

3. 站点地图
   - XML 站点地图
   - RSS Feed
   - robots.txt

4. 搜索引擎验证
   - Google Search Console
   - 百度站长平台
   - Bing Webmaster Tools

## 开发计划

1. [ ] 评论功能
2. [ ] 暗色主题
3. [ ] 文章统计
4. [ ] 阅读时间估算
5. [ ] 相关文章推荐

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