async function searchPosts() {
    const query = document.getElementById('search-input').value;
    if (!query) return;

    try {
        const response = await fetch(`http://localhost:3000/api/search?q=${encodeURIComponent(query)}`);
        if (!response.ok) {
            throw new Error(`搜索请求失败: ${response.status}`);
        }
        
        const results = await response.json();
        displaySearchResults(results);
    } catch (error) {
        console.error('搜索失败:', error);
        const container = document.getElementById('search-results');
        container.innerHTML = `<div class="search-error">搜索出错: ${error.message}</div>`;
    }
}

function displaySearchResults(results) {
    const container = document.getElementById('search-results');
    
    if (!results || results.length === 0) {
        container.innerHTML = '<div class="search-empty">未找到相关文章</div>';
        return;
    }

    container.innerHTML = '';
    results.forEach(result => {
        const article = document.createElement('article');
        article.className = 'search-result';
        article.innerHTML = `
            <h3><a href="${result.path}">${result.title}</a></h3>
            <p>${result.excerpt}</p>
            <div class="tags">
                ${result.tags.map(tag => `<span class="tag">${tag}</span>`).join('')}
            </div>
        `;
        container.appendChild(article);
    });
} 