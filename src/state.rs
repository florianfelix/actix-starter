use std::sync::Mutex;


pub struct AppState {
    pub app_name: String,
}

pub struct AppStateMutable {
    pub counter: Mutex<i32>,
}