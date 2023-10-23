use shared::prelude::*;

use crate::commands::prelude::*;

/// Add keywords we can quickly check in the Commands module
pub fn add_expected_commands(
    mut expected_commands: ResMut<PossibleCommands>,
    mut command_list: ResMut<AccountCommands>,
) {
    expected_commands.0.push("acct".to_string());
    command_list.0.push(Box::new(UsernameProvidedCommand {}));
    command_list.0.push(Box::new(PasswordCreated {}));
    command_list.0.push(Box::new(PasswordProvided {}));
    command_list
        .0
        .push(Box::new(UserConfirmedPasswordCommand {}));
    command_list
        .0
        .push(Box::new(ProvideCharacterNameCommand {}));
    command_list
        .0
        .push(Box::new(SelectedCreateCharacterCommand {}));
    command_list.0.push(Box::new(SelectCharacterCommand {}));
}
