// 搜索服务配置
const SEARCH_API = 'https://search-bpyd.shuttle.app/api/search';

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

// 执行搜索
async function performSearch() {
    const query = document.getElementById('search-input').value.trim();
    if (!query) return;

    const resultsContainer = document.getElementById('search-results');
    resultsContainer.innerHTML = '<div class="search-loading">搜索中...</div>';

    try {
        // 使用 POST 请求
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
        displayResults(results);
    } catch (error) {
        console.error('搜索出错:', error);
        resultsContainer.innerHTML = `
            <div class="search-error">
                搜索服务暂时不可用，请稍后再试<br>
                <small>${error.message}</small>
            </div>
        `;
    }
}

// 显示搜索结果
function displayResults(results) {
    const container = document.getElementById('search-results');
    
    if (!results || results.length === 0) {
        container.innerHTML = '<div class="no-result">未找到相关内容</div>';
        return;
    }

    const html = results.map(result => {
        const title = result.path.split('/').pop().replace('.md', '');
        return `
            <article class="search-result">
                <a href="${result.path}">
                    <h3 class="result-title">${title}</h3>
                    <div class="result-meta">
                        <span class="result-path">${result.path}</span>
                        <span class="result-line">第 ${result.line_number} 行</span>
                    </div>
                    <p class="result-content">${result.content}</p>
                </a>
            </article>
        `;
    }).join('');

    container.innerHTML = html;
}

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

// 添加输入防抖
const debounce = (func, wait) => {
    let timeout;
    return function(...args) {
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(this, args), wait);
    };
};

// 实时搜索（延迟 300ms）
document.getElementById('search-input').addEventListener('input', 
    debounce(performSearch, 300)
); 