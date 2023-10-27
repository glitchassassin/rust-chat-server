use diesel::{prelude::*, sql_types::Timestamp};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Timestamp,
    // pub updated_at: Timestamp,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::user_room_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRoomPermission {
    pub id: i32,
    pub user_id: i32,
    pub room_id: i32,
    pub permission: crate::db::schema::sql_types::RoomPermission,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
