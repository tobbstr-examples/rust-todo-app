mod app;
mod shared;

use app::{App, Config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = config_from_environment_variables();
    let app = App::new(cfg).await;
    app.run().await
}

fn config_from_environment_variables() -> Config {
    let db_username = std::env::var("DB_USERNAME").unwrap_or("postgres".to_string());
    let db_password = std::env::var("DB_PASSWORD").unwrap_or("postgres".to_string());
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = std::env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_name = std::env::var("DB_INSTANCE").unwrap_or("postgres".to_string());
    let http_host = std::env::var("HTTP_HOST").unwrap_or("localhost".to_string());
    let http_port = std::env::var("HTTP_PORT").unwrap_or("8080".to_string());

    Config {
        db_username,
        db_password,
        db_host,
        db_port: db_port.parse().expect("DB_PORT must be a number"),
        db_name,
        http_host: http_host,
        http_port: http_port.parse().expect("HTTP_PORT must be a number"),
    }
}
