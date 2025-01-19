#!/bin/bash

# 遍历所有图库文章
for file in content/gallery/*.md; do
    # 跳过 _index.md
    if [[ $(basename "$file") == "_index.md" ]]; then
        continue
    fi
    
    # 获取文件名（不含扩展名）
    filename=$(basename "$file" .md)
    
    # 更新文章内容
    cat > "$file" << EOL
+++
title = "${filename}"
description = "图片描述"
date = 2024-01-01  # 添加日期字段

[extra]
image = "images/${filename}.jpeg"  # 使用对应的图片文件
+++

${filename} 的详细描述...
EOL
done 