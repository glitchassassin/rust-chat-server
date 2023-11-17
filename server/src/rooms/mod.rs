use actix_web::{
    dev::HttpServiceFactory,
    error,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    models::{NewRoom, Room},
    DbPool,
};

async fn rooms(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let rooms = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        Room::get_all(&mut conn)?
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(rooms))
}

async fn new_room(
    pool: web::Data<DbPool>,
    request: web::Json<NewRoom>,
) -> actix_web::Result<impl Responder> {
    let new_room = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        let new_room = NewRoom {
            name: request.name.clone(),
        };

        Room::insert(&mut conn, new_room)?
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(new_room))
}

pub fn routes() -> impl HttpServiceFactory {
    web::resource("/rooms")
        .route(web::get().to(rooms))
        .route(web::post().to(new_room))
}
