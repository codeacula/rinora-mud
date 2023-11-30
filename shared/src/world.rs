use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct EntityCollection(pub Vec<Entity>);

#[derive(Component, Debug)]
pub struct Exits(pub Vec<Entity>);

#[derive(Component, Debug)]
pub struct Plane {
    pub plane_id: i32,
}

#[derive(Bundle, Debug)]
pub struct PlaneBundle {
    pub plane: Plane,
    pub name: DisplayName,
    pub description: Description,
    pub continents: EntityCollection,
}

#[derive(Component, Debug)]
pub struct Continent {
    pub continent_id: i32,
    pub plane_id: i32,
    pub areas: Vec<Entity>,
    pub plane: Entity,
}

#[derive(Bundle, Debug)]
pub struct ContinentBundle {
    pub continent: Continent,
    pub name: DisplayName,
    pub description: Description,
    pub areas: EntityCollection,
}

#[derive(Component, Debug)]
pub struct Area {
    pub area_id: i32,
    pub continent_id: i32,
    pub continent: Entity,
}

#[derive(Bundle, Debug)]
pub struct AreaBundle {
    pub area: Area,
    pub name: DisplayName,
    pub description: Description,
    pub rooms: EntityCollection,
}

#[derive(Component, Clone, Debug)]
pub struct Environment {
    pub environment_id: i32,
}

#[derive(Bundle, Debug)]
pub struct EnvironmentBundle {
    pub environment: Environment,
    pub name: DisplayName,
    pub rooms: EntityCollection,
}

#[derive(Component, Debug)]
pub struct Room {
    pub room_id: i32,
    pub area_id: i32,
    pub area: Entity,
    pub environment_id: i32,
}

#[derive(Bundle, Debug)]
pub struct RoomBundle {
    pub room: Room,
    pub name: DisplayName,
    pub description: Description,
    pub exits: Exits,
    pub entities: EntityCollection,
}

#[derive(Component, Debug)]
pub struct Exit {
    pub direction: String,
    pub exit_id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
    pub from_room: Entity,
    pub to_room: Entity,
}

#[derive(Bundle, Debug)]
pub struct ExitBundle {
    pub exit: Exit,
    pub to: ExitTo,
}

/// Indicates the component has an exit to that entity
#[derive(Component, Debug)]
pub struct ExitTo(pub Entity);

#[derive(Component, Debug, Clone, Copy)]
pub struct Location {
    pub location_id: i32,
    pub entity: Entity,
}
