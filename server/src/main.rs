pub mod db;
pub mod rooms;
use self::db::{establish_connection, models::*};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

#[get("/init")]
async fn initialize() -> impl Responder {
    use self::db::schema::rooms::dsl::*;

    let connection = &mut establish_connection();

    let new_rooms = vec![
        NewRoom {
            name: "General".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
        NewRoom {
            name: "Random".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        },
    ];

    diesel::insert_into(rooms)
        .values(&new_rooms)
        .returning(Room::as_returning())
        .get_result(connection)
        .expect("Error creating room");
    format!("Database initialized!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(initialize)
            .service(rooms::routes())
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
