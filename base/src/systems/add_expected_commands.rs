use crate::commands::prelude::*;
use shared::prelude::*;

/// Add keywords we can quickly check in the Commands module
pub fn add_expected_commands(mut command_list: ResMut<GameCommands>) {
    command_list
        .0
        .get_mut(&UserStatus::NeedUsername)
        .unwrap()
        .push(Box::new(UsernameProvidedCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::CreatePassword)
        .unwrap()
        .push(Box::new(CreateAccountPasswordCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::NeedPassword)
        .unwrap()
        .push(Box::new(ProvideAccountPasswordCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::ConfirmPassword)
        .unwrap()
        .push(Box::new(ConfirmAccountPasswordCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::CreateCharacter)
        .unwrap()
        .push(Box::new(ProvideCharacterNameCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::LoggedIn)
        .unwrap()
        .push(Box::new(CreateCharacterCommand {}));

    command_list
        .0
        .get_mut(&UserStatus::LoggedIn)
        .unwrap()
        .push(Box::new(SelectCharacterCommand {}));
}
