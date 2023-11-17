use actix_web::{
    dev::HttpServiceFactory,
    error,
    web::{self},
    HttpResponse, Responder, ResponseError,
};

use crate::{
    models::{NewUser, User},
    DbPool,
};

async fn users(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        User::get_all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}

async fn new_user(
    pool: web::Data<DbPool>,
    request: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let new_user = web::block(move || {
        let mut conn = pool.get()?;

        let new_user = NewUser {
            name: request.name.clone(),
            email: request.email.clone(),
        };

        User::insert(&mut conn, new_user)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(new_user))
}

pub fn routes() -> impl HttpServiceFactory {
    web::resource("/users")
        .route(web::get().to(users))
        .route(web::post().to(new_user))
}
