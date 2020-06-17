use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

fn default_port() -> u16 {
    3000
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

#[get("/{id}/{name}")]
async fn index(info: web::Path<(String, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Config>().expect("Failed to load config.");

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}
