// 搜索服务配置
const SEARCH_API = 'https://search-bpyd.shuttle.app/api/search';

// 添加搜索缓存
const searchCache = new Map();
const CACHE_EXPIRE_TIME = 5 * 60 * 1000; // 缓存5分钟

// 防抖函数优化
const debounce = (func, wait) => {
    let timeout;
    return function(...args) {
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(this, args), wait);
    };
};

// 执行搜索
async function performSearch() {
    const query = document.getElementById('search-input').value.trim();
    if (!query) return;

    const resultsContainer = document.getElementById('search-results');
    const searchBtn = document.querySelector('.search-header button');
    
    try {
        // 检查缓存
        const cacheKey = query.toLowerCase();
        const cachedResult = searchCache.get(cacheKey);
        if (cachedResult && (Date.now() - cachedResult.timestamp < CACHE_EXPIRE_TIME)) {
            displayResults(cachedResult.data, query);
            return;
        }

        // 显示加载状态
        searchBtn.disabled = true;
        searchBtn.textContent = '搜索中...';
        resultsContainer.innerHTML = '<div class="search-loading">搜索中...</div>';

        const response = await fetch(SEARCH_API, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ query })
        });
        
        if (!response.ok) {
            throw new Error(`搜索请求失败: ${response.status}`);
        }
        
        const results = await response.json();
        
        // 更新缓存
        searchCache.set(cacheKey, {
            data: results,
            timestamp: Date.now()
        });

        displayResults(results, query);
    } catch (error) {
        console.error('搜索出错:', error);
        resultsContainer.innerHTML = `
            <div class="search-error">
                搜索服务暂时不可用，请稍后再试<br>
                <small>${error.message}</small>
            </div>
        `;
    } finally {
        // 恢复按钮状态
        searchBtn.disabled = false;
        searchBtn.textContent = '搜索';
    }
}

// 优化防抖延迟
const debouncedSearch = debounce(performSearch, 200); // 减少延迟时间

// 添加预加载功能
function preloadCommonSearches() {
    const commonQueries = ['rust', 'zola', 'blog']; // 常用搜索词
    commonQueries.forEach(async query => {
        try {
            const response = await fetch(SEARCH_API, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ query })
            });
            
            if (response.ok) {
                const results = await response.json();
                searchCache.set(query, {
                    data: results,
                    timestamp: Date.now()
                });
            }
        } catch (error) {
            console.error('预加载搜索失败:', error);
        }
    });
}

// 页面加载完成后进行预加载
document.addEventListener('DOMContentLoaded', () => {
    // 延迟预加载，避免影响页面加载
    setTimeout(preloadCommonSearches, 2000);
});

// 实时搜索监听
document.getElementById('search-input').addEventListener('input', (e) => {
    const query = e.target.value.trim();
    if (query.length >= 2) { // 只在输入至少2个字符时触发搜索
        debouncedSearch();
    }
});

// 打开搜索对话框
function openSearch() {
    document.getElementById('search-dialog').style.display = 'flex';
    document.getElementById('search-input').focus();
}

// 关闭搜索对话框
document.getElementById('search-dialog').addEventListener('click', function(e) {
    if (e.target === this) {
        this.style.display = 'none';
    }
});

// 添加高亮函数
function highlightText(text, query) {
    const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`(${escapedQuery})`, 'gi');
    return text.replace(regex, '<mark>$1</mark>');
}

// 格式化博客 URL
function formatBlogUrl(path) {
    // 移除 .md 扩展名
    const withoutExt = path.replace(/\.md$/, '');
    
    // 如果路径以 blog/ 开头，移除它
    const cleanPath = withoutExt.replace(/^blog\//, '');
    
    // 构建完整的博客 URL
    return `/blog/${cleanPath}/`;
}

// 显示搜索结果
function displayResults(results, query) {
    const container = document.getElementById('search-results');
    
    if (!results || results.length === 0) {
        container.innerHTML = '<div class="no-result">未找到相关内容</div>';
        return;
    }

    const html = results.map(result => {
        const contextBefore = result.context_before
            ?.map(line => `<p class="context-line">${highlightText(line, query)}</p>`)
            .join('') || '';
            
        const contextAfter = result.context_after
            ?.map(line => `<p class="context-line">${highlightText(line, query)}</p>`)
            .join('') || '';

        // 使用 formatBlogUrl 处理路径
        const blogUrl = formatBlogUrl(result.path);

        return `
            <article class="search-result">
                <a href="${blogUrl}">
                    <div class="result-header">
                        <span class="result-path">${result.path}</span>
                        <span class="result-line">第 ${result.line_number} 行</span>
                    </div>
                    <div class="result-content">
                        ${contextBefore}
                        <p class="match-line">${highlightText(result.content, query)}</p>
                        ${contextAfter}
                    </div>
                </a>
            </article>
        `;
    }).join('');

    container.innerHTML = html;
}

// 添加样式
const style = document.createElement('style');
style.textContent = `
.search-result {
    background: #fff;
    border: 1px solid #eee;
    border-radius: 8px;
    margin-bottom: 1rem;
    overflow: hidden;
    transition: all 0.2s;
}

.search-result:hover {
    border-color: #ddd;
    box-shadow: 0 2px 8px rgba(0,0,0,0.05);
}

.search-result a {
    display: block;
    padding: 1rem;
    text-decoration: none;
    color: inherit;
}

.result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
    color: #666;
}

.result-path {
    color: #0366d6;
}

.result-line {
    color: #666;
}

.result-content {
    font-size: 0.925rem;
    line-height: 1.6;
}

.context-line {
    color: #666;
    margin: 0;
    padding: 0.125rem 0;
}

.match-line {
    background: #fffbdd;
    margin: 0;
    padding: 0.125rem 0;
}

mark {
    background: #fff5b1;
    padding: 0.125em 0;
    border-radius: 2px;
}

.search-loading {
    text-align: center;
    padding: 2rem;
    color: #666;
}

.search-error {
    text-align: center;
    padding: 2rem;
    color: #e53e3e;
}

.no-result {
    text-align: center;
    padding: 2rem;
    color: #666;
}
`;

document.head.appendChild(style);

// ESC 键关闭对话框
document.addEventListener('keydown', function(e) {
    if (e.key === 'Escape') {
        document.getElementById('search-dialog').style.display = 'none';
    }
});

// 添加回车键搜索支持
document.getElementById('search-input').addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        performSearch();
    }
}); 