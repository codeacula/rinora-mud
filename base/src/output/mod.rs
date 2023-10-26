pub mod character_name_invalid;
pub mod character_was_created;
pub mod display_character_entering_room;
pub mod display_character_exists;
pub mod display_character_logged_into_room;
pub mod display_room_to_user;
pub mod handle_generic_error;
pub mod send_prompt_to_user;
pub mod show_login_screen;
pub mod username_does_not_exist;
pub mod username_exists;

pub mod prelude {
    pub use crate::output::character_name_invalid::*;
    pub use crate::output::character_was_created::*;
    pub use crate::output::display_character_entering_room::*;
    pub use crate::output::display_character_exists::*;
    pub use crate::output::display_character_logged_into_room::*;
    pub use crate::output::display_room_to_user::*;
    pub use crate::output::handle_generic_error::*;
    pub use crate::output::send_prompt_to_user::*;
    pub use crate::output::show_login_screen::*;
    pub use crate::output::username_does_not_exist::*;
    pub use crate::output::username_exists::*;
}
