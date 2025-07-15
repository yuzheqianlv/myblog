// Fuse.js 搜索引擎实现
class FuseSearch {
    constructor() {
        this.fuse = null;
        this.searchData = null;
        this.options = {
            includeScore: true,
            includeMatches: true,
            threshold: 0.3,          // 模糊匹配阈值
            location: 0,             // 匹配位置
            distance: 100,           // 搜索距离
            maxPatternLength: 32,    // 最大模式长度
            minMatchCharLength: 1,   // 最小匹配字符长度 (中文优化)
            keys: [
                { name: 'title', weight: 0.4 },
                { name: 'description', weight: 0.3 },
                { name: 'body', weight: 0.3 }
            ]
        };
    }

    async init() {
        try {
            const response = await fetch('/search_index.zh.json');
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            this.searchData = await response.json();
            this.fuse = new Fuse(this.searchData, this.options);
            console.log('Fuse搜索引擎初始化成功，索引了', this.searchData.length, '篇文章');
            return true;
        } catch (error) {
            console.error('Fuse搜索引擎初始化失败:', error);
            return false;
        }
    }

    search(query) {
        if (!this.fuse) {
            console.error('搜索引擎未初始化');
            return [];
        }

        const results = this.fuse.search(query);
        
        // 过滤和去重结果
        const filteredResults = this.filterAndDeduplicateResults(results);
        
        return filteredResults.map(result => ({
            item: result.item,
            score: result.score,
            matches: result.matches
        }));
    }

    // 过滤和去重搜索结果
    filterAndDeduplicateResults(results) {
        const seenTitles = new Map();
        const filteredResults = [];

        for (const result of results) {
            const item = result.item;
            
            // 跳过无效页面
            if (!item.title || item.title.trim() === '' || 
                item.url.includes('/search/') ||
                item.url.includes('/gallery/') ||
                (!item.body || item.body.trim() === '')) {
                continue;
            }

            const title = item.title.trim();
            
            // 如果已经看过这个标题
            if (seenTitles.has(title)) {
                const existingResult = seenTitles.get(title);
                
                // 优先选择博客版本（URL包含/blog/）
                if (item.url.includes('/blog/') && !existingResult.item.url.includes('/blog/')) {
                    // 替换为博客版本
                    const index = filteredResults.findIndex(r => r.item.title === title);
                    if (index !== -1) {
                        filteredResults[index] = result;
                        seenTitles.set(title, result);
                    }
                }
                // 如果当前是非博客版本而已存在博客版本，跳过
                continue;
            }

            // 新标题，添加到结果中
            seenTitles.set(title, result);
            filteredResults.push(result);
        }

        return filteredResults;
    }

    // 生成高亮摘要
    generateHighlightedTeaser(item, matches, maxLength = 200) {
        let content = item.body || item.description || '';
        
        if (!matches || matches.length === 0) {
            return content.substring(0, maxLength) + (content.length > maxLength ? '...' : '');
        }

        // 找到最佳匹配片段
        const bodyMatches = matches.filter(m => m.key === 'body');
        const descriptionMatches = matches.filter(m => m.key === 'description');
        
        let bestMatch = bodyMatches.length > 0 ? bodyMatches[0] : 
                       descriptionMatches.length > 0 ? descriptionMatches[0] : null;

        if (!bestMatch || !bestMatch.indices || bestMatch.indices.length === 0) {
            return content.substring(0, maxLength) + (content.length > maxLength ? '...' : '');
        }

        // 获取匹配内容
        const sourceContent = bestMatch.key === 'body' ? item.body : item.description;
        if (!sourceContent) {
            return content.substring(0, maxLength) + (content.length > maxLength ? '...' : '');
        }

        const indices = bestMatch.indices[0];
        const matchStart = indices[0];
        const matchEnd = indices[1];
        
        // 计算摘要范围
        const contextLength = 80; // 匹配前后的上下文长度
        const start = Math.max(0, matchStart - contextLength);
        const end = Math.min(sourceContent.length, matchEnd + contextLength);
        
        let teaser = sourceContent.substring(start, end);
        
        // 添加高亮
        const matchText = sourceContent.substring(matchStart, matchEnd + 1);
        const teaserMatchStart = matchStart - start;
        const teaserMatchEnd = teaserMatchStart + matchText.length;
        
        teaser = teaser.substring(0, teaserMatchStart) + 
                '<mark>' + teaser.substring(teaserMatchStart, teaserMatchEnd) + '</mark>' +
                teaser.substring(teaserMatchEnd);
        
        return (start > 0 ? '...' : '') + teaser + (end < sourceContent.length ? '...' : '');
    }
}

// 防抖函数
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// 搜索引擎实例
const fuseSearch = new FuseSearch();

