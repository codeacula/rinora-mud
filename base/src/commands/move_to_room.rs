use shared::prelude::*;

pub struct MoveToRoomCommand {}

const KNOWN_DIRECTIONS: [&str; 12] = [
    "n", "s", "e", "w", "u", "d", "ne", "nw", "se", "nw", "in", "out",
];

const DIRECTION_MAP: [(&str, &str); 12] = [
    ("n", "north"),
    ("s", "south"),
    ("e", "east"),
    ("w", "west"),
    ("u", "up"),
    ("d", "down"),
    ("ne", "northeast"),
    ("nw", "northwest"),
    ("se", "southeast"),
    ("sw", "southwest"),
    ("in", "in"),
    ("out", "out"),
];

fn clean_direction_input(str: &str) -> String {
    if KNOWN_DIRECTIONS.contains(&str) {
        return str.to_string();
    };

    for (short, long) in DIRECTION_MAP.iter() {
        if str == *long {
            return short.to_string();
        }
    }

    str.to_string()
}

impl GameCommand for MoveToRoomCommand {
    fn run(&self, command: &UserCommand, world: &mut World) -> Result<bool, String> {
        let state: SystemState<(Query<&Location>, Query<&User>, Res<RoomMap>, Query<&Exits>)> =
            SystemState::new(world);

        let (location_query, user_query, room_map, exit_query) = state.get_mut(world);

        let direction = clean_direction_input(&command.full_command);

        let user = user_query.get(command.entity).expect("No user found");

        let location = location_query
            .get(user.current_character.expect("No current character"))
            .expect("No location found");
        let room_num = location.0;

        let room_entity = room_map.0.get(&room_num).expect("Room not found");
        let exits = exit_query.get(*room_entity).expect("No exits found");

        for exit in exits.0.iter() {
            if exit.direction == direction {
                let new_room = room_map.0.get(&exit.to).expect("Room not found");
                let new_room = new_room.clone();

                let mut location = location_query
                    .get_mut(user.current_character.expect("No current character"))
                    .expect("No location found");

                location.0 = new_room;

                return Ok(true);
            }
        }

        // Get all the exits in the room they're currently in
        let user_query = world.query::<(&Character, &Room)>();
        let query = world.query::<(Entity, &Room, &Exits)>();

        // If the exit matches the input, move them to that room

        Ok(true)
    }
}
