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
        assert_eq!(true, super::is_valid_username("leroy"));
        assert_eq!(true, super::is_valid_username("LEROY"));
        assert_eq!(true, super::is_valid_username("lErOy"));
        assert_eq!(true, super::is_valid_username("leroy123"));
        assert_eq!(true, super::is_valid_username("leroy_123"));
        assert_eq!(true, super::is_valid_username("leroy_123_456"));

        assert_eq!(false, super::is_valid_username("123leroy"));
        assert_eq!(false, super::is_valid_username("123"));
        assert_eq!(false, super::is_valid_username("_diggle"));
        assert_eq!(false, super::is_valid_username("de"));
        assert_eq!(
            false,
            super::is_valid_username(
                "deeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
            )
        );
    }
}
