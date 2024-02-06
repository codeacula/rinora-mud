use super::prelude::*;

#[derive(Event, Debug)]
pub struct CharacterLoggedInEvent(pub Entity);

#[derive(Debug, Event)]
pub struct SendGoAheadEvent(pub Entity);

#[derive(Debug, Event)]
pub struct SpeakEvent {
    pub speaker: Entity,
    pub target: Option<Entity>,
    pub text: String,
}
