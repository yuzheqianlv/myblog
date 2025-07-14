# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Chinese personal blog built with Zola static site generator, featuring article publishing, image gallery, and full-text search functionality. The blog includes a separate Rust-based search service deployed on Shuttle platform.

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
# Update gallery content automatically
./scripts/update_gallery.sh
```

### Search Service (Separate Deployment)
```bash
# Deploy search service to Shuttle platform
cargo shuttle deploy
```

## Architecture

### Frontend (Zola Blog)
- **Static Site Generator**: Zola with Tera templating
- **Content**: Markdown files in `content/` directory
- **Templates**: HTML templates in `templates/` directory
- **Static Assets**: CSS, JavaScript, images in `static/` directory
- **Configuration**: `config.toml` for site settings

### Backend (Search Service)
- **Technology**: Rust service deployed on Shuttle platform
- **API Endpoint**: `https://search-bpyd.shuttle.app/api/search`
- **Functionality**: Full-text search with context highlighting
- **Note**: Source code not in this repository

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
- **Dark/Light Mode**: Automatic theme switching based on time and user preference
- **CSS Variables**: Consistent theming throughout the site
- **Time-based**: Configurable night mode hours (19:00-07:00 default)

## Configuration

### Site Settings (`config.toml`)
- Site title, description, and author information
- SEO meta tags and verification codes
- Markdown processing options (syntax highlighting, emoji support)
- Taxonomy configuration for tags
- Theme settings for dark mode

### Deployment Configuration
- **Vercel**: `vercel.json` for frontend deployment
- **Shuttle**: `Shuttle.toml` for search service deployment

## Content Management

### Adding Blog Posts
1. Create new `.md` file in `content/blog/`
2. Add frontmatter with title, date, tags, and description
3. Write content in Markdown

### Managing Gallery
1. Add images to `static/images/`
2. Create corresponding `.md` files in `content/gallery/`
3. Use `scripts/update_gallery.sh` to batch update gallery posts

### SEO Optimization
- Structured data markup in templates
- Open Graph and Twitter Card meta tags
- Sitemap generation enabled
- RSS feed support
- Image optimization and lazy loading

## Development Notes

### Search Service Integration
The search service is deployed separately and provides:
- Full-text search across all content
- Context highlighting with before/after lines
- Real-time search with debouncing
- Error handling and fallback UI

### Responsive Design
- Mobile-first approach with sidebar collapse
- Flexible grid layouts for gallery
- Touch-friendly navigation and search interface

### Performance Optimizations
- Client-side search result caching
- Image lazy loading in gallery
- Debounced search input
- Preloading of common search terms