// 将模板定义移到最前面
const metadataTemplate = `+++
title = "文章标题"
date = "${new Date().toISOString().split('T')[0]}"
description = "文章描述"
tags = ["标签1", "标签2"]
categories = ["分类1"]
draft = true
+++

`;

const editor = document.getElementById('markdown-editor');
const preview = document.querySelector('.preview-content');
const lineNumbers = document.querySelector('.line-numbers');

// 配置 marked
marked.setOptions({
    gfm: true, // 启用 GitHub 风格 Markdown
    breaks: true, // 启用换行符
    pedantic: false, // 不要太严格
    smartLists: true, // 优化列表
    smartypants: true, // 优化标点符号
    highlight: function (code, language) {
        if (language && hljs.getLanguage(language)) {
            try {
                return hljs.highlight(code, { language }).value;
            } catch (err) {}
        }
        return code;
    }
});

// 自定义渲染器
const renderer = new marked.Renderer();

// 处理段落，避免添加多余的换行
renderer.paragraph = (text) => {
    return `<p>${text}</p>`;
};

// 处理代码块，精确控制行数
renderer.code = (code, language) => {
    const validLanguage = hljs.getLanguage(language) ? language : 'plaintext';
    const highlightedCode = hljs.highlight(code, { language: validLanguage }).value;
    // 添加 data-source-line 属性来标记源代码行号
    return `<div class="code-block-wrapper" data-source-line><pre><code class="hljs language-${validLanguage}">${highlightedCode}</code></pre></div>`;
};

// 处理列表，避免额外的空行
renderer.list = (body, ordered) => {
    const type = ordered ? 'ol' : 'ul';
    return `<${type}>${body}</${type}>`;
};

// 处理列表项，确保紧凑布局
renderer.listitem = (text) => {
    return `<li>${text}</li>`;
};

// 处理标题，避免额外的边距
renderer.heading = (text, level) => {
    return `<h${level}>${text}</h${level}>`;
};

// 更新 marked 配置
marked.setOptions({
    renderer,
    gfm: true,
    breaks: true,
    smartLists: true,
    smartypants: true
});

// 同步滚动控制
let isEditorScrolling = false;
let isPreviewScrolling = false;

// 计算元素的实际内容高度
function getContentHeight(element) {
    if (element === editor) {
        const lineHeight = parseFloat(getComputedStyle(editor).lineHeight);
        return editor.value.split('\n').length * lineHeight;
    } else {
        return element.scrollHeight;
    }
}

// 计算更精确的滚动百分比
function getScrollPercentage(element) {
    const viewportHeight = element.clientHeight;
    const contentHeight = getContentHeight(element);
    const maxScroll = contentHeight - viewportHeight;
    const currentScroll = element.scrollTop;
    
    return maxScroll > 0 ? (currentScroll / maxScroll) : 0;
}

// 计算元素的实际行数
function getElementLines(element) {
    const style = window.getComputedStyle(element);
    const lineHeight = parseFloat(style.lineHeight);
    
    if (element.classList.contains('code-block-wrapper')) {
        const codeElement = element.querySelector('code');
        const codeText = codeElement ? codeElement.textContent : '';
        // 查找对应的编辑器文本
        const editorLines = editor.value.split('\n');
        const currentLineNum = parseInt(element.dataset.startLine) || 1;
        
        // 在编辑器中查找这个代码块
        let foundCodeBlock = false;
        let codeBlockLine = 0;
        for (let i = currentLineNum - 1; i < editorLines.length; i++) {
            const line = editorLines[i].trim();
            if (line.startsWith('```')) {
                if (!foundCodeBlock) {
                    foundCodeBlock = true;
                    codeBlockLine = i;
                    break;
                }
            }
        }
        
        // 只计算代码块标记符号所在的行
        return 1;
    } else if (element.classList.contains('metadata-placeholder')) {
        return 8;
    } else {
        // 只计算非空行
        const text = element.textContent;
        const nonEmptyLines = text.split('\n').filter(line => line.trim()).length;
        return Math.max(1, nonEmptyLines);
    }
}

// 为每个预览元素添加行号属性
function addLineNumberAttributes(element, startLine) {
    element.dataset.startLine = startLine;
    if (element.classList.contains('code-block-wrapper')) {
        const codeElement = element.querySelector('code');
        const lines = codeElement ? codeElement.textContent.split('\n').length : 1;
        element.dataset.endLine = startLine + lines - 1;
    } else {
        const lines = element.textContent.split('\n').length;
        element.dataset.endLine = startLine + lines - 1;
    }
}

