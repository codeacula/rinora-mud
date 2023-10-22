use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

/// This command allows a user to select a character to log in to
pub struct SelectCharacter {}

impl GameCommand for SelectCharacter {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            warn!("No session data found.");
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            return false;
        }

        let Some(user) = world.get::<User>(command.entity) else {
            warn!("Couldn't find user entity");
            return false;
        };

        let db_repo = world.resource::<DbInterface>();

        let does_own = db_repo
            .characters
            .does_user_own_character(&command.keyword.clone(), user.id);

        if !does_own {
            info!("User doesn't own that character.");
        }

        does_own
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(
            Res<DbInterface>,
            Res<RoomMap>,
            ResMut<CharacterMap>,
            Query<&mut UserSessionData>,
            Query<&mut EntityCollection>,
            Commands,
        )> = SystemState::new(world);
        let (db_repo, room_map, mut character_map, mut query, mut room_query, mut commands) =
            system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        // Make sure character exists
        let Some(character) = db_repo.characters.get_character_by_name(&command.keyword)? else {
            world.send_event(CharacterNotFound(command.entity));
            return Ok(());
        };

        // Make sure room is mapped
        let Some(room_entity) = room_map.get_room(&character.location) else {
            warn!("Unable to find character's room in the room map.");
            world.send_event(GenericErrorEvent(command.entity));
            return Ok(());
        };

        // They're set to be placed in game
        let character_id = character.info.character_id;
        let location_id = character.location.0;
        let character_entity = commands.spawn(character).id();

        user_sesh.status = UserStatus::InGame;

        character_map.0.insert(character_id, character_entity);

        if let Ok(mut entities) = room_query.get_mut(character_entity) {
            entities.0.push(character_entity);
        }

        // Tag this character as being controlled by the player
        commands
            .entity(character_entity)
            .insert(IsControlledBy(command.entity));

        user_sesh.controlling_entity = Some(character_entity);

        debug!(
            "Tagged character entity {character_entity:?} as controlled by entity {:?}",
            command.entity
        );

        debug!("Spawned character in room {location_id:?} entity {room_entity:?}");

        world.send_event(EntityEnteredWorld {
            entity: character_entity,
            room_entity_is_in: room_entity,
            triggered_by: MovementTriggeredBy::Login,
        });

        world.send_event(EntityEnteredRoom {
            entity: character_entity,
            room_entity_is_in: room_entity,
            triggered_by: MovementTriggeredBy::Login,
        });

        system_state.apply(world);

        Ok(())
    }
}
