use actix_web::{web, App, HttpServer};
use tera::Tera;

mod handlers;
mod models;
mod routes;

pub async fn start_server(port: u16) -> std::io::Result<()> {
    let templates = Tera::new("templates/**/*").expect("模板加载失败");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(templates.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
} 