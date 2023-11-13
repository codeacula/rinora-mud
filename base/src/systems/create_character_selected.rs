use shared::prelude::*;

pub fn create_character_selected(
    mut create_character_rx: EventReader<CreateCharacterSelectedEvent>,
    mut prompt_for_name_tx: EventWriter<PromptUserForCharacterName>,
    mut query: Query<&mut UserSessionData>,
) {
    for ev in create_character_rx.read() {
        let Ok(mut user_session_data) = query.get_mut(ev.0) else {
            error!("User session data not found for user {:?}", ev.0);
            continue;
        };

        user_session_data.status = UserStatus::CreateCharacter;
        prompt_for_name_tx.send(PromptUserForCharacterName(ev.0));
    }
}
