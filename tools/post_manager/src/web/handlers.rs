use actix_web::{web, HttpResponse, Result};
use tera::Tera;
use super::models::{Post, PostMetadata};
use std::fs::write;
use serde_json::json;
use actix_files::{Files, NamedFile};

pub async fn editor_page(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let ctx = tera::Context::new();
    let body = tmpl.render("editor.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("模板渲染失败"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

pub async fn list_posts() -> Result<HttpResponse> {
    let posts: Vec<Post> = vec![];
    Ok(HttpResponse::Ok().json(posts))
}

#[derive(serde::Deserialize)]
pub struct SavePostRequest {
    metadata: PostMetadata,
    content: String,
}

pub async fn create_post(post: web::Json<SavePostRequest>) -> Result<HttpResponse> {
    // 生成文件名
    let filename = post.metadata.title.to_lowercase()
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
    
    // 获取项目根目录
    let content_dir = std::env::current_dir()?
        .parent()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取父目录"))?
        .parent()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("无法获取项目根目录"))?
        .join("content")
        .join("blog");

    // 创建文章文件
    let file_path = content_dir.join(format!("{}.md", filename));
    
    // 组合文章内容
    let full_content = format!(
        "{}\n\n{}", 
        post.metadata.to_toml(),  // 使用 to_toml 方法
        post.content
    );

    // 保存文件
    write(&file_path, full_content)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("保存文件失败: {}", e)))?;

    Ok(HttpResponse::Created().json(json!({
        "message": "文章已保存",
        "path": file_path.to_string_lossy()
    })))
}

pub async fn get_post(_id: web::Path<String>) -> Result<HttpResponse> {
    // TODO: 获取指定文章
    Ok(HttpResponse::Ok().json(Post::default()))
}

pub async fn update_post(
    _id: web::Path<String>,
    _post: web::Json<Post>
) -> Result<HttpResponse> {
    // TODO: 更新文章
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_post(_id: web::Path<String>) -> Result<HttpResponse> {
    // TODO: 删除文章
    Ok(HttpResponse::NoContent().finish())
}

pub async fn upload_file(
    _payload: actix_multipart::Multipart
) -> Result<HttpResponse> {
    // TODO: 处理文件上传
    Ok(HttpResponse::Ok().finish())
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/posts", web::get().to(list_posts))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}", web::get().to(get_post))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
            .route("/upload", web::post().to(upload_file))
    )
    .service(
        Files::new("/static", "static")
            .show_files_listing()
            .use_last_modified(true)
            .use_etag(true)
            .prefer_utf8(true)
    )
    .service(
        web::resource("/favicon.ico")
            .route(web::get().to(|| async { 
                NamedFile::open("static/favicon.ico")
            }))
    )
    .route("/", web::get().to(editor_page));
} 