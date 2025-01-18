use axum::{
    extract::Query,
    routing::{get, get_service},
    Router,
    Json,
    response::Redirect,
};
use http::{Method, HeaderName, StatusCode};
use serde::Deserialize;
use std::{sync::Arc, path::PathBuf};
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    services::ServeFile,
};

use search_engine::{SearchEngine, SearchResult};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub struct SearchServer {
    engine: Arc<SearchEngine>,
}

impl SearchServer {
    pub fn new(engine: SearchEngine) -> Self {
        Self {
            engine: Arc::new(engine),
        }
    }

    pub async fn run(self) {
        let cors = CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods([Method::GET])
            .allow_headers([HeaderName::from_static("content-type")]);
        
        let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("static");
        
        println!("静态文件目录: {:?}", static_dir);
        
        if !static_dir.exists() {
            println!("警告: 静态文件目录不存在: {:?}", static_dir);
            std::fs::create_dir_all(&static_dir).expect("创建静态文件目录失败");
        }
        
        let static_files_service = get_service(
            ServeDir::new(&static_dir)
                .append_index_html_on_directories(true)
                .not_found_service(ServeFile::new(static_dir.join("index.html")))
        );
        
        let app = Router::new()
            .route("/api/search", get(Self::search))
            .route("/blog/*path", get(Self::handle_blog_path))
            .fallback_service(static_files_service)
            .layer(cors)
            .with_state(self.engine);

        println!("搜索服务器运行在 http://localhost:3000");
        
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn handle_blog_path(
        axum::extract::Path(path): axum::extract::Path<String>,
    ) -> Result<Redirect, StatusCode> {
        // 分离路径和锚点
        let parts: Vec<&str> = path.split('#').collect();
        let base_path = parts[0].trim_start_matches('/').trim_end_matches('/');
        let anchor = parts.get(1).map(|&s| s).unwrap_or("");
        
        // 构建 Zola 开发服务器的 URL
        let url = if anchor.is_empty() {
            format!("http://127.0.0.1:1111/blog/{}/", base_path)
        } else {
            format!("http://127.0.0.1:1111/blog/{}/#{}", base_path, anchor)
        };
        
        Ok(Redirect::temporary(&url))
    }

    async fn search(
        Query(query): Query<SearchQuery>,
        engine: axum::extract::State<Arc<SearchEngine>>,
    ) -> Json<Vec<SearchResult>> {
        match engine.search(&query.q) {
            Ok(results) => Json(results),
            Err(_) => Json(vec![]),
        }
    }
} 