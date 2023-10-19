use bevy::{ecs::system::SystemState, prelude::*};
use shared::prelude::*;

pub struct SelectedCreateCharacter {}

impl GameCommand for SelectedCreateCharacter {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            return false;
        }

        command.full_command == "1"
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let mut system_state: SystemState<(Query<&mut UserSessionData>, EventWriter<TextEvent>)> =
            SystemState::new(world);
        let (mut query, mut text_event_tx) = system_state.get_mut(world);
        let mut user_sesh = query.get_mut(command.entity).unwrap();

        text_event_tx.send(TextEvent::from_str(
            command.entity,
            "{{11}}What would you like your character's name to be?",
        ));

        user_sesh.status = UserStatus::CreateCharacter;

        Ok(())
    }
}
