use crate::models::{NewUserRoomPermission, RoomPermission, UserRoomPermission};

use super::db_error::DbError;

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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Self>, DbError> {
        use crate::schema::users::dsl::*;

        Ok(users.select(User::as_select()).load(conn)?)
    }
    pub fn get(conn: &mut PgConnection, user_id: i32) -> Result<Self, DbError> {
        use crate::schema::users::dsl::*;

        Ok(users
            .select(User::as_select())
            .filter(id.eq(user_id))
            .first(conn)?)
    }
    pub fn insert(conn: &mut PgConnection, new_user: NewUser) -> Result<Self, DbError> {
        use crate::schema::users::dsl::*;

        let user = diesel::insert_into(users)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)?;

        let default_channels = vec![1, 2];
        for room_id in default_channels {
            UserRoomPermission::add_permission(
                conn,
                NewUserRoomPermission {
                    user_id: user.id,
                    room_id,
                    permission: RoomPermission::Member,
                },
            )?;
        }

        Ok(user)
    }
    pub fn delete(conn: &mut PgConnection, user_id: i32) -> Result<Self, DbError> {
        use crate::schema::users::dsl::*;

        Ok(diesel::delete(users)
            .filter(id.eq(user_id))
            .returning(User::as_returning())
            .get_result(conn)?)
    }
}
