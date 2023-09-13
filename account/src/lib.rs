use bevy::prelude::*;
use shared::prelude::*;

pub struct AccountPlugin;

fn handle_new_connections(
    mut ev_new_connection: EventReader<NewConnectionEvent>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for ev in ev_new_connection.iter() {
        outgoing_queue.send_str(ev.id, "Please provide your username.\n");
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
    mut query: Query<&mut User>,
    mut incoming_account_events: EventReader<AccountEvent>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for account_event in incoming_account_events.iter() {
        let mut user = query.get_mut(account_event.entity).unwrap();

        match user.status {
            UserStatus::NeedUsername => {
                user.username = account_event.input[0].clone();
                user.status = UserStatus::NeedPassword;
                outgoing_queue.send_str(user.connection, "Please provide your password.\n");
            }
            UserStatus::NeedPassword => {
                user.status = UserStatus::InGame;
                outgoing_queue.send_str(user.connection, "You are now logged in!\n");
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
        app.add_systems(
            Update,
            (
                handle_disconnect,
                handle_new_connections,
                handle_account_event,
            ),
        );
    }
}
