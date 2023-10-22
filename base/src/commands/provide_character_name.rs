use crate::output::get_login_screen::*;
use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

/// Sets a name for the character the user is currently creating
///
/// ### Run Conditions
///
/// * Must have a user session
/// * Must be creating a character
pub struct ProvideCharacterName {}

impl GameCommand for ProvideCharacterName {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            warn!(
                "Somehow user didn't have session data: {:?}",
                command.entity
            );
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
            error!("Error checking if character exists: {err:?}");
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
mod tests {
    use shared::prelude::*;

    use super::ProvideCharacterName;

    fn get_app() -> App {
        let mut app = App::new();
        app.update();

        app
    }

    fn get_command() -> Box<dyn GameCommand> {
        Box::new(ProvideCharacterName {})
    }

    fn get_context() -> (App, Box<dyn GameCommand>, UserCommand) {
        return (get_app(), get_command(), get_user_command());
    }

    fn get_user_command() -> UserCommand {
        UserCommand {
            entity: Entity::PLACEHOLDER,
            full_command: String::from("apollo"),
            keyword: String::from("apollo"),
            parts: vec![String::from("apollo")],
            raw_command: String::from("apollo\n"),
        }
    }

    #[test]
    fn user_must_have_valid_session() {
        let (app, command, user_command) = get_context();

        assert_eq!(false, command.can_execute(&user_command, &app.world));
    }

    #[test]
    fn user_must_be_creating_a_character() {
        let (mut app, command, mut user_command) = get_context();

        let created_entity = app.world.spawn(UserSessionData {
            status: UserStatus::CreateCharacter,
            char_to_delete: None,
            controlling_entity: None,
            username: String::from("boots"),
            connection: Uuid::new_v4(),
            pwd: None,
        });

        user_command.entity = created_entity.id();
        verify_account_command_runs_on(
            &command,
            UserStatus::CreateCharacter,
            &user_command,
            &mut app.world,
        );
    }

    #[test]
    fn cant_have_provided_more_than_one_letter() {
        todo!("Complete me!");
    }

    #[test]
    fn name_must_be_alphabetic() {
        todo!("Complete me!");
    }

    #[test]
    fn character_doesnt_exist() {
        todo!("Complete me!");
    }

    #[test]
    fn sends_generic_error_on_db_issue_checking_name() {
        todo!("Complete me!");
    }

    #[test]
    fn sends_character_exists_event_if_exists() {
        todo!("Complete me!");
    }

    #[test]
    fn sends_generic_error_on_db_issue_creating_user() {
        todo!("Complete me!");
    }

    #[test]
    fn sends_character_created_event_on_success() {
        todo!("Complete me!");
    }

    #[test]
    fn returns_the_user_to_logged_in_on_success() {
        todo!("Complete me!");
    }

    #[test]
    fn sends_show_login_screen_on_success() {
        todo!("Complete me!");
    }
}