// 更新行号和内容对应关系
function updateLineNumbers() {
    const editorLines = editor.value.split('\n');
    const previewElements = preview.querySelectorAll('p, h1, h2, h3, h4, h5, h6, .code-block-wrapper, ul, ol, blockquote, .metadata-placeholder');
    
    let currentLine = 1;
    const lineMap = new Map();
    
    // 生成行号
    const numbers = editorLines.map((line, i) => {
        const lineNum = i + 1;
        const isCurrentLine = editor.value.substr(0, editor.selectionStart).split('\n').length === lineNum;
        return `<span class="${isCurrentLine ? 'active' : ''}" data-line="${lineNum}">${lineNum}</span>`;
    }).join('');
    
    lineNumbers.innerHTML = numbers;
    
    // 为预览元素添加行号属性
    previewElements.forEach(element => {
        if (element.classList.contains('code-block-wrapper')) {
            // 为代码块找到对应的源代码行
            const editorText = editor.value;
            const lines = editorText.split('\n');
            let codeBlockStart = -1;
            
            // 查找代码块开始标记
            for (let i = currentLine - 1; i < lines.length; i++) {
                if (lines[i].trim().startsWith('```')) {
                    codeBlockStart = i + 1;
                    break;
                }
            }
            
            if (codeBlockStart !== -1) {
                element.dataset.startLine = codeBlockStart;
                element.dataset.endLine = codeBlockStart;
                currentLine = codeBlockStart + 1;
            }
        } else {
            const lines = getElementLines(element);
            element.dataset.startLine = currentLine;
            element.dataset.endLine = currentLine + lines - 1;
            
            // 记录每一行对应的元素
            for (let i = currentLine; i <= currentLine + lines - 1; i++) {
                lineMap.set(i, element);
            }
            
            currentLine += lines;
        }
    });
    
    // 高亮当前行对应的元素
    const currentLineNum = editor.value.substr(0, editor.selectionStart).split('\n').length;
    const currentElement = lineMap.get(currentLineNum);
    if (currentElement) {
        previewElements.forEach(el => el.classList.remove('active'));
        currentElement.classList.add('active');
    }
}

// 优化滚动同步
const debouncedSyncScroll = debounce((source, target, percentage) => {
    syncScroll(source, target, percentage);
}, 16);

// 编辑器滚动处理
editor.addEventListener('scroll', () => {
    if (!isPreviewScrolling) {
        const percentage = getScrollPercentage(editor);
        requestAnimationFrame(() => {
            debouncedSyncScroll(editor, preview, percentage);
        });
    }
});

// 预览区域滚动处理
preview.addEventListener('scroll', () => {
    if (!isEditorScrolling) {
        const percentage = getScrollPercentage(preview);
        requestAnimationFrame(() => {
            debouncedSyncScroll(preview, editor, percentage);
        });
    }
});

// 优化滚动同步函数
function syncScroll(source, target, percentage) {
    if (source === editor && !isPreviewScrolling) {
        isEditorScrolling = true;
        
        // 计算目标滚动位置
        const previewScrollHeight = target.scrollHeight - target.clientHeight;
        const targetScrollTop = previewScrollHeight * percentage;
        
        // 同步滚动位置
        target.scrollTop = targetScrollTop;
        lineNumbers.scrollTop = editor.scrollTop;
        
        setTimeout(() => {
            isEditorScrolling = false;
        }, 150);
    } else if (source === preview && !isEditorScrolling) {
        isPreviewScrolling = true;
        
        // 计算目标滚动位置
        const editorScrollHeight = target.scrollHeight - target.clientHeight;
        const targetScrollTop = editorScrollHeight * percentage;
        
        // 同步滚动位置
        target.scrollTop = targetScrollTop;
        lineNumbers.scrollTop = targetScrollTop;
        
        setTimeout(() => {
            isPreviewScrolling = false;
        }, 150);
    }
}

// 更新预览内容时的滚动处理
function updatePreview() {
    const content = editor.value;
    
    try {
        // 分离元数据和内容
        const parts = content.split('+++');
        let markdown = content;
        let hasMetadata = false;
        
        if (parts.length > 2) {
            markdown = parts.slice(2).join('+++').trim();
            hasMetadata = true;
        }
        
        // 解析 Markdown
        const html = marked.parse(markdown);
        
        // 添加元数据占位
        preview.innerHTML = hasMetadata ? 
            '<div class="metadata-placeholder"></div>' + html :
            html;
        
        // 高亮代码块
        preview.querySelectorAll('pre code').forEach((block) => {
            hljs.highlightElement(block);
        });

        // 更新行号和对应关系
        requestAnimationFrame(() => {
            updateLineNumbers();
        });
    } catch (err) {
        console.error('预览失败:', err);
    }
}

