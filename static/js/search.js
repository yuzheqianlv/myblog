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
        const response = await fetch(`${SEARCH_API}?q=${encodeURIComponent(query)}`);
        if (!response.ok) throw new Error('搜索请求失败');
        
        const results = await response.json();
        displayResults(results);
    } catch (error) {
        resultsContainer.innerHTML = `
            <div class="search-error">
                搜索出错: ${error.message}
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

    const html = results.map(result => `
        <article class="search-result">
            <a href="${result.path}">
                <h3 class="result-title">${result.title}</h3>
                <p class="result-content">${result.excerpt}</p>
                ${result.tags ? `
                    <div class="tags">
                        ${result.tags.map(tag => `<span class="tag">${tag}</span>`).join('')}
                    </div>
                ` : ''}
            </a>
        </article>
    `).join('');

    container.innerHTML = html;
}

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

document.getElementById('search-input').addEventListener('input', 
    debounce(performSearch, 300)
); 