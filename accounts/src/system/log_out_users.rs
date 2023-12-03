use database::prelude::DbInterface;
use shared::prelude::*;

pub(crate) fn log_out_users(
    query: Query<(Entity, &UserSessionData), With<LogOutUser>>,
    character_query: Query<(Entity, &Character, &Location), With<IsControlledBy>>,
    db_interface: Res<DbInterface>,
    mut commands: Commands,
) {
    for (entity, user_sesh) in query.iter() {
        if user_sesh.entity_they_are_controlling.is_some() {
            let (character_entity, character, location) = character_query
                .get(user_sesh.entity_they_are_controlling.unwrap())
                .unwrap();

            db_interface
                .characters
                .update_location(character.character_id, location.location_id);

            commands.entity(character_entity).despawn_recursive();
            debug!("Despawned character {:?}", character_entity);
        }
        commands.entity(entity).despawn_recursive();
        debug!("Despawned user {:?}", entity);
    }
}
