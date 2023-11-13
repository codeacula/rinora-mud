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

    text_event.text.text_slices.push(TextSlice {
        foreground: 117,
        text: name.clone() + "\n",
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

    let mut exit_string = String::from(format!("You see {} leading", exit_phrase));

    for (index, value) in exits.0.iter().enumerate() {
        if index == 0 {
            exit_string.push_str(" ");
        } else if index == exits.0.len() - 1 {
            exit_string.push_str(" and ");
        } else {
            exit_string.push_str(", ");
        }

        let exit = query.get(*value).expect("Unable to find exit");

        exit_string.push_str(get_long_direction(&exit.direction).as_str());
    }
    exit_string.push_str(".");

    text_event.text.text_slices.push(TextSlice {
        foreground: 23,
        text: exit_string + "\n",
        ..Default::default()
    });

    // You see exits leading.
    text_event_tx.send(text_event);
}
