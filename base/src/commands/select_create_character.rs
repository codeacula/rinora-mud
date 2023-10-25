use shared::prelude::*;

pub struct SelectCreateCharacterCommand {}

impl GameCommand for SelectCreateCharacterCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>, EventWriter<TextEvent>)> =
            SystemState::new(world);
        let (mut query, mut text_event_tx) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return Ok(false);
        };

        if user_session.status != UserStatus::LoggedIn {
            return Ok(false);
        }

        Ok(true)
    }
}
