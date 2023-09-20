pub fn to_title_case(inc_str: &str) -> String {
    let mut copy = inc_str.clone().to_string().to_lowercase();
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
