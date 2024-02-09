/// This method returns true is the provided string contains only A-Z, a-z, or 0-9
pub fn is_alphanumeric(inc_str: &str) -> bool {
    inc_str.chars().all(char::is_alphanumeric)
}

/// This method returns true if the provided string contains only A-Z or a-z
pub fn is_alphabetic(inc_str: &str) -> bool {
    inc_str.chars().all(char::is_alphabetic)
}

/// Determines if a string is a valid username. A valid username is between 3 and 15 characters, has letters, numbers,
/// and underscores, and does not start with a number.
pub fn is_valid_username(inc_str: &str) -> bool {
    for c in inc_str.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    let invalid_starting_chars = "0123456789_";
    let length = inc_str.len();

    length > 2 && length < 17 && !inc_str.starts_with(|c| invalid_starting_chars.contains(c))
}

/// Given a string, converts it to a full sentence, which starts with a capital letter and ends in punctuation
pub fn to_full_sentence(inc_str: &str) -> String {
    let mut characters: Vec<char> = inc_str.chars().collect();

    // Capitalize the first character
    if let Some(first_character) = characters.first_mut() {
        first_character.make_ascii_uppercase();
        characters[0] = *first_character;
    }

    // Add punctuation if missing at the end
    if let Some(last_character) = characters.last() {
        if !last_character.is_ascii_punctuation() {
            characters.push('.')
        }
    }

    characters.iter().collect()
}

/// Returns a [String] instance `inc_str` formatted in title case.
///
/// # Example:
///
/// ```
/// let title_case_name = to_title_case("doggo");
/// assert_eq!("Doggo", title_case_name);
/// ```
pub fn to_title_case(inc_str: &str) -> String {
    let mut copy = inc_str.to_string().to_lowercase();
    if let Some(r) = copy.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    copy.to_string()
}

/// Given a name, cleans it of all invalid characters and returns it in title case.
pub fn clean_name(name: &str) -> String {
    let mut cleanedup_name: Vec<char> = name
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleanedup_name[0] = cleanedup_name[0].to_ascii_uppercase();

    cleanedup_name.iter().collect::<String>()
}

const DIRECTION_MAP: [(&str, &str); 12] = [
    ("n", "north"),
    ("s", "south"),
    ("e", "east"),
    ("w", "west"),
    ("u", "up"),
    ("d", "down"),
    ("ne", "northeast"),
    ("nw", "northwest"),
    ("se", "southeast"),
    ("sw", "southwest"),
    ("in", "in"),
    ("out", "out"),
];

pub fn get_short_direction(original: &String) -> String {
    for (short, long) in DIRECTION_MAP.iter() {
        if original == *short || original == *long {
            return short.to_string();
        }
    }

    original.to_string()
}

pub fn get_long_direction(original: &String) -> String {
    for (short, long) in DIRECTION_MAP.iter() {
        if original == *short || original == *long {
            return long.to_string();
        }
    }

    original.to_string()
}

pub fn is_valid_direction(original: &String) -> bool {
    for (short, long) in DIRECTION_MAP.iter() {
        if original == *short || original == *long {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::to_title_case;

    #[test]
    fn it_does_the_title_text_correctly() {
        assert_eq!("Leroy", to_title_case("leroy"));
        assert_eq!("Leroy", to_title_case("LEROY"));
        assert_eq!("Leroy", to_title_case("lErOy"));
    }

    #[test]
    fn is_valid_username_returns_true_for_valid_names() {
        assert!(super::is_valid_username("leroy"));
        assert!(super::is_valid_username("LEROY"));
        assert!(super::is_valid_username("lErOy"));
        assert!(super::is_valid_username("leroy123"));
        assert!(super::is_valid_username("leroy_123"));
        assert!(super::is_valid_username("leroy_123_456"));

        assert!(!super::is_valid_username("123leroy"));
        assert!(!super::is_valid_username("123"));
        assert!(!super::is_valid_username("_diggle"));
        assert!(!super::is_valid_username("de"));
        assert!(!super::is_valid_username(
            "deeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
        ));
    }

    #[test]
    fn formats_sentences_correctly() {
        assert_eq!(
            super::to_full_sentence("this is a sentence"),
            "This is a sentence."
        );

        assert_eq!(
            super::to_full_sentence("this is a sentence?"),
            "This is a sentence?"
        );

        assert_eq!(
            super::to_full_sentence("This is a sentence."),
            "This is a sentence."
        );
    }
}
