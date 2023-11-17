pub mod models;
pub mod rooms;
pub mod schema;
pub mod users;
use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use diesel::{prelude::*, r2d2};

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // connect to SQLite DB
    let manager = r2d2::ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Unable to connect to Postgres");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(rooms::routes())
            .service(users::routes())
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
