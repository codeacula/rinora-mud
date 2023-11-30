use std::sync::mpsc::*;

use crate::prelude::*;
use bevy::prelude::*;

pub fn build_test_app() -> App {
    let mut app = App::new();
    app.add_event::<GenericErrorEvent>();

    app
}

pub fn build_user_command(command: String, entity: Entity) -> UserCommand {
    let raw = command.clone().replace(|c: char| !c.is_ascii(), "");
    let full_command = raw.trim().to_string();
    let parts: Vec<String> = full_command.split(' ').map(|f| f.to_string()).collect();
    let keyword = parts.get(0).unwrap_or(&"".to_string()).trim().to_string();

    UserCommand {
        entity,
        full_command,
        keyword,
        parts,
        raw_command: command.clone(),
    }
}

pub struct EntityBuilder {
    pub location: Option<Location>,
    pub session_data: Option<UserSessionData>,
}

impl EntityBuilder {
    pub fn build(&mut self, world: &mut World) -> Entity {
        let mut new_entity = world.spawn_empty();

        if let Some(user_sesh) = self.session_data.clone() {
            self.session_data = None;
            new_entity.insert(user_sesh);
        }

        if let Some(location) = self.location {
            self.location = None;
            new_entity.insert(location);
        }

        new_entity.id()
    }

    pub fn new() -> Self {
        Self {
            location: None,
            session_data: None,
        }
    }

    pub fn set_location(&mut self, location: Location) -> &mut Self {
        self.location = Some(location);

        self
    }

    pub fn set_session_data(&mut self, session_data: UserSessionData) -> &mut Self {
        self.session_data = Some(session_data);
        self
    }
}

impl Default for EntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_channels<T>() -> (Sender<T>, Receiver<T>) {
    channel::<T>()
}
