use database::prelude::*;
use shared::prelude::*;

pub struct ProvideAccountPasswordCommand {}

impl GameCommand for ProvideAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut query = world.query::<&UserSessionData>();

        let Ok(user_sesh) = query.get(world, command.entity) else {
            return Ok(false);
        };

        let provided_password = command.full_command.clone();
        let db_repo = world.get_resource::<DbInterface>().unwrap();

        let Some(user) = db_repo
            .users
            .find_with_credentials(&user_sesh.username, &provided_password)?
        else {
            world.send_event(UnableToLocateAccountEvent(command.entity));
            return Ok(true);
        };

        world.send_event(UserLoggedInEvent {
            entity: command.entity,
            id: user.id,
            password: provided_password,
        });

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use database::get_test_db_interface;
    use shared::prelude::*;

    use crate::commands::prelude::ProvideAccountPasswordCommand;

    #[test]
    fn doesnt_run_if_no_user_sesh() {
        let mut app = build_test_app();
        let command = build_user_command(String::from("password"), Entity::PLACEHOLDER);

        let result = ProvideAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn account_cant_be_located() {
        let mut app = build_test_app();

        app.add_event::<UnableToLocateAccountEvent>();

        let db_handle = get_test_db_interface();

        db_handle
            .users
            .create_user("test", "test")
            .expect("Could not create user");

        app.insert_resource(db_handle);

        let mut entity_builder = EntityBuilder::new();

        let mut user_sesh = UserSessionData::new();
        user_sesh.username = String::from("test2");
        entity_builder.set_session_data(user_sesh);

        let command =
            build_user_command(String::from("test"), entity_builder.build(&mut app.world));

        let result: Result<bool, String> =
            ProvideAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<UnableToLocateAccountEvent>>();
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn works() {
        let mut app = build_test_app();

        app.add_event::<UserLoggedInEvent>();

        let db_handle = get_test_db_interface();
        db_handle
            .users
            .create_user("test2", "test2")
            .expect("Could not create user");

        app.insert_resource(db_handle);

        let mut entity_builder = EntityBuilder::new();

        let mut user_sesh = UserSessionData::new();
        user_sesh.username = String::from("test2");
        entity_builder.set_session_data(user_sesh);

        let command =
            build_user_command(String::from("test2"), entity_builder.build(&mut app.world));

        let result: Result<bool, String> =
            ProvideAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<UserLoggedInEvent>>();
        assert_eq!(evs.len(), 1);
    }
}
