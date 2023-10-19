use crate::output::get_login_screen::*;
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
            world.send_event(InvalidCharacterName(command.entity));
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

        world.send_event(TextEvent::new(entity, &get_login_screen(&characters)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
