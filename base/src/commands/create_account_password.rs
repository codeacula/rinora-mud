use shared::prelude::*;

pub struct CreateAccountPasswordCommand {}

impl GameCommand for CreateAccountPasswordCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let password = command.full_command.clone();

        if password.len() < 3 {
            world.send_event(PasswordNotLongEnoughEvent(command.entity));
            return Ok(true);
        }

        world.send_event(UserProvidedPasswordEvent {
            user_entity: command.entity,
            password,
        });

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use shared::prelude::test::*;
    use shared::prelude::*;

    use crate::commands::prelude::CreateAccountPasswordCommand;

    #[test]
    fn password_isnt_long_enough() {
        let mut app = build_test_app();

        app.add_event::<PasswordNotLongEnoughEvent>();

        let command = build_user_command(String::from(""), Entity::PLACEHOLDER);

        let result = CreateAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<PasswordNotLongEnoughEvent>>();
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn works() {
        let mut app = build_test_app();

        app.add_event::<UserProvidedPasswordEvent>();

        let command = build_user_command(String::from("1234"), Entity::PLACEHOLDER);

        let result = CreateAccountPasswordCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<UserProvidedPasswordEvent>>();
        assert_eq!(evs.len(), 1);
    }
}
