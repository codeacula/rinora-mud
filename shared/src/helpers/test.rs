use crate::prelude::*;
use bevy::prelude::*;

pub fn build_test_app() -> App {
    let mut app = App::new();
    app.add_event::<GenericErrorEvent>();

    app
}

pub fn build_user_command(command: String, entity: Entity) -> UserCommand {
    let full_cmd = command.clone();

    UserCommand {
        entity,
        full_command: command.clone(),
        keyword: command.clone(),
        parts: command.split(' ').map(|f| f.to_string()).collect(),
        raw_command: format!("{full_cmd}\n"),
    }
}

pub struct EntityBuilder {
    session_data: Option<UserSessionData>,
}

impl EntityBuilder {
    pub fn build(&mut self, world: &mut World) -> Entity {
        let mut new_entity = world.spawn_empty();

        if let Some(user_sesh) = self.session_data.clone() {
            self.session_data = None;
            new_entity.insert(user_sesh);
        }

        new_entity.id()
    }

    pub fn new() -> Self {
        Self { session_data: None }
    }

    pub fn set_session_data(&mut self, session_data: UserSessionData) -> &mut Self {
        self.session_data = Some(session_data);
        self
    }
}
