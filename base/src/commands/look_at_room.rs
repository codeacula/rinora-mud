use shared::prelude::*;

use crate::events::ShowRoomToBeing;

pub struct LookAtRoomCommand {}

impl GameCommand for LookAtRoomCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let cleaned_keyword = command.keyword.to_lowercase();

        if cleaned_keyword != "l" && cleaned_keyword != "look" {
            return Ok(false);
        }

        let default_value = &"here".to_string();
        let target = command.parts.get(1).unwrap_or(default_value);

        if target != "here" {
            return Ok(false);
        }

        let user_sesh =
            if let Some(user_sesh) = world.entity(command.entity).get::<UserSessionData>() {
                user_sesh
            } else {
                return Err("Could not find user session data".to_string());
            };

        let controlling_entity = user_sesh.entity_they_are_controlling.unwrap();
        let room_entity = world.entity_mut(controlling_entity);
        let room = room_entity.get::<Location>().unwrap().entity;

        world.send_event(ShowRoomToBeing {
            entity: controlling_entity,
            room,
        });
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::events::ShowRoomToBeing;

    use super::*;

    fn build_entity_and_get_command(app: &mut App, string_command: &str) -> UserCommand {
        let mut room_buider = EntityBuilder::new();
        let room_entity = room_buider.build(&mut app.world);

        let mut character_builder = EntityBuilder::new();
        character_builder.set_location(Location {
            location_id: 1,
            entity: room_entity,
        });
        let character_entity = character_builder.build(&mut app.world);

        let mut entity_builder = EntityBuilder::new();
        let mut user_sesh = UserSessionData::new();
        user_sesh.entity_they_are_controlling = Some(character_entity);
        entity_builder.set_session_data(user_sesh);

        let command = build_user_command(
            string_command.to_string(),
            entity_builder.build(&mut app.world),
        );

        command
    }

    #[test]
    fn test_look() {
        let mut app = build_test_app();
        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "look\r\n");

        let result = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| is_valid));
    }

    #[test]
    fn test_look_here_works() {
        let mut app = build_test_app();
        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "look here\r\n");

        let result = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| is_valid));
    }

    #[test]
    fn another_command_fails() {
        let mut app = build_test_app();
        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "get piggy\r\n");

        let result: Result<bool, String> = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| !is_valid));
    }

    #[test]
    fn fails_while_trying_to_look_at() {
        let mut app = build_test_app();
        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "look micharoj\r\n");

        let result: Result<bool, String> = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| !is_valid));
    }

    #[test]
    fn works_with_uppercase() {
        let mut app = build_test_app();
        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "LOOK\r\n");

        let result: Result<bool, String> = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| is_valid));
    }

    #[test]
    fn issues_the_right_event() {
        let mut app = build_test_app();
        app.add_event::<ShowRoomToBeing>();

        let command = LookAtRoomCommand {};
        let user_command = build_entity_and_get_command(&mut app, "LOOK\r\n");

        let result: Result<bool, String> = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| is_valid));

        let events = app.world.resource::<Events<ShowRoomToBeing>>();
        assert!(events.len() == 1);
    }

    #[test]
    fn sets_the_room_correctly() {
        let mut app = build_test_app();
        app.add_event::<ShowRoomToBeing>();

        let user_command = build_entity_and_get_command(&mut app, "look");

        let command = LookAtRoomCommand {};
        let result: Result<bool, String> = command.run(&user_command, &mut app.world);
        assert!(result.is_ok_and(|is_valid| is_valid));

        let events = app.world.resource::<Events<ShowRoomToBeing>>();
        let (sent_event, _) = events.get_event(events.oldest_id()).unwrap();
        assert!(sent_event.room != Entity::PLACEHOLDER);
    }
}
