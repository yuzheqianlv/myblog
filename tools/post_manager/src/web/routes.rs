use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    handlers::config(cfg);
} 