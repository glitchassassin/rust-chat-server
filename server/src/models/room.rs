use super::db_error::DbError;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Self>, DbError> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms.select(Room::as_select()).load(conn)?)
    }
    pub fn get(conn: &mut PgConnection, room_id: i32) -> Result<Self, DbError> {
        use crate::schema::rooms::dsl::*;

        Ok(rooms
            .select(Room::as_select())
            .filter(id.eq(room_id))
            .first(conn)?)
    }
    pub fn insert(conn: &mut PgConnection, new_room: NewRoom) -> Result<Self, DbError> {
        use crate::schema::rooms::dsl::*;

        Ok(diesel::insert_into(rooms)
            .values(&new_room)
            .returning(Room::as_returning())
            .get_result(conn)?)
    }
    pub fn delete(conn: &mut PgConnection, room_id: i32) -> Result<Self, DbError> {
        use crate::schema::rooms::dsl::*;

        Ok(diesel::delete(rooms)
            .filter(id.eq(room_id))
            .returning(Room::as_returning())
            .get_result(conn)?)
    }
}
