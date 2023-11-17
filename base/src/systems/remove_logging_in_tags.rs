use shared::prelude::*;

pub fn remove_logging_in_tags(
    logging_in_query: Query<(Entity, &EntityIsLoggingIn)>,
    mut commands: Commands,
) {
    for (entity, _) in logging_in_query.iter() {
        commands.entity(entity).remove::<EntityIsLoggingIn>();
    }
}
