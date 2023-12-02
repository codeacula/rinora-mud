use shared::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct LoggingIn {
    pub(crate) username: String,
}

#[derive(Component, Debug)]
pub(crate) struct NeedsToProvideConfirmationPassword;

#[derive(Component, Debug)]
pub(crate) struct NeedsToProvideNewPassword;

#[derive(Component, Debug)]
pub(crate) struct NeedsAccountPassword;
