# Gallery Generator

一个用于自动生成 Zola 博客图库文章的工具。

## 功能特点

- 自动扫描 `static/images` 目录下的图片
- 为每张图片生成对应的 Markdown 文件
- 支持 jpg、jpeg、png、gif、webp 格式
- 自动设置文章日期、标题和图片路径
- 生成基本的文章模板

## 使用前提

1. 安装 Rust 开发环境
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 确保项目结构正确：
```
myblog/
├── static/
│   └── images/     # 存放图片的目录
├── content/
│   └── gallery/    # 存放生成的文章
└── tools/
    └── gallery_generator/
```

## 使用方法

1. 准备图片
```bash
# 在博客根目录执行
mkdir -p static/images
# 复制图片到 images 目录
cp your-images/* static/images/
```

2. 运行生成器
```bash
# 进入工具目录
cd tools/gallery_generator

# 编译并运行
cargo run
```

3. 检查生成的文件
```bash
ls ../../content/gallery/
```

## 生成的文件格式

每个生成的 Markdown 文件格式如下：

```markdown
+++
title = "图片文件名"
date = YYYY-MM-DD
description = "图片描述"

[taxonomies]
tags = ["图片"]

[extra]
image = "/images/图片文件名"
+++

这是一张图片的描述...
```

## 自定义修改

生成文件后，你可能需要：

1. 修改文章标题使其更有意义
2. 添加合适的描述
3. 添加更多标签
4. 调整发布日期

## 注意事项

- 图片文件名请使用有意义的英文名称
- 建议图片分辨率适中，避免文件过大
- 生成后请检查并编辑 Markdown 文件的内容
- 确保 `static/images` 目录存在

## 开发相关

如需修改生成的模板格式，编辑 `src/main.rs` 中的 `content` 变量：

```rust
let content = format!(
    r#"+++
title = "{}"
date = {}
description = "图片描述"
// ... 自定义其他字段
"#,
    file_name,
    date
);
```

## 依赖说明

- chrono = "0.4" - 处理日期
- walkdir = "2.3" - 文件系统遍历

## 许可证

MIT 