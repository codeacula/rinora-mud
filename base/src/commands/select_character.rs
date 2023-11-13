use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

/// This command allows a user to select a character to log in to
///
/// ### Run Conditions
///
/// * Must have a user session
/// * Must be logged in
/// * Must own the character
///
/// ### Run Events
///
/// * `CharacterNotFoundEvent` - Unable to locate the character. Shouldn't ever get here
/// * `GenericErrorEvent` - When the character's room isn't in the room map
/// * `CreateCharacterEvent` - Character creation passed checks and is ready to go
///
pub struct SelectCharacterCommand {}

impl GameCommand for SelectCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let Some(user) = world.get::<User>(command.entity) else {
            warn!("Couldn't find user entity");
            return Ok(false);
        };

        let db_repo = world.resource::<DbInterface>();

        info!("Command: {:?}", command);

        let does_own = db_repo
            .characters
            .does_user_own_character(&command.keyword, user.id);

        if does_own {
            world.send_event(CharacterSelectedEvent {
                name: command.keyword.clone(),
                user_entity: command.entity,
            });
        } else {
            world.send_event(CharacterNotFoundEvent(command.entity));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use database::{
        get_test_db_interface,
        prelude::{create_bad_test_character, create_test_character},
    };
    use shared::prelude::*;

    use super::SelectCharacterCommand;

    fn get_app() -> App {
        let mut app = App::new();
        app.add_event::<CharacterNameInvalidEvent>()
            .add_event::<CharacterExistsEvent>()
            .add_event::<CreateCharacterEvent>()
            .add_event::<CharacterSelectedEvent>();
        app.update();

        app
    }

    fn get_command() -> Box<dyn GameCommand> {
        Box::new(SelectCharacterCommand {})
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
                status: UserStatus::LoggedIn,
                char_to_delete: None,
                controlling_entity: None,
                username: String::from("boots"),
                connection: Uuid::new_v4(),
                pwd: None,
            })
            .id()
    }

    fn spawn_user(world: &mut World, entity: Entity) {
        world.entity_mut(entity).insert(User {
            administrator: false,
            current_character: None,
            id: 1,
            username: String::from("testuser"),
        });
    }

    #[test]
    fn user_must_have_valid_session() {
        let mut app = get_app();
        let command = get_command();
        let user_command = get_user_command(String::from("Butts"));

        assert_eq!(false, command.run(&user_command, &mut app.world).unwrap());
    }

    #[test]
    fn user_must_own_character() {
        let mut app: App = get_app();
        let command = get_command();
        let db_interface = get_test_db_interface();

        let entity = spawn_entity(&mut app.world);
        spawn_user(&mut app.world, entity);

        let username = String::from("Billy");
        let mut user_command = get_user_command(username.clone());

        create_bad_test_character(&db_interface, username);

        app.insert_resource(db_interface);
        user_command.entity = entity;

        assert!(!command.run(&user_command, &mut app.world).unwrap());
    }

    #[test]
    fn sends_character_selected_event() {
        let mut app: App = get_app();
        let command = get_command();
        let db_interface = get_test_db_interface();

        let entity = spawn_entity(&mut app.world);
        spawn_user(&mut app.world, entity);

        let username = String::from("Billy");
        let mut user_command = get_user_command(username.clone());

        create_test_character(&db_interface, username);

        app.insert_resource(db_interface);
        user_command.entity = entity;

        command
            .run(&user_command, &mut app.world)
            .expect("Command failed to run.");

        let char_selected_res = app
            .world
            .get_resource::<Events<CharacterSelectedEvent>>()
            .expect("Unable to locate resource.");

        assert_eq!(1, char_selected_res.len());
    }
}
