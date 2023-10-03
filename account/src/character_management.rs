use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub struct CreateCharacter {}

impl GameCommand for CreateCharacter {
    fn can_execute(&self, command: &UserCommand, world: &World) -> bool {
        let Some(user_session) = world.get::<UserSessionData>(command.entity) else {
            return false;
        };

        if user_session.status != UserStatus::LoggedIn {
            return false;
        }

        if command.full_command != "1" {
            return false;
        }

        return true;
    }

    fn run(&self, command: &UserCommand, world: &mut World) -> Result<(), String> {
        let (entity, user, user_sesh) = world
            .query::<(Entity, &User, &UserSessionData)>()
            .get_mut(world, command.entity)
            .unwrap()
            .to_owned();

        if command.parts.len() > 1 || !is_alphabetic(&command.keyword) {
            world.send_event(TextEvent::from_str(
                entity,
                "Character names can only contain the letters A-Z, and only one word. Please try again.",
            ));
            return Ok(());
        }
        let character_name = command.keyword.clone();

        let db_repo = world.resource::<DbInterface>();

        let exists_res = db_repo.characters.does_character_exist(&character_name);

        if let Err(err) = exists_res {
            error!("Error checking if character exists: {:?}", err);
            world.send_event(TextEvent::send_generic_error(entity));
            return Ok(());
        }

        if exists_res.unwrap() {
            world.send_event(TextEvent::from_str(
                entity,
                "That character already exists. Please try a different name.",
            ));
            return Ok(());
        }

        let settings = world.resource::<GameSettings>();

        if let Err(err) =
            db_repo
                .characters
                .create_character(&character_name, settings.default_room, user)
        {
            error!("Error creating character for user: {:?}", err);
            world.send_event(TextEvent::send_generic_error(entity));
            return Ok(());
        }

        world.send_event(TextEvent::from_str(
            entity,
            "Character created! You can now select them from the login screen",
        ));

        user_sesh.status = UserStatus::LoggedIn;

        let characters = match db_repo.characters.get_all_by_user(user.id) {
            Ok(characters) => characters,
            Err(e) => {
                error!(
                    "Unable to fetch user's characters after creating a character: {:?}",
                    e
                );
                world.send_event(TextEvent::send_generic_error(entity));
                return Ok(());
            }
        };

        world.send_event(TextEvent::new(
            entity,
            &crate::get_login_screen(&characters),
        ));
        Ok(())
    }
}
