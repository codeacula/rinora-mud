use shared::prelude::*;

pub(crate) fn run_user_commands(world: &mut World) {
    // Go ahead and take these out now so we don't have to deal with borrower issues
    let game_commands = world.remove_resource::<GameCommands>().unwrap();
    let mut user_provided_command_rx = world
        .remove_resource::<Events<UserProvidedCommandEvent>>()
        .unwrap();

    for ev in user_provided_command_rx.drain() {
        let mut user_command = UserCommand::new(ev.command.clone());
        user_command.entity = ev.entity;

        for command in game_commands.0.iter() {
            match command.run(&user_command, world) {
                Ok(result) => {
                    if result {
                        break;
                    }
                }
                Err(err) => {
                    error!("Error running command: {}", err);
                }
            }
        }
    }

    // We need to put the resources back when done
    world.insert_resource(game_commands);
    world.insert_resource(user_provided_command_rx);
}
