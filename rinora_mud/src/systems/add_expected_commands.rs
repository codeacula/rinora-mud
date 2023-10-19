use shared::prelude::*;

use crate::commands::prelude::*;

/// Add keywords we can quickly check in the Commands module
pub fn add_expected_commands(
    mut expected_commands: ResMut<PossibleCommands>,
    mut command_list: ResMut<AccountCommands>,
) {
    expected_commands.0.push("acct".to_string());
    command_list.0.push(Box::new(UsernameProvided {}));
    command_list.0.push(Box::new(PasswordCreated {}));
    command_list.0.push(Box::new(PasswordProvided {}));
    command_list.0.push(Box::new(UserConfirmedPassword {}));
    command_list.0.push(Box::new(ProvideCharacterName {}));
    command_list.0.push(Box::new(SelectedCreateCharacter {}));
    command_list.0.push(Box::new(CharacterWasSelected {}));
}
