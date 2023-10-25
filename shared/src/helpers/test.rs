use crate::prelude::*;
use Uuid;

pub fn verify_account_command_runs_on(
    command_to_test: &Box<dyn GameCommand>,
    when_should_pass: UserStatus,
    user_command: &UserCommand,
    world: &mut World,
) {
    let statuses_to_test = vec![
        UserStatus::CreateCharacter,
        UserStatus::CreatePassword,
        UserStatus::ConfirmDelete,
        UserStatus::ConfirmPassword,
        UserStatus::DeleteCharacter,
        UserStatus::InGame,
        UserStatus::LoggedIn,
        UserStatus::NeedUsername,
        UserStatus::NeedPassword,
        UserStatus::ToggleAutologin,
    ];

    for status_to_test in statuses_to_test {
        let connection = Uuid::new_v4();
        let username = String::from("apollo");

        let should_return_true = when_should_pass == status_to_test;

        world
            .entity_mut(user_command.entity)
            .insert(UserSessionData {
                status: status_to_test,
                char_to_delete: None,
                controlling_entity: None,
                username,
                connection,
                pwd: None,
            });

        assert_eq!(
            should_return_true,
            command_to_test.run(user_command, world).unwrap()
        );
    }
}
