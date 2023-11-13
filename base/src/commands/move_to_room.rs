use shared::prelude::*;

pub struct MoveToRoomCommand {}

impl GameCommand for MoveToRoomCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let mut state: SystemState<(
            Query<&Location>,
            Query<&mut UserSessionData>,
            Res<RoomMap>,
            Query<&Exits>,
            Query<&Exit>,
        )> = SystemState::new(world);

        let (location_query, user_query, room_map, exits_query, exit_query) = state.get_mut(world);

        let user_sesh = user_query.get(command.entity).expect("No user found");

        let location = location_query
            .get(user_sesh.controlling_entity.expect("No current character"))
            .expect("No location found");
        let room_num = location.0;

        let room_entity = room_map.0.get(&room_num).expect("Room not found");
        let exits = exits_query.get(*room_entity).expect("No exits found");

        let cleaned_direction = get_short_direction(&command.full_command);

        for exit_entity in exits.0.iter() {
            let exit = exit_query.get(*exit_entity).expect("No exit found");

            if exit.direction == cleaned_direction {
                info!("We matched the direction!");
                return Ok(true);
            }
        }

        Ok(false)
    }
}
