use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use atom_syndication::Feed;
use serde::Deserialize;
use source::{SourceType, Sources};
use std::str::FromStr;

mod source;
mod state;

#[cfg(debug_assertions)]
const IP: &'static str = "127.0.0.1";

#[cfg(not(debug_assertions))]
const IP: &'static str = "0.0.0.0";

const fn default_port() -> u16 {
    3000
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}

#[get("/{id}/{name}/feed")]
async fn index(
    path: web::Path<(String, String)>,
    data: web::Data<state::State>,
) -> Result<HttpResponse> {
    let t = SourceType::from_str(&path.0)
        .map_err::<HttpResponse, _>(|_| HttpResponse::NotFound().body("not found"))?;

    let feed: Feed = data
        .sources()
        .fetch(t, &path.1)
        .await
        .ok_or(HttpResponse::NotFound().body("not found"))??
        .into();

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(feed.to_string()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = envy::from_env::<Config>().expect("Failed to load config.");

    HttpServer::new(|| {
        App::new()
            .data(state::State::new(Sources::new()))
            .wrap(Logger::default())
            .service(index)
            .default_service(web::route().to(|| HttpResponse::NotFound().body("not found")))
    })
    .bind((IP, config.port))?
    .run()
    .await
}
