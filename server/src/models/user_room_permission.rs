use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use super::db_error::DbError;

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::RoomPermission"]
pub enum RoomPermission {
    Owner,
    Admin,
    Moderator,
    Member,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::user_room_permissions)]
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

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::user_room_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserRoomPermission {
    pub user_id: i32,
    pub room_id: i32,
    pub permission: RoomPermission,
}

impl UserRoomPermission {
    pub fn add_permission(
        conn: &mut PgConnection,
        new_user_room_permission: NewUserRoomPermission,
    ) -> Result<Self, DbError> {
        use crate::schema::user_room_permissions::dsl::*;

        Ok(diesel::insert_into(user_room_permissions)
            .values(&new_user_room_permission)
            .returning(UserRoomPermission::as_returning())
            .get_result(conn)?)
    }
}
