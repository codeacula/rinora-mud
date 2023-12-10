use super::prelude::*;

#[derive(Event, Debug)]
pub struct CharacterLoggedInEvent(pub Entity);

#[derive(Debug, Event)]
pub struct SendGoAheadEvent(pub Entity);
