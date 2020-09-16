use actix_web::{web, Responder, HttpResponse};

use crate::state::{AppState, AppStateMutable};

pub async fn handler_root(data: web::Data<AppState>, shared_data: web::Data<AppStateMutable>) -> impl Responder {
    // unmutable data
    let app_name = &data.app_name;

    // thread safe shared data
    let mut counter = shared_data.counter.lock().unwrap();
    *counter += 1;

    let response_body = format!("Hello from {}, Request Number: {}", app_name, counter);
    HttpResponse::Ok().body(&response_body)
}