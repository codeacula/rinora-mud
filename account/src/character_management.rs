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
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            return false;
        }

        let Some(user) = world.get::<User>(command.entity) else {
            return false;
        };

        let db_repo = world.resource::<DbInterface>();

        db_repo
            .characters
            .does_user_own_character(&command.keyword.clone(), user.id)
    }

    fn run(&self, _command: &UserCommand, _world: &mut World) -> Result<(), String> {
        /*let mut system_state: SystemState<(
            Res<DbInterface>,
            Res<GameSettings>,
            Query<(Entity, &User, &mut UserSessionData)>,
            EventWriter<TextEvent>,
        )> = SystemState::new(world);
        let (db_repo, settings, mut query, mut text_event_tx) = system_state.get_mut(world);
        let (entity, user, mut user_sesh) = query.get_mut(command.entity).unwrap();*/

        todo!()
    }
}
    