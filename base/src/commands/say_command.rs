use shared::prelude::*;

pub struct SayCommand;

impl GameCommand for SayCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        if command.keyword.to_lowercase() != "say" {
            return Ok(false);
        }

        // This is who is speaking
        let mut speaking_entity = command.entity;

        // This lets us know where we should start to concatenate from. By default we want 1, for "say Hello!"
        let mut amount_to_skip = 0;

        // The event we'll end up sending. Right now we'll use an empty string and no target
        let mut speak_event = SpeakEvent {
            speaker: command.entity,
            target: None,
            text: String::from(""),
        };

        // Who all will hear the event
        let mut entities_present: Vec<Entity> = Vec::new();

        // If it's a user controlling someone, it should come from whom they're controlling
        if let Some(user_session_data) = world.get::<UserSessionData>(command.entity) {
            if let Some(controlled_entity) = user_session_data.entity_they_are_controlling {
                speaking_entity = controlled_entity;
            }
        }

        if let Some(location) = world.get::<Location>(speaking_entity) {
            if let Some(entity_collection) = world.get::<EntityCollection>(location.entity) {
                let entities_in_room = entity_collection.0.clone();

                for entity_in_room in entities_in_room {
                    entities_present.push(entity_in_room);
                }
            }
        } else {
            error!("Expected speaker to be in a location but wasn't.");
            return Ok(true);
        }

        {
            let is_to_someone = command
                .parts
                .first()
                .is_some_and(|x| x.to_lowercase() == "to");

            let who_to_option = command.parts.get(1);

            if is_to_someone && who_to_option.is_some() {
                let who_to = who_to_option.unwrap();
                for possible_target in entities_present {
                    let display_name_option = world.get::<DisplayName>(possible_target);
                    if possible_target != speak_event.speaker
                        && display_name_option.is_some()
                        && display_name_option.unwrap().0.to_lowercase() == *who_to.to_lowercase()
                    {
                        speak_event.target = Some(possible_target);
                        amount_to_skip = 2;
                    }
                }
            }
        }

        let mut text_parts: Vec<String> =
            command.parts.iter().skip(amount_to_skip).cloned().collect();

        // Make sure the first letter is capitalized
        if let Some(first_part) = text_parts.first() {
            let mut chars: Vec<char> = first_part.chars().collect();
            chars[0] = chars[0].to_uppercase().nth(0).unwrap();
            text_parts[0] = chars.iter().collect();
        }

        // Make sure it ends in punctuation
        if let Some(last_part) = text_parts.last() {
            if let Some(last_char) = last_part.chars().last() {
                if !last_char.is_ascii_punctuation() {
                    text_parts.last_mut().unwrap().push('.');
                }
            }
        }

        speak_event.text = text_parts.join(" ");

        info!("Event text: {:?}", speak_event.text);

        world.send_event(speak_event);

        Ok(true)
    }
}
