use shared::prelude::*;

use crate::events::CharacterNameInvalid;

pub fn character_name_invalid(
    mut character_name_invalid_rx: EventReader<CharacterNameInvalid>,
    mut text_event_writer_tx: EventWriter<TextEvent>,
    mut show_prompt_writer_tx: EventWriter<ShowPrompt>,
) {
    for ev in character_name_invalid_rx.iter() {
        text_event_writer_tx.send(TextEvent::from_str(
            ev.0,
            "Character names can only contain the letters A-Z, and only one word. Please try again.",
        ));
        show_prompt_writer_tx.send(ShowPrompt(ev.0));
    }
}

#[cfg(test)]
mod tests {
    use crate::events::CharacterNameInvalid;

    use super::character_name_invalid;
    use shared::prelude::*;

    fn get_app() -> App {
        let mut app = App::new();
        app.add_event::<CharacterNameInvalid>()
            .add_event::<TextEvent>()
            .add_event::<ShowPrompt>()
            .add_systems(Update, character_name_invalid);

        app.world
            .send_event(CharacterNameInvalid(Entity::PLACEHOLDER));

        app.update();

        app
    }

    #[test]
    fn sends_a_text_event() {
        let app = get_app();

        let text_event_reader = app.world.resource::<Events<TextEvent>>();

        assert_eq!(1, text_event_reader.len());
    }

    #[test]
    fn sends_a_prompt_event() {
        let app = get_app();

        let show_prompt_reader = app.world.resource::<Events<ShowPrompt>>();

        assert_eq!(1, show_prompt_reader.len());
    }
}
