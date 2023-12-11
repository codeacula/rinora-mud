use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbCharacter {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub current_room_id: i32,
    pub current_hp: i32,
    pub current_mp: i32,
    pub pronouns: i16,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::characters)]
pub struct NewDbCharacter {
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub current_room_id: i32,
    pub pronouns: i16,
}
