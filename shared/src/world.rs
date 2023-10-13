use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct EntityCollection(pub Vec<Entity>);

#[derive(Component)]
pub struct Exits(pub Vec<Entity>);

#[derive(Component)]
pub struct Plane {
    pub plane_id: i32,
}

#[derive(Bundle)]
pub struct PlaneBundle {
    pub plane: Plane,
    pub name: DisplayName,
    pub description: Description,
    pub continents: EntityCollection,
}

#[derive(Component)]
pub struct Continent {
    pub continent_id: i32,
    pub plane_id: i32,
    pub areas: Vec<Entity>,
}

#[derive(Bundle)]
pub struct ContinentBundle {
    pub continent: Continent,
    pub name: DisplayName,
    pub description: Description,
    pub areas: EntityCollection,
}

#[derive(Component)]
pub struct Area {
    pub area_id: i32,
    pub continent_id: i32,
}

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub name: DisplayName,
    pub description: Description,
    pub rooms: EntityCollection,
}

#[derive(Component, Clone)]
pub struct Environment {
    pub environment_id: i32,
}

#[derive(Bundle)]
pub struct EnvironmentBundle {
    pub environment: Environment,
    pub name: DisplayName,
    pub rooms: EntityCollection,
}

#[derive(Component, Debug)]
pub struct Room {
    pub room_id: i32,
    pub area_id: i32,
    pub environment_id: i32,
}

#[derive(Bundle)]
pub struct RoomBundle {
    pub room: Room,
    pub name: DisplayName,
    pub description: Description,
    pub exits: Exits,
    pub entities: EntityCollection,
}

#[derive(Component)]
pub struct Exit {
    pub exit_id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
}

#[derive(Bundle)]
pub struct ExitBundle {
    pub exit: Exit,
    pub to: ExitTo,
}

/// Indicates the component has an exit to that entity
#[derive(Component)]
pub struct ExitTo(pub Entity);

#[derive(Component)]
pub struct Location(pub i32);

#[derive(Event, Debug)]
pub struct EntityEnteredRoom {
    pub entity: Entity,
    pub room: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredWorld {
    pub entity: Entity,
    pub room: Entity,
}
