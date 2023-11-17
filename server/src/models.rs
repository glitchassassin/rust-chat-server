use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn get_all(
        conn: &mut PgConnection,
    ) -> Result<diesel::QueryResult<Vec<Self>>, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let results = users.select(User::as_select()).load(conn);

        Ok(results)
    }
}

#[derive(Queryable, Insertable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRoom {
    pub name: String,
}

impl Room {
    pub fn get_all(
        conn: &mut PgConnection,
    ) -> Result<diesel::QueryResult<Vec<Self>>, diesel::result::Error> {
        use crate::schema::rooms::dsl::*;

        let results = rooms.select(Room::as_select()).load(conn);

        Ok(results)
    }
    pub fn insert(
        conn: &mut PgConnection,
        new_room: NewRoom,
    ) -> Result<diesel::QueryResult<Self>, diesel::result::Error> {
        use crate::schema::rooms::dsl::*;

        let result = diesel::insert_into(rooms)
            .values(&new_room)
            .returning(Room::as_returning())
            .get_result(conn);

        Ok(result)
    }
}

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::RoomPermission"]
pub enum RoomPermission {
    Owner,
    Admin,
    Moderator,
    Member,
}

#[derive(Queryable, Insertable, Selectable, Serialize)]
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
