use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRoom {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize)]
#[ExistingTypePath = "crate::db::schema::sql_types::RoomPermission"]
pub enum RoomPermission {
    Owner,
    Admin,
    Moderator,
    Member,
}

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::user_room_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRoomPermission {
    #[serde(default)]
    pub id: i32,
    pub user_id: i32,
    pub room_id: i32,
    pub permission: RoomPermission,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
