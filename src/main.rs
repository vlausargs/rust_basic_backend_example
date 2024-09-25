mod auth;
mod base;
mod util;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use deadpool_postgres::Runtime;
use std::io::Result;
use tokio_postgres::NoTls;
use util::{env_loader, env_mapper::Config};

async fn status() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_loader::load_env();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config: Config = Config::from_env().unwrap();
    println!(
        "Server is running on http://{}:{}",
        config.server.host, config.server.port
    );

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/", web::get().to(status))
            .route(
                "/auth/login",
                web::post().to(auth::login::controller::post_controller::execute),
            )
            .route(
                "/auth/register",
                web::post().to(auth::register::controller::post_controller::execute),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
