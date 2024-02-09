use super::prelude::*;

#[derive(Event, Debug)]
pub struct CharacterLoggedInEvent(pub Entity);

#[derive(Debug, Event)]
pub struct SendGoAheadEvent(pub Entity);

#[derive(Debug, Event)]
pub struct SpeakEvent {
    pub room: Entity,
    pub speaker: Entity,
    pub target: Option<Entity>,
    pub text: String,
}

#[derive(Debug, Event)]
pub struct HeardEvent {
    pub listener: Entity,
    pub speaker: Entity,
    pub target: Option<Entity>,
    pub text: String,
}
