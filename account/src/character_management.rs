use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

pub struct ProvideCharacterName {}

impl GameCommand for ProvideCharacterName {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status != UserStatus::CreateCharacter {
            return false;
        }

        true
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        if command.parts.len() > 1 || !is_alphabetic(&command.keyword) {
            world.send_event(TextEvent::from_str(
                command.entity,
                "Character names can only contain the letters A-Z, and only one word. Please try again.",
            ));
            return Ok(());
        }

        let mut system_state: SystemState<(
            Res<DbInterface>,
            Res<GameSettings>,
            Query<(Entity, &User, &mut UserSessionData)>,
            EventWriter<TextEvent>,
        )> = SystemState::new(world);
        let (db_repo, settings, mut query, mut text_event_tx) = system_state.get_mut(world);

        let character_name = command.keyword.clone();
        let exists_res = db_repo.characters.does_character_exist(&character_name);

        if let Err(err) = exists_res {
            error!("Error checking if character exists: {:?}", err);
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        }

        if exists_res.unwrap() {
            text_event_tx.send(TextEvent::from_str(
                command.entity,
                "That character already exists. Please try a different name.",
            ));
            return Ok(());
        }

        let (entity, user, mut user_sesh) = query.get_mut(command.entity).unwrap();

        if let Err(err) =
            db_repo
                .characters
                .create_character(&character_name, settings.default_room, user)
        {
            error!("Error creating character for user: {:?}", err);
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        }

        text_event_tx.send(TextEvent::from_str(
            entity,
            "Character created! You can now select them from the login screen",
        ));

        user_sesh.status = UserStatus::LoggedIn;

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!(
                    "Unable to fetch user's characters after creating a character: {:?}",
                    e
                );
                world.send_event(TextEvent::send_generic_error(entity));
                return Ok(());
            }
        };

        world.send_event(TextEvent::new(
            entity,
            &crate::get_login_screen(&characters),
        ));
        Ok(())
    }
}

pub struct SelectedCreateCharacter {}

impl GameCommand for SelectedCreateCharacter {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            return false;
        }

        command.full_command == "1"
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>, EventWriter<TextEvent>)> =
            SystemState::new(world);
        let (mut query, mut text_event_tx) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "{{11}}What would you like your character's name to be?",
        ));

        user_sesh.status = UserStatus::CreateCharacter;

        Ok(())
    }
}
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
            Query<&mut Room>,
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
        let Some(room_entity) = room_map.0.get(&character.location.0) else {
            warn!("Unable to find character's room in the room map.");
            text_event_tx.send(TextEvent::send_generic_error(command.entity));
            return Ok(());
        };
        info!("User: {:?}", command.entity);

        // They're set to be placed in game
        let character_id = character.info.id;
        user_sesh.status = UserStatus::InGame;
        let character_entity = commands.spawn(character).id();

        character_map.0.insert(character_id, character_entity);

        if let Ok(mut room) = room_query.get_mut(character_entity) {
            room.entities.push(character_entity);
        }

        // Tag this character as being controlled by the player
        commands
            .entity(character_entity)
            .insert(IsControlledBy(command.entity));

        
        debug!("Tagged character entity {:?} as controlled by entity {:?}", character_entity, command.entity);

        ent_entered_world_tx.send(EntityEnteredWorld {
            entity: character_entity,
            room: *room_entity,
        });

        ent_entered_room_tx.send(EntityEnteredRoom {
            entity: character_entity,
            room: *room_entity,
        });

        system_state.apply(world);

        Ok(())
    }
}
