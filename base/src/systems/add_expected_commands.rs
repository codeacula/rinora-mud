use crate::commands::prelude::*;
use bevy::utils::HashMap;
use shared::prelude::*;

/// Add keywords we can quickly check in the Commands module
pub fn add_expected_commands(mut command_list: ResMut<GameCommands>) {
    command_list.0[UserStatus::NeedUsername]
    command_list.0.push(
        UserStatus::NeedUsername,
        Box::new(UsernameProvidedCommand {}),
    );
    command_list
        .0
        .push(UserStatus::CreatePassword, Box::new(PasswordCreated {}));
    command_list.0.push(Box::new(PasswordProvided {}));
    command_list
        .0
        .push(Box::new(UserConfirmedPasswordCommand {}));
    command_list
        .0
        .push(Box::new(ProvideCharacterNameCommand {}));
    command_list
        .0
        .push(Box::new(SelectCreateCharacterCommand {}));
    command_list.0.push(Box::new(SelectCharacterCommand {}));
}
