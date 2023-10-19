use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

pub struct CharacterWasSelected {}

impl GameCommand for CharacterWasSelected {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            info!("No session data found.");
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            info!("User isn't logged in.");
            return false;
        }

        let Some(user) = world.get::<User>(command.entity) else {
            info!("Couldn't find user entity");
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
            Query<&mut Exits>,
            EventWriter<EntityEnteredWorld>,
            EventWriter<EntityEnteredRoom>,
            EventWriter<TextEvent>,
            Commands,
        )> = SystemState::new(world);
        let (
            db_repo,
            room_map,
            mut character_map,
            mut query,
            mut room_query,
            mut ent_entered_world_tx,
            mut ent_entered_room_tx,
            mut text_event_tx,
            mut commands,
        ) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        // Make sure character exists
        let Some(character) = db_repo.characters.get_character_by_name(&command.keyword)? else {
            warn!("Unable to locate character even after validating they exist & are owned.");
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        };

        // Make sure room is mapped
        let Some(room_entity) = room_map.get_room(&character.location) else {
            warn!("Unable to find character's room in the room map.");
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        };

        // They're set to be placed in game
        let character_id = character.info.character_id;
        let location_id = character.location.0;

        user_sesh.status = UserStatus::InGame;
        let character_entity = commands.spawn(character).id();

        character_map.0.insert(character_id, character_entity);

        if let Ok(mut exits) = room_query.get_mut(character_entity) {
            exits.0.push(character_entity);
        }

        // Tag this character as being controlled by the player
        commands
            .entity(character_entity)
            .insert(IsControlledBy(command.entity));

        user_sesh.controlling_entity = Some(character_entity);

        debug!(
            "Tagged character entity {:?} as controlled by entity {:?}",
            character_entity, command.entity
        );

        debug!(
            "Spawned character in room {:?} entity {:?}",
            location_id, room_entity
        );

        ent_entered_world_tx.send(EntityEnteredWorld {
            entity: character_entity,
            room_entity_is_in: room_entity,
            triggered_by: MovementTriggeredBy::Login,
        });

        ent_entered_room_tx.send(EntityEnteredRoom {
            entity: character_entity,
            room_entity_is_in: room_entity,
            triggered_by: MovementTriggeredBy::Login,
        });

        system_state.apply(world);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
