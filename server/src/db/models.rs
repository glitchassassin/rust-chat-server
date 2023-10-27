use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::RoomPermission"]
pub enum RoomPermission {
    Owner,
    Admin,
    Moderator,
    Member,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::user_room_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRoomPermission {
    pub id: i32,
    pub user_id: i32,
    pub room_id: i32,
    pub permission: RoomPermission,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
