use shared::prelude::*;

#[derive(Clone, Debug, Event)]
pub(crate) struct WelcomeUserEvent(pub(crate) Entity);
