use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

mod handlers;
mod services;
mod state;

use services::root_service_config;
use state::{AppState, AppStateMutable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppStateMutable {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("ACTIX-STARTER"),
            })
            .app_data(shared_data.clone())
            .configure(root_service_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
