use shared::prelude::*;

pub fn send_room_description(
    target: Entity,
    name: String,
    description: String,
    text_event_tx: &mut EventWriter<TextEvent>,
) {
    let mut text_event = TextEvent {
        entity: target,
        text: TextBlock {
            text_slices: Vec::new(),
        },
        add_newline: true,
    };

    text_event.text.text_slices.push(TextSlice {
        foreground: 94,
        text: name.clone() + "\n",
        ..Default::default()
    });

    text_event.text.text_slices.push(TextSlice {
        foreground: 7,
        text: description.clone() + "\n",
        ..Default::default()
    });

    text_event_tx.send(text_event);
}
