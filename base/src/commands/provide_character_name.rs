use bevy::{ecs::system::SystemState, prelude::*};
use database::prelude::*;
use shared::prelude::*;

/// Sets a name for the character the user is currently creating
///
/// ### Run Conditions
///
/// * Must have a user session
/// * Must be creating a character
pub struct ProvideCharacterNameCommand {}

impl GameCommand for ProvideCharacterNameCommand {
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
            world.send_event(CharacterNameInvalidEvent(command.entity));
            return Ok(());
        }

        let mut system_state: SystemState<(Res<DbInterface>,)> = SystemState::new(world);
        let db_repo = system_state.get_mut(world).0;

        let character_name = command.keyword.clone();
        let character_exists = db_repo.characters.does_character_exist(&character_name)?;

        if character_exists {
            world.send_event(CharacterExistsEvent(command.entity));
            return Ok(());
        }

        world.send_event(CreateCharacterEvent {
            name: character_name,
            user_entity: command.entity,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use shared::prelude::*;

    use super::ProvideCharacterNameCommand;

    fn get_app() -> App {
        let mut app = App::new();
        app.add_event::<CharacterNameInvalidEvent>()
            .add_event::<CharacterExistsEvent>();
        app.update();

        app
    }

    fn get_command() -> Box<dyn GameCommand> {
        Box::new(ProvideCharacterNameCommand {})
    }

    fn get_context() -> (App, Box<dyn GameCommand>, UserCommand) {
        return (
            get_app(),
            get_command(),
            get_user_command(String::from("Apollo")),
        );
    }

    fn get_user_command(command: String) -> UserCommand {
        let full_cmd = command.clone();

        UserCommand {
            entity: Entity::PLACEHOLDER,
            full_command: command.clone(),
            keyword: command.clone(),
            parts: command.split(' ').map(|f| f.to_string()).collect(),
            raw_command: format!("{full_cmd}\n"),
        }
    }

    fn spawn_entity(world: &mut World) -> Entity {
        world
            .spawn(UserSessionData {
                status: UserStatus::CreateCharacter,
                char_to_delete: None,
                controlling_entity: None,
                username: String::from("boots"),
                connection: Uuid::new_v4(),
                pwd: None,
            })
            .id()
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
        let (mut app, command, mut _user_command) = get_context();

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(String::from("Big Beans"));
        user_command.entity = created_entity;

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterNameInvalidEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }

    #[test]
    fn name_must_be_alphabetic() {
        let (mut app, command, _user_command) = get_context();

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(String::from("3235sgndas42s"));
        user_command.entity = created_entity;

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterNameInvalidEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }

    #[test]
    fn character_already_exists() {
        let (mut app, command, _user_command) = get_context();

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(String::from("3235sgndas42s"));
        user_command.entity = created_entity;

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterExistsEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }

    #[test]
    fn sends_character_created_event_on_success() {
        todo!("Complete me!");
    }
}
