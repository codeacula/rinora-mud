use database::prelude::*;
use shared::prelude::*;

/// Sets a name for the character the user is currently creating
///
/// ### Run Conditions
///
/// * Must have a user session
/// * Must be creating a character
///
/// ### Run Events
///
/// * `CharacterNameInvalidEvent` - When the user provides an invalid name
/// * `CharacterExistsEvent` - User tried to provide a character name that's taken
/// * `CreateCharacterEvent` - Character creation passed checks and is ready to go
///
pub struct ProvideCreateCharacterNameCommand {}

impl GameCommand for ProvideCreateCharacterNameCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        if command.parts.len() > 1 || !is_alphabetic(&command.keyword) || command.keyword.len() > 15
        {
            world.send_event(CharacterNameInvalidEvent(command.entity));
            return Ok(true);
        }

        let mut system_state: SystemState<(Res<DbInterface>,)> = SystemState::new(world);
        let db_repo = system_state.get_mut(world).0;

        let character_name = command.keyword.clone();
        let character_exists = db_repo.characters.does_character_exist(&character_name)?;

        if character_exists {
            world.send_event(CharacterExistsEvent(command.entity));
            return Ok(true);
        }

        world.send_event(CreateCharacterEvent {
            name: character_name,
            user_entity: command.entity,
        });

        Ok(true)
    }
}

#[cfg(dbtest)]
mod tests {
    use database::get_test_db_interface;
    use shared::prelude::*;

    use super::ProvideCreateCharacterNameCommand;

    #[test]
    fn user_must_have_valid_session() {
        let mut app = build_test_app();
        let command = ProvideCreateCharacterNameCommand {};
        let user_command = build_user_command(String::from("password"), Entity::PLACEHOLDER);

        assert_eq!(false, command.run(&user_command, &mut app.world).unwrap());
    }

    #[test]
    fn cant_have_provided_more_than_one_letter() {
        let mut app = build_test_app();
        app.insert_resource(get_test_db_interface());
        let command = ProvideCreateCharacterNameCommand {};
        let user_command = build_user_command(String::from("password"), Entity::PLACEHOLDER);

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterNameInvalidEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }
    /*
    #[test]
    fn name_must_be_alphabetic() {
        let mut app = get_app();
        let command = get_command();

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(String::from("3235sgndas42s"));
        user_command.entity = created_entity;

        let mut app = build_test_app();
        let user_command = build_user_command(String::from("password"), Entity::PLACEHOLDER);

        let res = ProvideCharacterNameCommand {}.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterNameInvalidEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }

    #[test]
    fn character_already_exists() {
        let mut app = get_app();
        let command = get_command();
        let character_name = String::from("Kang");

        let db_interface = get_test_db_interface();
        create_test_character(&db_interface, character_name.clone());

        app.world.insert_resource(db_interface);

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(character_name);
        user_command.entity = created_entity;

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CharacterExistsEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }

    #[test]
    fn sends_character_created_event_on_success() {
        let mut app = get_app();
        let command = get_command();
        let character_name = String::from("Kang");

        let db_interface = get_test_db_interface();

        app.world.insert_resource(db_interface);

        let created_entity = spawn_entity(&mut app.world);

        let mut user_command = get_user_command(character_name);
        user_command.entity = created_entity;

        let res = command.run(&user_command, &mut app.world);

        let evs = app.world.resource::<Events<CreateCharacterEvent>>();

        assert!(res.is_ok());
        assert_eq!(1, evs.len());
    }
    */
}
