pub mod display_character_entering_room;
pub mod display_character_logged_into_room;
pub mod display_room_to_user;
pub mod get_login_screen;

pub mod prelude {
    pub use crate::output::display_character_entering_room::*;
    pub use crate::output::display_character_logged_into_room::*;
    pub use crate::output::display_room_to_user::*;
    pub use crate::output::get_login_screen::*;
}
