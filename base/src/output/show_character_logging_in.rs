use shared::prelude::*;

pub(crate) fn show_character_logging_in(
    mut character_logged_in_rx: EventReader<CharacterLoggedInEvent>,
    mut send_text_tx: EventWriter<SendTextToEntityEvent>,
    character_query: Query<(&DisplayName, &Location), With<Character>>,
    room_query: Query<(&EntityCollection, &Room)>,
    area_query: Query<&Area>,
    continent_query: Query<&Continent>,
    plane_query: Query<&DisplayName, With<Plane>>,
    room_map: Res<RoomMap>,
) {
    for CharacterLoggedInEvent(character_entity) in character_logged_in_rx.read() {
        let entity = *character_entity;

        let (display_name, location) = match character_query.get(entity) {
            Ok(data) => data,
            Err(_) => continue,
        };

        let (entity_collection, room) = match room_query.get(location.entity) {
            Ok(room) => room,
            Err(_) => continue,
        };

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
            &format!("<<15>>The light from the Heart of Ero'ghal fills your vision with white momentarily, before you are wrenched away in an instant. You suddenly take a deep breath and open your eyes, safe upon the {plane_name}."),
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
    }
}
