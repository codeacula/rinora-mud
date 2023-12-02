use shared::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct CreatingAccount {
    pub(crate) username: String,
    pub(crate) password: Option<String>,
}

#[derive(Component, Debug)]
pub(crate) struct LoggingIn {
    pub(crate) username: String,
}
