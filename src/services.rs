use actix_web::{web};

use crate::handlers::handler_root;

pub fn root_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
        .route(web::get().to(handler_root))
    );
}