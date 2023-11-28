use shared::prelude::*;

pub struct LookAtCommand {}

impl GameCommand for LookAtCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_at() {
        let mut world = World::new();
        let command = UserCommand::new("look");
        let command = LookAtCommand {};
        let result = command.run(&command, &mut world);
        assert!(result.is_ok());
    }
}
