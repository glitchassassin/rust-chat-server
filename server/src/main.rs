pub mod db;
use self::db::{establish_connection, models::*};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

#[get("/users")]
async fn users() -> impl Responder {
    use self::db::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(users))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
