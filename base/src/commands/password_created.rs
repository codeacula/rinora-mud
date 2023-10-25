use shared::prelude::*;

pub struct PasswordCreated {}

impl GameCommand for PasswordCreated {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>, EventWriter<TextEvent>)> =
            SystemState::new(world);
        let (mut query, mut text_event_tx) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        let password = command.full_command.clone();

        user_sesh.pwd = Some(password);
        user_sesh.status = UserStatus::ConfirmPassword;

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "Excellent. Now, provide your password again for confirmation.",
        ));
        Ok(true)
    }
}
