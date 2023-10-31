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
