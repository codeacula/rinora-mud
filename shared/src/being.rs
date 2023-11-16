use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Being {
    pub pronouns: Pronouns,
}

#[derive(Component)]
pub struct Character {
    pub character_id: i32,
    pub user_id: i32,
}

#[derive(Component)]
pub struct Pronouns(pub i16);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub being: Being,
    pub description: Description,
    pub display_name: DisplayName,
    pub info: Character,
    pub location: Location,
    pub health: Health,
    pub mana: Mana,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            being: Being {
                pronouns: Pronouns(3), // Default to "they/them" for now
            },
            description: Description("".to_string()),
            display_name: DisplayName("".to_string()),
            health: Health { current: 0, max: 0 },
            mana: Mana { current: 0, max: 0 },
            location: Location {
                entity: Entity::PLACEHOLDER,
                location_id: 0,
            },
            info: Character {
                character_id: 0,
                user_id: 0,
            },
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct IsControlledBy(pub Entity);

#[derive(Component)]
pub struct Mana {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug)]
pub struct EntityWantsToMove {
    pub who_entity: Entity,
    pub exit_entity: Entity,
}

#[derive(Event, Debug)]
pub struct EntityMovedRooms {
    pub moving_entity: Entity,
    pub from_room: Entity,
    pub to_room: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredRoomEvent {
    pub entity: Entity,
    pub room_entity_is_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredWorldEvent {
    pub entity: Entity,
    pub room_entity_is_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLeftRoomEvent {
    pub entity: Entity,
    pub room_entity_was_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLeftWorldEvent {
    pub entity: Entity,
    pub room_entity_was_in: Entity,
}

#[derive(Event, Debug)]
pub struct PromptUserForCharacterName(pub Entity);

#[derive(Event, Debug, Copy, Clone)]
pub struct MoveEntityToRoom {
    pub entity: Entity,
    pub room: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredPlaneEvent {
    pub entity: Entity,
    pub plane_entity_is_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLeftPlaneEvent {
    pub entity: Entity,
    pub plane_entity_was_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredContinentEvent {
    pub entity: Entity,
    pub continent_entity_is_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLeftContinentEvent {
    pub entity: Entity,
    pub continent_entity_was_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityEnteredAreaEvent {
    pub entity: Entity,
    pub area_entity_is_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLeftAreaEvent {
    pub entity: Entity,
    pub area_entity_was_in: Entity,
}

#[derive(Event, Debug)]
pub struct EntityLoggedIn {
    pub entity: Entity,
    pub room_entity_is_in: Entity,
}
