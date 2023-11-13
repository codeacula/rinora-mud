pub mod confirm_account_password;
pub mod create_account_password;
pub mod create_character_command;
pub mod move_to_room;
pub mod provide_account_password;
pub mod provide_character_name;
pub mod provide_username;
pub mod select_character;

pub mod prelude {
    pub use crate::commands::confirm_account_password::*;
    pub use crate::commands::create_account_password::*;
    pub use crate::commands::create_character_command::*;
    pub use crate::commands::move_to_room::*;
    pub use crate::commands::provide_account_password::*;
    pub use crate::commands::provide_character_name::*;
    pub use crate::commands::provide_username::*;
    pub use crate::commands::select_character::*;
}
