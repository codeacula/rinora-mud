use shared::prelude::*;

pub fn move_entity_to_room(
    move_entity_to_room_rx: EventReader<MoveEntityToRoom>,
    being_query: Query<(&Being, Option<&Location>)>,
) {
    // Check if they entered the world for the first time
    // Check if they're moving planes
    // Check if they're moving continents
    // Check if they're moving areas
    // Check if they're moving rooms
}

#[cfg(test)]
mod tests {
    use crate::commands::prelude::*;

    use super::*;

    fn get_app_and_run() -> App {
        let mut app = build_test_app();

        app.add_systems(Update, move_entity_to_room);

        app.run();

        app
    }

    // Check if they entered the world for the first time
    #[test]
    fn entity_entered_world_for_first_time() {
        let mut app = get_app_and_run();

        let mut entity_builder = EntityBuilder::new();
        let entity = entity_builder.build(&mut app.world);

        app.world.send_event(MoveEntityToRoom {
            entity,
            room: Entity::PLACEHOLDER,
            triggered_by: MovementTriggeredBy::UserInput,
        });

        app.run();

        let events = app.world.resource::<Events<EntityEnteredWorldEvent>>();

        assert_eq!(events.len(), 1);
    }

    #[test]
    fn entity_was_already_in_world() {}

    // Check if they're moving planes
    #[test]
    fn entity_is_moving_to_new_plane() {}

    #[test]
    fn entity_is_staying_in_same_plane() {}

    // Check if they're moving continents
    #[test]
    fn entity_is_moving_to_new_continent() {}

    #[test]
    fn entity_is_staying_in_same_continent() {}

    // Check if they're moving areas
    #[test]
    fn entity_is_moving_to_new_area() {}

    #[test]
    fn entity_is_staying_in_same_area() {}

    // Check if they're moving rooms
    #[test]
    fn entity_is_moving_to_new_room() {}
}
