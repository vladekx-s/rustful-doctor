mod config;
use config::Config;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let cfg = Config::load();

    let server = Router::new().route("/", get(handler));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", cfg.port))
        .await
        .expect("Ошибка привязки адреса. Проверьте адрес");

    axum::serve(listener, server)
        .await
        .expect("Ошибка запуска сервера")
}

async fn handler() { /* ... */ }