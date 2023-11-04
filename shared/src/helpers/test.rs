use crate::prelude::*;

pub fn build_test_app() -> App {
    let mut app = App::new();
    app.add_event::<GenericErrorEvent>();
    app.update();

    app
}

pub fn build_user_command(command: String) -> UserCommand {
    let full_cmd = command.clone();

    UserCommand {
        entity: Entity::PLACEHOLDER,
        full_command: command.clone(),
        keyword: command.clone(),
        parts: command.split(' ').map(|f| f.to_string()).collect(),
        raw_command: format!("{full_cmd}\n"),
    }
}

pub fn build_entity(world: &mut World) -> Entity {
    world
        .spawn(UserSessionData {
            status: UserStatus::CreateCharacter,
            char_to_delete: None,
            controlling_entity: None,
            username: String::from("boots"),
            connection: Uuid::new_v4(),
            pwd: None,
        })
        .id()
}
