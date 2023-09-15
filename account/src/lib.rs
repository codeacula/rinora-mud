use bevy::prelude::*;
use database::prelude::*;
use shared::prelude::*;

pub struct AccountPlugin;

fn add_expected_commands(mut expected_commands: ResMut<PossibleCommands>) {
    expected_commands.0.push("acct".to_string());
}

fn handle_new_connections(
    mut ev_new_connection: EventReader<NewConnectionEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
) {
    for ev in ev_new_connection.iter() {
        ev_outgoing_text_events.send(TextEvent::new(
            ev.entity,
            &String::from("Please provide your username."),
        ));
    }
}

fn handle_disconnect(
    mut ev_disconnection_event: EventReader<DisconnectionEvent>,
    query: Query<&User>,
    mut commands: Commands,
) {
    for ev in ev_disconnection_event.iter() {
        if let Ok(_user) = query.get(ev.entity) {
            commands.entity(ev.entity).despawn_recursive();
        } else {
            error!("User disconnected but no user component found");
        }
    }
}

fn handle_account_event(
    mut query: Query<(Entity, &mut User)>,
    mut incoming_account_events: EventReader<AccountEvent>,
    mut ev_outgoing_text_events: EventWriter<TextEvent>,
    db_repo: Res<DbInterface>,
) {
    for account_event in incoming_account_events.iter() {
        let (entity, mut user) = query.get_mut(account_event.entity).unwrap();

        match user.status {
            UserStatus::NeedUsername => {
                let username = account_event.input[0].clone();

                if db_repo.users.does_user_exist(&username).unwrap() {
                    ev_outgoing_text_events.send(TextEvent::new(
                        entity,
                        &String::from("User account found. Please provide your password."),
                    ));
                } else {
                    ev_outgoing_text_events.send(TextEvent::new(
                        entity,
                        &String::from("Welcome, new user! Please provide your password!"),
                    ));
                }

                user.username = username;
                user.status = UserStatus::NeedPassword;
                ev_outgoing_text_events.send(TextEvent::new(
                    entity,
                    &String::from("Please provide your password."),
                ));
            }
            UserStatus::NeedPassword => {
                user.status = UserStatus::InGame;
                ev_outgoing_text_events.send(TextEvent::new(
                    entity,
                    &String::from("You are now logged in!"),
                ));
            }
            UserStatus::LoggedIn => {
                info!("Logged in");
            }
            UserStatus::InGame => {
                error!("Shouldn't have made it here!");
            }
        }
    }
}

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_expected_commands).add_systems(
            Update,
            (
                handle_disconnect,
                handle_new_connections,
                handle_account_event,
            ),
        );
    }
}
