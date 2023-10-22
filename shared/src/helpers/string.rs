/// This method returns true is the provided string contains only A-Z, a-z, or 0-9
pub fn is_alphanumeric(inc_str: &str) -> bool {
    inc_str.chars().all(char::is_alphanumeric)
}

/// This method returns true if the provided string contains only A-Z or a-z
pub fn is_alphabetic(inc_str: &str) -> bool {
    inc_str.chars().all(char::is_alphabetic)
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

#[cfg(test)]
mod tests {
    use super::to_title_case;

    #[test]
    fn it_does_the_title_text_correctly() {
        assert_eq!("Leroy", to_title_case("leroy"));
        assert_eq!("Leroy", to_title_case("LEROY"));
        assert_eq!("Leroy", to_title_case("lErOy"));
    }
}
