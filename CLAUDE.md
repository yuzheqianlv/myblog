# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Chinese personal blog built with Zola static site generator, featuring article publishing, image gallery, and full-text search functionality. The blog includes a separate Rust-based search service deployed on Shuttle platform.

## Critical Development Rules

- **NEVER modify the `/zola/` directory** - it contains the complete Zola source code for reference only
- **Chinese-first content** - Primary language is Chinese (zh-CN) with technical terms in English
- **Preserve SEO optimizations** - Extensive meta tags, structured data, and search engine verification codes are already implemented

## Development Commands

### Local Development
```bash
# Install Zola (if not already installed)
brew install zola

# Start development server with live reload
zola serve

# Build static files for production
zola build
```

### Gallery Management
```bash
# Update gallery content automatically (batch processes .md files in content/gallery/)
./scripts/update_gallery.sh

# Manual process: Add images to static/images/ first, then create corresponding .md files
```

### Search Service (Separate Deployment)
```bash
# Deploy search service to Shuttle platform
cargo shuttle deploy
```

### Deployment
```bash
# Vercel deployment uses specific Zola version (v0.17.2)
# Build command in vercel.json downloads and uses Zola directly
# No additional setup needed for Vercel deployment
```

## Architecture

### Frontend (Zola Blog)
- **Static Site Generator**: Zola v0.17.2 with Tera templating engine
- **Language**: Chinese (zh-CN) with Unicode support, English technical terms
- **Content**: Markdown files with TOML frontmatter in `content/` directory
- **Templates**: Tera HTML templates in `templates/` directory with comprehensive SEO
- **Static Assets**: CSS, JavaScript, images in `static/` directory
- **Configuration**: `config.toml` with Chinese localization, search indexing, syntax highlighting

### Backend (Search Service)
- **Technology**: Rust + Axum framework deployed on Shuttle platform
- **API Endpoint**: `https://search-bpyd.shuttle.app/api/search`
- **Search Engine**: ripgrep-based full-text search with context highlighting
- **Fallback**: Client-side Fuse.js search when remote service unavailable
- **Note**: Search service source code not in this repository

### Hybrid Search Architecture
- **Primary**: Remote Rust service with ripgrep backend
- **Secondary**: Local Fuse.js with client-side indexing
- **Caching**: 5-minute client-side cache using Map storage
- **Features**: Real-time search, debouncing, context highlighting, preloading

### Key Components

#### Content Structure
- `content/blog/`: Blog articles in Markdown
- `content/gallery/`: Image gallery posts
- `content/about.md`: About page
- `content/friends.md`: Friends/links page
- `content/search.md`: Search page content

#### Templates
- `templates/base.html`: Base template with sidebar, navigation, and theme switching
- `templates/blog.html`: Blog listing page
- `templates/blog-page.html`: Individual blog post
- `templates/gallery.html`: Gallery listing
- `templates/search.html`: Search interface

#### Search Integration
- **Frontend**: `static/js/search.js` handles API calls, caching, and result display
- **Features**: Real-time search, debouncing, context highlighting, mobile-responsive
- **Caching**: 5-minute client-side cache with Map storage
- **Preloading**: Common search terms preloaded on page load

### Theme System
- **Dark/Light Mode**: Automatic theme switching based on time (19:00-07:00) and user preference
- **CSS Variables**: Consistent theming throughout the site
- **Syntax Highlighting**: Dual theme support with CSS class-based switching
  - Light theme: `base16-ocean-light` → `syntax-theme-light.css`
  - Dark theme: `base16-ocean-dark` → `syntax-theme-dark.css`
- **Configuration**: Time-based auto-switching configurable in `config.toml`

## Configuration

### Site Settings (`config.toml`)
- **Localization**: Chinese language (`zh`) as default with UTF-8 support
- **Search**: Fuse.js format indexing with content truncation (1500 chars)
- **Markdown**: Emoji support, smart punctuation, external link targets
- **Syntax Highlighting**: CSS class-based with dual theme support
- **Taxonomy**: Tags with RSS/feed generation enabled
- **SEO**: Verification codes for Google, Baidu, Bing search engines
- **Theme**: Auto dark mode with configurable time range (19:00-07:00)

### Deployment Configuration
- **Vercel**: `vercel.json` downloads Zola v0.17.2 and builds to `public/`
- **Shuttle**: `Shuttle.toml` for independent search service deployment
- **Build Process**: No Node.js dependencies, pure static site generation

## Content Management

### Adding Blog Posts
1. Create new `.md` file in `content/blog/` with TOML frontmatter
2. Required frontmatter: `title`, `date`, `description`, optional `tags`
3. Content in Markdown with Chinese text and English technical terms
4. Syntax highlighting automatically applied for code blocks

### Managing Gallery
1. **Manual Process**: Add images to `static/images/` directory first
2. Create corresponding `.md` files in `content/gallery/` with TOML frontmatter
3. **Automated Process**: Use `./scripts/update_gallery.sh` for batch operations
   - Script generates standardized frontmatter for all gallery items
   - Maps image files to corresponding .md files automatically
   - Skips `_index.md` files during processing

### SEO Optimization
- **Meta Tags**: Comprehensive title, description, keywords for Chinese SEO
- **Structured Data**: Schema.org markup for articles and breadcrumbs
- **Social Media**: Open Graph and Twitter Card integration
- **Search Engines**: Verification codes for Google, Baidu, Bing
- **Performance**: Sitemap, RSS feeds, image optimization, lazy loading

## Development Guidelines

### Content Standards
- **Language**: Chinese primary content with English for technical terms
- **Encoding**: UTF-8 throughout, proper Chinese character handling
- **SEO**: Meta descriptions in Chinese, structured data for search engines
- **Images**: Add to `static/images/` before creating gallery posts

### Technical Considerations
- **Search**: Dual-system architecture (remote Rust + local Fuse.js)
- **Themes**: CSS class-based syntax highlighting for theme switching
- **Performance**: Client-side caching, debounced search, lazy loading
- **Deployment**: Vercel auto-builds with downloaded Zola binary

### Code Organization
- **Templates**: Tera templating with Chinese/English mixed content
- **Static Assets**: Organized by type (js/, images/, CSS in root)
- **Configuration**: All settings centralized in `config.toml`
- **Scripts**: Gallery automation in `scripts/` directory