// 优化预览更新
const debouncedPreview = debounce(updatePreview, 150);
const debouncedLineNumbers = debounce(updateLineNumbers, 50);

// 处理输入事件
editor.addEventListener('input', () => {
    debouncedLineNumbers();
    debouncedPreview();
});

// 处理光标移动
editor.addEventListener('click', () => {
    const currentLine = editor.value.substr(0, editor.selectionStart).split('\n').length;
    highlightCurrentLine(currentLine);
});

editor.addEventListener('keyup', (e) => {
    if (e.key === 'ArrowUp' || e.key === 'ArrowDown' || e.key === 'Home' || e.key === 'End') {
        const currentLine = editor.value.substr(0, editor.selectionStart).split('\n').length;
        highlightCurrentLine(currentLine);
    }
});

// 高亮当前行
function highlightCurrentLine(lineNum) {
    // 高亮行号
    lineNumbers.querySelectorAll('span').forEach(span => {
        span.classList.toggle('active', parseInt(span.dataset.line) === lineNum);
    });
    
    // 高亮预览元素
    preview.querySelectorAll('[data-start-line]').forEach(element => {
        const startLine = parseInt(element.dataset.startLine);
        const endLine = parseInt(element.dataset.endLine);
        element.classList.toggle('active', lineNum >= startLine && lineNum <= endLine);
    });
}

// 修改 Tab 键处理函数
function handleTab(e) {
    e.preventDefault();
    
    // 如果编辑器为空，插入元数据模板
    if (!editor.value.trim()) {
        editor.value = metadataTemplate;
        const titlePos = metadataTemplate.indexOf('"文章标题"');
        editor.setSelectionRange(titlePos + 1, titlePos + 5);
        debouncedPreview();
        debouncedLineNumbers();
        return;
    }
    
    // 获取当前行的缩进级别
    const pos = editor.selectionStart;
    const lines = editor.value.substr(0, pos).split('\n');
    const currentLine = lines[lines.length - 1];
    const match = currentLine.match(/^(\s*)/);
    const indent = match ? match[1] : '';
    
    // 在光标位置插入制表符或空格
    const spaces = '    ';
    const start = editor.selectionStart;
    const end = editor.selectionEnd;
    editor.value = editor.value.substring(0, start) + spaces + editor.value.substring(end);
    editor.selectionStart = editor.selectionEnd = start + spaces.length;
    
    // 触发更新
    debouncedPreview();
    debouncedLineNumbers();
    
    // 触发 input 事件以确保其他功能正常工作
    editor.dispatchEvent(new Event('input'));
}

// 处理键盘事件
editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        handleTab(e);
    } else if (e.key === 'Enter') {
        const pos = editor.selectionStart;
        const lines = editor.value.substr(0, pos).split('\n');
        const currentLine = lines[lines.length - 1];
        const match = currentLine.match(/^(\s*)([-*]\s)?/);
        
        if (match) {
            e.preventDefault();
            const [, indent, list] = match;
            
            // 如果当前行只有缩进和列表符号，则清除它们
            if (currentLine.trim() === (list || '').trim()) {
                const lastNewLine = editor.value.lastIndexOf('\n', pos - 1);
                editor.value = editor.value.substring(0, lastNewLine + 1) + editor.value.substring(pos);
                editor.selectionStart = editor.selectionEnd = lastNewLine + 1;
            } else {
                // 否则保持缩进和列表符号
                const insertion = '\n' + indent + (list || '');
                editor.value = editor.value.substring(0, pos) + insertion + editor.value.substring(pos);
                editor.selectionStart = editor.selectionEnd = pos + insertion.length;
            }
        }
    }
    
    updateLineNumbers();
});

// 初始化行号
updateLineNumbers();

// 设置占位文本
editor.placeholder = `按 Tab 键插入元数据模板...

或者开始写作:

# 文章标题

正文内容...`;

// 优化自动保存
const debouncedSave = debounce(async (content) => {
    try {
        await fetch('/api/posts/draft', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ content }),
        });
    } catch (err) {
        console.error('自动保存失败:', err);
    }
}, 1000);