// 初始化搜索功能
async function initFuseSearch() {
    const searchInput = document.getElementById("search");
    const searchResults = document.getElementById("search-results");
    const searchResultsItems = document.querySelector(".search-results__items");
    const searchStats = document.getElementById("search-stats");
    const searchEmpty = document.getElementById("search-empty");
    const MAX_ITEMS = 10;

    if (!searchInput || !searchResults || !searchResultsItems) {
        console.warn('搜索元素未找到，可能不在搜索页面');
        return;
    }

    // 初始化搜索引擎
    const initSuccess = await fuseSearch.init();
    if (!initSuccess) {
        searchResultsItems.innerHTML = '<li class="search-results__error">搜索功能初始化失败</li>';
        return;
    }

    let currentTerm = "";

    // 执行搜索函数
    function performSearch() {
        const query = searchInput.value.trim();
        
        // 避免重复搜索
        if (query === currentTerm) {
            return;
        }
        
        currentTerm = query;
        
        // 空搜索隐藏结果
        if (query === "") {
            searchResults.style.display = "none";
            searchEmpty.style.display = "none";
            if (searchStats) searchStats.style.display = "none";
            return;
        }

        // 显示加载状态
        searchResults.style.display = "block";
        searchResultsItems.innerHTML = '<li class="search-results__loading">搜索中...</li>';
        
        // 执行搜索
        const results = fuseSearch.search(query);
        displayResults(results, query);
    }

    // 显示搜索结果
    function displayResults(results, query) {
        searchResultsItems.innerHTML = '';
        
        if (results.length === 0) {
            searchResults.style.display = "none";
            searchEmpty.style.display = "block";
            return;
        }
        
        // 隐藏空状态，显示结果
        searchEmpty.style.display = "none";
        searchResults.style.display = "block";
        
        // 显示搜索统计
        if (searchStats) {
            const searchCount = document.querySelector('.search-count');
            if (searchCount) {
                searchCount.textContent = `找到 ${results.length} 条结果`;
            }
            searchStats.style.display = "block";
        }

        const fragment = document.createDocumentFragment();
        const displayCount = Math.min(results.length, MAX_ITEMS);
        
        for (let i = 0; i < displayCount; i++) {
            const result = results[i];
            const teaser = fuseSearch.generateHighlightedTeaser(
                result.item, 
                result.matches
            );
            
            const li = document.createElement('li');
            li.className = 'search-results__item';
            li.innerHTML = `
                <a href="${result.item.url}">${result.item.title || '无标题'}</a>
                <div class="search-results__teaser">${teaser}</div>
                <div class="search-results__url">${result.item.url}</div>
                <div class="search-results__score">相关度: ${Math.round((1 - result.score) * 100)}%</div>
            `;
            fragment.appendChild(li);
        }
        
        searchResultsItems.appendChild(fragment);
        
        // 显示结果统计
        if (results.length > MAX_ITEMS) {
            const moreInfo = document.createElement('li');
            moreInfo.className = 'search-results__more-info';
            moreInfo.innerHTML = `<em>显示前 ${MAX_ITEMS} 条，共找到 ${results.length} 条结果</em>`;
            searchResultsItems.appendChild(moreInfo);
        }
    }

    // 搜索输入事件处理
    const debouncedSearch = debounce(performSearch, 300);
    searchInput.addEventListener("input", debouncedSearch);

    // 回车键搜索
    searchInput.addEventListener("keydown", function(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            performSearch();
        }
    });

    // 点击外部关闭搜索结果
    document.addEventListener('click', function(e) {
        if (searchResults.style.display === "block" && 
            !searchResults.contains(e.target) && 
            !searchInput.contains(e.target)) {
            searchResults.style.display = "none";
        }
    });

    // ESC键关闭搜索结果
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && searchResults.style.display === "block") {
            searchResults.style.display = "none";
            searchInput.blur();
        }
    });

    // 输入框获得焦点时如果有内容则显示结果
    searchInput.addEventListener('focus', function() {
        if (searchInput.value.trim() !== '') {
            searchResults.style.display = "block";
        }
    });
    
    // 清除搜索功能
    const searchClear = document.getElementById('search-clear');
    if (searchClear) {
        searchInput.addEventListener('input', function() {
            if (searchInput.value.trim()) {
                searchClear.style.display = 'block';
            } else {
                searchClear.style.display = 'none';
            }
        });
        
        searchClear.addEventListener('click', function() {
            searchInput.value = '';
            searchClear.style.display = 'none';
            searchResults.style.display = 'none';
            searchEmpty.style.display = 'none';
            if (searchStats) searchStats.style.display = 'none';
            searchInput.focus();
        });
    }
    
    // 建议标签点击功能
    const suggestionTags = document.querySelectorAll('.suggestion-tag');
    suggestionTags.forEach(tag => {
        tag.addEventListener('click', function() {
            const query = this.dataset.query;
            searchInput.value = query;
            performSearch();
        });
    });

    console.log('Fuse搜索功能初始化完成');
}

// 页面加载完成后初始化搜索
if (document.readyState === "complete" || 
    (document.readyState !== "loading" && !document.documentElement.doScroll)) {
    initFuseSearch();
} else {
    document.addEventListener("DOMContentLoaded", initFuseSearch);
}