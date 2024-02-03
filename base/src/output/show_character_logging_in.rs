use shared::prelude::*;

use crate::helpers::send_room_descriptions::send_room_description;

pub(crate) fn show_character_logging_in(
    mut character_logged_in_rx: EventReader<CharacterLoggedInEvent>,
    mut send_text_tx: EventWriter<SendTextToEntityEvent>,
    mut text_event_tx: EventWriter<TextEvent>,
    character_query: Query<(&DisplayName, &Location), With<Character>>,
    mut room_query: Query<(
        &mut EntityCollection,
        &Room,
        &DisplayName,
        &Description,
        &Exits,
    )>,
    area_query: Query<&Area>,
    continent_query: Query<&Continent>,
    plane_query: Query<&DisplayName, With<Plane>>,
    exit_query: Query<&Exit>,
) {
    for CharacterLoggedInEvent(character_entity) in character_logged_in_rx.read() {
        let entity = *character_entity;

        let (display_name, location) = match character_query.get(entity) {
            Ok(data) => data,
            Err(_) => continue,
        };

        let (mut entity_collection, room, room_display_name, room_description, exits) =
            match room_query.get_mut(location.entity) {
                Ok(room) => room,
                Err(_) => continue,
            };

        // Add the player to the room's entity_collection
        entity_collection.0.push(entity);

        let mut plane_name = "Unknown";

        if area_query.contains(room.area) {
            let area = area_query.get(room.area).unwrap();

            if continent_query.contains(area.continent) {
                let continent = continent_query.get(area.continent).unwrap();

                if plane_query.contains(continent.plane) {
                    let display_name = plane_query.get(continent.plane).unwrap();
                    plane_name = &display_name.0;
                }
            }
        }

        send_text_tx.send(SendTextToEntityEvent::new(
            entity,
            &format!("<<15>>The light from the Heart of Ero'ghal fills your vision with white momentarily, before you are wrenched away in an instant. You suddenly take a deep breath and open your eyes, safe upon {plane_name}."),
        ));

        // Show the character logging in to everyone else
        for entity_in_room in entity_collection.0.iter() {
            if entity_in_room == &entity {
                continue;
            }

            send_text_tx.send(SendTextToEntityEvent::new(
                *entity_in_room,
                &format!("<<15>>{} appears in a flash of light.", display_name.0),
            ));
        }

        send_room_description(
            entity,
            &room_display_name.0,
            &room_description.0,
            exits,
            &exit_query,
            &mut text_event_tx,
        )
    }
}