// 自动保存处理
editor.addEventListener('input', () => {
    debouncedSave(editor.value);
});

// 初始化预览
updatePreview();

// 导入导出功能
const importBtn = document.getElementById('import-btn');
const exportBtn = document.getElementById('export-btn');
const fileInput = document.getElementById('file-input');

// 处理文件导入
importBtn.addEventListener('click', () => {
    fileInput.click();
});

fileInput.addEventListener('change', (e) => {
    const file = e.target.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            editor.value = e.target.result;
            debouncedPreview();
            debouncedLineNumbers();
            // 重置文件输入
            fileInput.value = '';
        };
        reader.readAsText(file);
    }
});

// 处理文件导出
exportBtn.addEventListener('click', () => {
    const content = editor.value;
    const blob = new Blob([content], { type: 'text/markdown' });
    
    // 从元数据中提取标题作为文件名
    let filename = 'untitled.md';
    const titleMatch = content.match(/title\s*=\s*"([^"]+)"/);
    if (titleMatch) {
        filename = `${titleMatch[1]}.md`;
    }
    
    // 创建下载链接
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    
    // 清理
    setTimeout(() => {
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }, 0);
});

// 支持拖放导入
editor.addEventListener('dragover', (e) => {
    e.preventDefault();
    editor.classList.add('dragover');
});

editor.addEventListener('dragleave', () => {
    editor.classList.remove('dragover');
});

editor.addEventListener('drop', (e) => {
    e.preventDefault();
    editor.classList.remove('dragover');
    
    const file = e.dataTransfer.files[0];
    if (file && (file.name.endsWith('.md') || file.name.endsWith('.markdown'))) {
        const reader = new FileReader();
        reader.onload = (e) => {
            editor.value = e.target.result;
            debouncedPreview();
            debouncedLineNumbers();
        };
        reader.readAsText(file);
    }
});

// 添加拖放样式
const dragoverStyle = `
.dragover {
    border: 2px dashed var(--primary-color) !important;
    background: var(--bg-color) !important;
}
`;

const styleSheet = document.createElement('style');
styleSheet.textContent = dragoverStyle;
document.head.appendChild(styleSheet);

// 导出功能
const exportMdBtn = document.getElementById('export-md');
const exportHtmlBtn = document.getElementById('export-html');

// 导出 Markdown
exportMdBtn.addEventListener('click', () => {
    const content = editor.value;
    const blob = new Blob([content], { type: 'text/markdown' });
    
    // 从元数据中提取标题作为文件名
    let filename = 'untitled.md';
    const titleMatch = content.match(/title\s*=\s*"([^"]+)"/);
    if (titleMatch) {
        filename = `${titleMatch[1]}.md`;
    }
    
    downloadFile(blob, filename);
});

// 导出 HTML
exportHtmlBtn.addEventListener('click', () => {
    const content = editor.value;
    
    // 分离元数据和内容
    const parts = content.split('+++');
    let markdown = content;
    let title = 'Untitled';
    
    if (parts.length > 2) {
        markdown = parts.slice(2).join('+++').trim();
        const titleMatch = parts[1].match(/title\s*=\s*"([^"]+)"/);
        if (titleMatch) {
            title = titleMatch[1];
        }
    }
    
    // 生成完整的 HTML 文档
    const htmlContent = `<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>${title}</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.7;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            color: #1e293b;
        }
        pre {
            background: #f8fafc;
            padding: 1rem;
            border-radius: 4px;
            overflow-x: auto;
            border: 1px solid #e2e8f0;
        }
        code {
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 0.875em;
        }
        blockquote {
            border-left: 4px solid #2563eb;
            margin: 0;
            padding-left: 1rem;
            color: #64748b;
        }
        img {
            max-width: 100%;
            height: auto;
        }
        table {
            border-collapse: collapse;
            width: 100%;
        }
        th, td {
            border: 1px solid #e2e8f0;
            padding: 0.75rem;
        }
        th {
            background: #f8fafc;
        }
    </style>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/github.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"></script>
</head>
<body>
    ${marked.parse(markdown)}
    <script>
        hljs.highlightAll();
    </script>
</body>
</html>`;

    const blob = new Blob([htmlContent], { type: 'text/html' });
    downloadFile(blob, `${title}.html`);
});

// 通用下载函数
function downloadFile(blob, filename) {
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    
    // 清理
    setTimeout(() => {
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }, 0);
} 