use shared::prelude::*;

pub fn send_room_description(
    target: Entity,
    name: &str,
    description: &str,
    exits: &Exits,
    query: &Query<&Exit>,
    text_event_tx: &mut EventWriter<SendTextToEntityEvent>,
) {
    let mut text_to_send = String::new();

    let mut display_name = name.to_owned();

    if let Some(last_char) = display_name.chars().last() {
        if !last_char.is_ascii_punctuation() {
            display_name.push('.');
        }
    }

    text_to_send.push_str(format!("<<94>>{}\n", display_name).as_str());
    text_to_send.push_str(format!("<<7>>{}\n", description).as_str());

    let exit_phrase = if exits.0.len() == 1 {
        "an exit"
    } else {
        "exits"
    };

    text_to_send.push_str(format!("<<23>>You see {} leading", exit_phrase).as_str());

    for (index, value) in exits.0.iter().enumerate() {
        if index == 0 {
            text_to_send.push(' ');
        } else if index == exits.0.len() - 1 {
            text_to_send.push_str("<<23>> and ");
        } else {
            text_to_send.push_str("<<23>>, ");
        }

        let exit = query.get(*value).expect("Unable to find exit");
        text_to_send.push_str(format!("<<14>>{}", get_long_direction(&exit.direction)).as_str());
    }

    text_to_send.push_str("<<23>>.\n");

    let text_event = SendTextToEntityEvent {
        entity: target,
        text: text_to_send,
    };

    text_event_tx.send(text_event);
}

/*
pub fn send_room_gmcp(
    send_gmcp_data_tx: &mut EventWriter<SendGmcpData>,
    controller: &IsControlledBy,
    room_id: i32,
    room_name: &String,
) {
    // Build the gmcp data
    send_gmcp_data_tx.send(SendGmcpData {
        command_name: "Room.Info".to_string(),
        data: format!(r#"{{"num":{},"name":"{}"}}"#, room_id, room_name),
        entity: controller.0,
    });
}
*/
