use bevy::prelude::Component;

#[derive(Component)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub user_id: String,
}
