use crate::commands::prelude::*;
use shared::prelude::*;

/// Add keywords we can quickly check in the Commands module
pub fn add_character_commands(mut command_list: ResMut<GameCommands>) {
    let command_vec = command_list.0.get_mut(&UserStatus::InGame).unwrap();

    command_vec.push(Box::new(MoveToRoomCommand {}));
    command_vec.push(Box::new(LookAtRoomCommand {}));
}
