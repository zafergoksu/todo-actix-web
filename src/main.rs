mod config;
mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    for (k, v) in dotenv::vars() {
        println!("{}, {}", k, v);
    }

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(handlers::status))
            .route("/todos{_:/?}", web::get().to(handlers::get_todos))
            .route("/todos{_:/?}", web::post().to(handlers::create_todo))
            .route(
                "/todos/{list_id}/items{_:/?}",
                web::get().to(handlers::get_items),
            )
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::put().to(handlers::check_item),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
