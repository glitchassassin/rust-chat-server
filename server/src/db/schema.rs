// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "room_permission"))]
    pub struct RoomPermission;
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoomPermission;

    user_room_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
        permission -> RoomPermission,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(user_room_permissions -> rooms (room_id));
diesel::joinable!(user_room_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    rooms,
    user_room_permissions,
    users,
);
