use shared::prelude::*;

pub struct CreateCharacterCommand {}

impl GameCommand for CreateCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        if command.keyword != "1" {
            return Ok(false);
        }

        world.send_event(CreateCharacterSelectedEvent(command.entity));
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use shared::prelude::test::*;
    use shared::prelude::*;

    use crate::commands::prelude::CreateCharacterCommand;

    #[test]
    fn skips_whem_cmd_isnt_1() {
        let mut app = build_test_app();

        app.add_event::<CreateCharacterSelectedEvent>();

        let command = build_user_command(String::from("2"), Entity::PLACEHOLDER);

        let result = CreateCharacterCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn works() {
        let mut app = build_test_app();

        app.add_event::<CreateCharacterSelectedEvent>();

        let command = build_user_command(String::from("1"), Entity::PLACEHOLDER);

        let result = CreateCharacterCommand {}.run(&command, &mut app.world);
        assert_eq!(result, Ok(true));

        let evs = app.world.resource::<Events<CreateCharacterSelectedEvent>>();
        assert_eq!(evs.len(), 1);
    }
}
