# 千虑者的博客

基于 Zola 静态网站生成器构建的个人博客。

## 项目结构

```
.
├── config.toml            # 网站配置文件
├── content/              # 内容目录
│   ├── blog/            # 博客文章
│   │   ├── _index.md   # 博客列表页配置
│   │   └── *.md        # 文章文件
│   ├── about.md        # 关于页面
│   └── friends.md      # 友链页面
├── static/              # 静态资源
│   ├── style.css       # 样式文件
│   ├── favicon.ico     # 网站图标
│   └── images/         # 图片目录
├── templates/          # 模板文件
│   ├── base.html      # 基础模板
│   ├── index.html     # 首页模板
│   ├── blog.html      # 博客列表模板
│   ├── blog-page.html # 文章页面模板
│   └── *.html         # 其他模板
└── tools/             # 工具目录
    └── gallery_generator/  # 图库生成工具
```

## 功能特点

- 🎨 简洁现代的设计风格
- 📱 响应式布局，支持移动设备
- 🏷️ 文章标签分类
- 💻 代码高亮支持
- 🖼️ 图片展示功能
- 🔗 友情链接页面

## 本地开发

1. 安装 Zola
```bash
# macOS
brew install zola

# 其他系统请参考：https://www.getzola.org/documentation/getting-started/installation/
```

2. 克隆项目
```bash
git clone <repository-url>
cd blog
```

3. 本地预览
```bash
zola serve
```

4. 构建网站
```bash
zola build
```

## 内容创建

### 创建新文章

1. 在 `content/blog/` 目录下创建 Markdown 文件
2. 添加必要的前置配置：
```markdown
+++
title = "文章标题"
date = 2025-01-18
description = "文章描述"

[taxonomies]
tags = ["标签1", "标签2"]
+++

文章内容...
```

### 添加图片

1. 将图片文件放入 `static/images/` 目录
2. 使用工具生成图库文章：
```bash
cd tools/gallery_generator
cargo run
```

## 自定义修改

### 样式修改

编辑 `static/style.css` 文件：
- 颜色变量在 `:root` 选择器中定义
- 布局相关样式在对应的类选择器中

### 模板修改

- 修改 `templates/` 目录下的相应模板文件
- 所有模板都继承自 `base.html`

### 配置修改

编辑 `config.toml` 文件：
- 网站基本信息
- Markdown 渲染配置
- 分类方式设置

## 部署

1. 推送到 GitHub
```bash
git add .
git commit -m "Update content"
git push
```

2. Vercel 自动部署
- 连接 GitHub 仓库
- 使用项目根目录的 `vercel.json` 配置

## 待办事项

### 🦀 Rust 工具开发
- [ ] 使用 Rust 开发文章管理 CLI 工具
  - [ ] 新建文章模板
  - [ ] 批量修改文章元数据
  - [ ] 文章格式检查和修复
- [ ] 开发 Rust 实现的文章搜索引擎
  - [ ] 全文索引
  - [ ] 标签智能推荐
  - [ ] 相关文章推荐
- [ ] 图片处理工具增强
  - [ ] 自动压缩和优化
  - [ ] WebP 格式转换
  - [ ] EXIF 信息提取
- [ ] Markdown 增强工具
  - [ ] 自动生成目录
  - [ ] 链接有效性检查
  - [ ] 图片引用检查

### 🎨 界面优化
- [ ] 深色模式支持
- [ ] 移动端适配优化
- [ ] 阅读进度指示
- [ ] 代码块复制按钮
- [ ] 图片预览优化

### 📝 内容功能
- [ ] 评论系统集成
- [ ] RSS 订阅优化
- [ ] 文章系列支持
- [ ] 阅读时间估算
- [ ] 文章版本控制

### 🚀 性能优化
- [ ] 静态资源优化
- [ ] 图片懒加载
- [ ] 预渲染优化
- [ ] 缓存策略优化

### 🔧 开发体验
- [ ] 开发环境 Docker 化
- [ ] GitHub Actions 自动化
- [ ] 开发文档完善
- [ ] 单元测试覆盖

## 为什么选择 Rust？

1. **性能优势**
   - 零成本抽象
   - 无 GC 暂停
   - 接近 C 的性能

2. **安全性**
   - 内存安全保证
   - 线程安全保证
   - 无数据竞争

3. **开发效率**
   - 强大的类型系统
   - 优秀的错误处理
   - 丰富的工具链

4. **跨平台支持**
   - 支持主流操作系统
   - WebAssembly 支持
   - 交叉编译能力

## 工具开发路线图

### 第一阶段：基础工具
1. 完善图库生成器
2. 开发文章管理工具
3. 实现基本的搜索功能

### 第二阶段：功能增强
1. 添加图片处理功能
2. 实现文章分析工具
3. 开发内容检查工具

### 第三阶段：高级特性
1. 实现智能推荐系统
2. 添加性能分析工具
3. 开发自动化工作流

## 参与贡献

如果你也对使用 Rust 开发博客工具感兴趣，欢迎参与：

1. Fork 项目
2. 创建特性分支
3. 提交变更
4. 推送到分支
5. 创建 Pull Request

## 开发指南

详细的开发指南请参考 `tools/` 目录下各工具的 README 文件。

## 贡献

欢迎提交 Issue 和 Pull Request。

## 许可证

MIT

## 作者

千虑者