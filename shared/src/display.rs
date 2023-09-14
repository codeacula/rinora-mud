pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/*
  Converts a color to the escape code we use
*/
pub fn get_color_code(color: &Color) -> String {
    return match color {
        Color::Black => String::from("0"),
        Color::Red => String::from("1"),
        Color::Green => String::from("2"),
        Color::Yellow => String::from("3"),
        Color::Blue => String::from("4"),
        Color::Magenta => String::from("5"),
        Color::Cyan => String::from("6"),
        Color::White => String::from("7"),
    };
}

pub struct TextBlock {
    pub text_slices: Vec<TextSlice>,
}

impl TextBlock {
    pub fn get_text(&self) -> String {
        let mut outgoing_text: Vec<String> = Vec::new();

        for slice in &self.text_slices {
            outgoing_text.push(build_color_code(&slice));
            outgoing_text.push(slice.text.clone());
        }

        outgoing_text.concat()
    }
}

pub struct TextSlice {
    pub foreground_color: Option<Color>,
    pub foreground_bright: bool,

    pub background_color: Option<Color>,
    pub background_bright: bool,

    pub text: String,
}

pub fn build_color_code(slice: &TextSlice) -> String {
    //"\u{1b}[1;31mtest\u{1b}[0ming\n"
    let mut code_bits: Vec<String> = Vec::new();

    if slice.foreground_bright {
        code_bits.push(String::from("1"));
    }

    if slice.foreground_color.is_some() {
        code_bits.push(format!(
            "3{}",
            get_color_code(slice.foreground_color.as_ref().unwrap())
        ))
    }

    if slice.background_bright {
        code_bits.push(String::from("1"));
    }

    if slice.background_color.is_some() {
        code_bits.push(format!(
            "4{}",
            get_color_code(&slice.background_color.as_ref().unwrap())
        ))
    }

    format!("\u{1b}[{}m", code_bits.join(";"))
}
