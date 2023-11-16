use shared::prelude::*;

pub fn send_room_description(
    target: Entity,
    name: &String,
    description: &String,
    exits: &Exits,
    query: &Query<&Exit>,
    text_event_tx: &mut EventWriter<TextEvent>,
) {
    let mut text_event = TextEvent {
        entity: target,
        text: TextBlock {
            text_slices: Vec::new(),
        },
        add_newline: true,
    };

    let mut display_name = name.clone();

    if let Some(last_char) = display_name.chars().last() {
        if !last_char.is_ascii_punctuation() {
            display_name.push('.');
        }
    }

    text_event.text.text_slices.push(TextSlice {
        foreground: 94,
        text: display_name.clone() + "\n",
        ..Default::default()
    });

    text_event.text.text_slices.push(TextSlice {
        foreground: 7,
        text: description.clone() + "\n",
        ..Default::default()
    });

    let exit_phrase = if exits.0.len() == 1 {
        "an exit"
    } else {
        "exits"
    };

    text_event.text.text_slices.push(TextSlice {
        foreground: 23,
        text: format!("You see {} leading", exit_phrase),
        ..Default::default()
    });

    for (index, value) in exits.0.iter().enumerate() {
        if index == 0 {
            text_event.text.text_slices.push(TextSlice {
                foreground: 23,
                text: " ".to_string(),
                ..Default::default()
            });
        } else if index == exits.0.len() - 1 {
            text_event.text.text_slices.push(TextSlice {
                foreground: 23,
                text: " and ".to_string(),
                ..Default::default()
            });
        } else {
            text_event.text.text_slices.push(TextSlice {
                foreground: 23,
                text: ", ".to_string(),
                ..Default::default()
            });
        }

        let exit = query.get(*value).expect("Unable to find exit");
        text_event.text.text_slices.push(TextSlice {
            foreground: 14,
            text: get_long_direction(&exit.direction),
            ..Default::default()
        });
    }

    text_event.text.text_slices.push(TextSlice {
        foreground: 23,
        text: ".".to_string(),
        ..Default::default()
    });

    // You see exits leading.
    text_event_tx.send(text_event);
}
