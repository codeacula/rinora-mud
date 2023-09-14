/// Represents a full block of text, from the beginning of the line to the \n
pub struct TextBlock {
    pub text_slices: Vec<TextSlice>,
}

/// Represents a slice of text in a text block. Can be colored.
#[derive(Clone)]
pub struct TextSlice {
    pub foreground: i8,
    pub background: i8,
    pub text: String,
}

impl TextSlice {
    pub fn new() -> Self {
        TextSlice {
            background: 0,
            foreground: 7,
            text: String::from(""),
        }
    }
}

impl TextBlock {
    pub fn from_str(string: &str) -> Self {
        let parser = TextBlockParser::new();

        parser.parse(&String::from(string))
    }
    pub fn from_string(string: &String) -> Self {
        let parser = TextBlockParser::new();

        parser.parse(string)
    }
}

#[derive(Clone, Copy)]
enum ParseStatus {
    Start,
    Text,
    StartColorBlock,
    InForeground,
    InBackground,
    FinishColorBlock,
}

struct TextBlockParser {
    buffer: String,
    current_slice: TextSlice,
    block: TextBlock,
    status: ParseStatus,
}

const START_COLOR_CONTROL: char = '{';
const END_COLOR: char = '}';

impl TextBlockParser {
    pub fn new() -> Self {
        TextBlockParser {
            buffer: String::from(""),
            current_slice: TextSlice::new(),
            block: TextBlock {
                text_slices: Vec::new(),
            },
            status: ParseStatus::Start,
        }
    }

    fn push(&mut self, c: char) {
        self.current_slice.text.push_str(&self.buffer);
        self.current_slice.text.push(c);
        self.buffer.clear();
    }

    fn finish_color_status(&mut self, c: char) {
        if c == END_COLOR {
            self.status = ParseStatus::Text
        } else {
            panic!("Invalid character sequence: {}", c);
        }
    }

    fn in_background_status(&mut self, c: char) {
        if c.is_numeric() {
            self.buffer.push(c);
        } else if c == END_COLOR {
            self.current_slice.background = self.buffer.parse().unwrap();
            self.buffer.clear();
            self.status = ParseStatus::FinishColorBlock
        } else {
            panic!("Invalid character sequence: {}", c);
        }
    }

    fn in_foreground_status(&mut self, c: char) {
        if c.is_numeric() {
            self.buffer.push(c);
        } else if c == ':' {
            self.current_slice.foreground = self.buffer.parse().unwrap();
            self.buffer.clear();
            self.status = ParseStatus::InBackground
        } else if c == END_COLOR {
            self.current_slice.foreground = self.buffer.parse().unwrap();
            self.buffer.clear();
            self.status = ParseStatus::FinishColorBlock
        } else {
            panic!("Invalid character sequence: {}", c);
        }
    }

    fn start_color_status(&mut self, c: char) {
        if c == START_COLOR_CONTROL {
            if !self.current_slice.text.is_empty() {
                let current_slice = self.current_slice.to_owned();
                self.current_slice = TextSlice::new();
                self.block.text_slices.push(current_slice);
            }

            self.buffer.clear();
            self.status = ParseStatus::InForeground
        } else {
            self.push(c);
            self.status = ParseStatus::Text
        }
    }

    fn start_parsing(&mut self, c: char) {
        if c == START_COLOR_CONTROL {
            self.buffer.clear();
            self.status = ParseStatus::StartColorBlock
        } else {
            self.push(c);
            self.status = ParseStatus::Text
        }
    }

    fn text_status(&mut self, c: char) {
        if c == START_COLOR_CONTROL {
            self.buffer.push(c);
            self.status = ParseStatus::StartColorBlock
        } else {
            self.current_slice.text.push(c);
        }
    }

    pub fn parse(mut self, incoming_string: &String) -> TextBlock {
        self.block = TextBlock {
            text_slices: Vec::new(),
        };

        for c in incoming_string.chars() {
            let status = self.status;
            match status {
                ParseStatus::Start => self.start_parsing(c),
                ParseStatus::Text => self.text_status(c),
                ParseStatus::StartColorBlock => self.start_color_status(c),
                ParseStatus::InForeground => self.in_foreground_status(c),
                ParseStatus::InBackground => self.in_background_status(c),
                ParseStatus::FinishColorBlock => self.finish_color_status(c),
            }
        }

        self.block.text_slices.push(self.current_slice.to_owned());
        self.block
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::TextBlock;

    use super::TextSlice;

    #[test]
    fn default_looks_good() {
        let text_slice = TextSlice::new();

        assert_eq!(text_slice.background, 0);
        assert_eq!(text_slice.foreground, 7);
        assert_eq!(text_slice.text, String::from(""));
    }

    #[test]
    fn it_processes_a_str() {
        let test_string = "{{15:8}}Warning, you're in {{2:0}}hot {{15:8}}danger!";
        let text_block = TextBlock::from_str(test_string);

        assert_eq!(text_block.text_slices.len(), 3);

        // First block
        assert_eq!(text_block.text_slices[0].foreground, 15);
        assert_eq!(text_block.text_slices[0].background, 8);
        assert_eq!(text_block.text_slices[0].text, "Warning, you're in ");

        // Second block
        assert_eq!(text_block.text_slices[1].foreground, 2);
        assert_eq!(text_block.text_slices[1].background, 0);
        assert_eq!(text_block.text_slices[1].text, "hot ");

        // Third block
        assert_eq!(text_block.text_slices[2].foreground, 15);
        assert_eq!(text_block.text_slices[2].background, 8);
        assert_eq!(text_block.text_slices[2].text, "danger!");
    }

    #[test]
    fn it_processes_a_string() {
        let test_string = String::from("{{15:8}}Warning, you're in {{2:0}}hot {{15:8}}danger!");
        let text_block = TextBlock::from_string(&test_string);

        assert_eq!(text_block.text_slices.len(), 3);

        // First block
        assert_eq!(text_block.text_slices[0].foreground, 15);
        assert_eq!(text_block.text_slices[0].background, 8);
        assert_eq!(text_block.text_slices[0].text, "Warning, you're in ");

        // Second block
        assert_eq!(text_block.text_slices[1].foreground, 2);
        assert_eq!(text_block.text_slices[1].background, 0);
        assert_eq!(text_block.text_slices[1].text, "hot ");

        // Third block
        assert_eq!(text_block.text_slices[2].foreground, 15);
        assert_eq!(text_block.text_slices[2].background, 8);
        assert_eq!(text_block.text_slices[2].text, "danger!");
    }
}
