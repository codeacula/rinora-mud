use bevy::prelude::*;
use shared::prelude::*;

pub fn manage_character_list(
    mut query: Query<(Entity, &mut UserSessionData)>,
    mut incoming_account_events: EventReader<AccountEvent>,
) {
    for account_event in incoming_account_events.iter() {
        let (_entity, user_sesh) = query.get_mut(account_event.entity).unwrap();

        match user_sesh.status {
            UserStatus::LoggedIn => {
                info!("Made it here");
            }
            _ => continue,
        }
    }
}
