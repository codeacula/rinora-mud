use shared::prelude::*;

pub fn get_login_screen(characters: &Vec<CharacterBundle>) -> String {
    let mut greeting = String::from("Your options:\n\n");

    greeting.push_str("  [{{15}}1{{7}}]: Create Character\n");

    if characters.is_empty() {
        greeting.push_str("You currently have no characters.\n")
    } else {
        greeting.push_str("Your characters are:\n");

        for character in characters {
            greeting.push_str(&format!("  {}\n", character.display_name.0));
        }
    }

    greeting.push_str("\nSend a number command or which character you want to play.");
    greeting
}
