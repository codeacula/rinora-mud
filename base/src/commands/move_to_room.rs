use shared::prelude::*;

pub struct MoveToRoomCommand {}

impl GameCommand for MoveToRoomCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut state: SystemState<(
            Query<&Location>,
            Query<&mut UserSessionData>,
            Res<RoomMap>,
            Query<&Exits>,
            Query<(Entity, &Exit)>,
        )> = SystemState::new(world);

        let (location_query, user_query, room_map, exits_query, exit_query) = state.get_mut(world);

        let user_sesh = user_query.get(command.entity).expect("No user found");

        let location = location_query
            .get(user_sesh.controlling_entity.expect("No current character"))
            .expect("No location found");
        let room_num = location.location_id;

        let room_entity = room_map.0.get(&room_num).expect("Room not found");
        let exits = exits_query.get(*room_entity).expect("No exits found");

        let cleaned_direction = get_short_direction(&command.full_command);

        let mut selected_exit: Option<&Exit> = None;
        let mut selected_exit_entity: Option<Entity> = None;

        for exit_entity in exits.0.iter() {
            let (found_exit_entity, exit) = exit_query.get(*exit_entity).expect("No exit found");

            if exit.direction == cleaned_direction {
                selected_exit = Some(exit);
                selected_exit_entity = Some(found_exit_entity);
                break;
            }
        }

        // If it's a valid direction but we have no exit, we want to tell them so
        if selected_exit.is_none() && is_valid_direction(&cleaned_direction) {
            world.send_event(InvalidDirectionEvent(command.entity));
            return Ok(true);
        }

        if selected_exit.is_none() {
            return Ok(false);
        }

        // Let's get who they're controlling and add the tag that they're trying to move
        let current_character = user_sesh.controlling_entity.expect("No current character");

        // Get the destination location to add to the EntityWantsToMove event

        world
            .entity_mut(current_character)
            .insert(EntityWantsToMove {
                exit_entity: selected_exit_entity.expect("No exit entity"),
                who_entity: current_character,
            });

        Ok(true)
    }
}
