use actix_web::{dev::HttpServiceFactory, get, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::{
    establish_connection,
    models::{NewRoom, Room},
};

async fn rooms() -> impl Responder {
    use crate::db::schema::rooms::dsl::*;

    let connection = &mut establish_connection();
    let results = rooms
        .select(Room::as_select())
        .load(connection)
        .expect("Error loading rooms");
    HttpResponse::Ok().json(results)
}

#[derive(Deserialize)]
struct PostNewRoom {
    name: String,
}

async fn new_room(request: web::Json<PostNewRoom>) -> impl Responder {
    use crate::db::schema::rooms::dsl::*;

    let connection = &mut establish_connection();

    let new_room = NewRoom {
        name: request.name.clone(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(rooms)
        .values(&new_room)
        .returning(Room::as_returning())
        .get_result(connection)
        .expect("Error creating room");
    HttpResponse::Ok().json(new_room)
}

pub fn routes() -> impl HttpServiceFactory {
    web::resource("/rooms")
        .route(web::get().to(rooms))
        .route(web::post().to(new_room))
}
