use bevy::prelude::*;
use shared::{
    network::{DisconnectionEvent, NewConnectionEvent, OutgoingQueue},
    user::User,
};

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

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_disconnect, handle_new_connections));
    }
}
