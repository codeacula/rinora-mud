use std::{fmt, str::FromStr};

use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct ShowPromptEvent(pub Entity);

/// Represents a slice of text in a text block. Can be colored.
#[derive(Clone, Debug)]
pub struct TextSlice {
    pub foreground: i32,
    pub background: i32,
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

impl Default for TextSlice {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a full block of text, from the beginning of the line to the \n
#[derive(Debug)]
pub struct TextBlock {
    pub text_slices: Vec<TextSlice>,
}

impl TextBlock {
    /// Takes a &String with <<3:2>> color formats and converts them to a TextBlock
    /// with the appropriate number of TextSlices.
    pub fn from_string(string: &String) -> Self {
        let parser = TextBlockParser::new();

        parser.parse(string)
    }
}

impl fmt::Display for TextBlock {
    /// Converts the TextSlices in a TextBlock into a string with <<3:2>> color.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("");

        for slice in self.text_slices.iter() {
            result.push_str(&format!(
                "<<{}:{}>>{}",
                slice.foreground, slice.background, slice.text
            ));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTextBlockError;

impl FromStr for TextBlock {
    type Err = ParseTextBlockError;

    /// Takes a &str with <<3:2>> color formats and converts them to a TextBlock
    /// with the appropriate number of TextSlices.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = TextBlockParser::new();

        Ok(parser.parse(&String::from(s)))
    }
}

#[derive(Event, Debug)]
pub struct TextEvent {
    pub entity: Entity,
    pub text: TextBlock,
}

impl TextEvent {
    pub fn new(entity: Entity, text: &String) -> Self {
        TextEvent {
            entity,
            text: TextBlock::from_string(text),
        }
    }

    pub fn from_str(entity: Entity, text: &str) -> Self {
        TextEvent {
            entity,
            text: TextBlock::from_str(text).unwrap(),
        }
    }

    pub fn send_generic_error(entity: Entity) -> Self {
        TextEvent { entity, text: TextBlock::from_str("<<9:0>>There was an error processing your command. Please email codeacula@codeacula.com",).unwrap() }
    }
}

#[derive(Event, Debug)]
pub struct SendTextToEntityEvent {
    pub entity: Entity,
    pub text: String,
}

impl SendTextToEntityEvent {
    pub fn new(entity: Entity, text: &str) -> Self {
        SendTextToEntityEvent {
            entity,
            text: String::from_str(text).unwrap(),
        }
    }
    pub fn send_generic_error(entity: Entity) -> Self {
        SendTextToEntityEvent { entity, text: String::from_str("<<9:0>>There was an error processing your command. Please email codeacula@codeacula.com").unwrap() }
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

const START_COLOR_CONTROL: char = '<';
const END_COLOR: char = '>';

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

    pub fn get_int_from_buffer(&self) -> Result<i32, String> {
        match self.buffer.parse::<i32>() {
            Ok(res) => Ok(res),
            Err(e) => Err(format!(
                "Unable to parse into a number. Text: {} Err: {:?}",
                self.buffer, e
            )),
        }
    }

    fn push(&mut self, c: char) {
        self.current_slice.text.push_str(&self.buffer);
        self.current_slice.text.push(c);
        self.buffer.clear();
    }

    /// We've reached the second }. Instead of keeping a buffer to allow <<4:3},
    /// I decided to make it an error if you don't have <<4:3>>
    fn finish_color_status(&mut self, c: char) -> Result<(), String> {
        if c == END_COLOR {
            self.status = ParseStatus::Text;
            Ok(())
        } else {
            Err(format!("Invalid character sequence: {}", c))
        }
    }

    /// Parsing a background color number
    fn in_background_status(&mut self, c: char) -> Result<(), String> {
        if c.is_numeric() {
            self.buffer.push(c);
            Ok(())
        } else if c == END_COLOR {
            self.current_slice.background = self.get_int_from_buffer()?;
            self.status = ParseStatus::FinishColorBlock;
            Ok(())
        } else {
            Err(format!("Invalid character sequence: {}", c))
        }
    }

    /// Parsing a fullground color number
    fn in_foreground_status(&mut self, c: char) -> Result<(), String> {
        if c.is_numeric() {
            self.buffer.push(c);
            Ok(())

        // : is the control character to signify we're moving to background colors
        } else if c == ':' {
            self.current_slice.foreground = self.get_int_from_buffer()?;
            self.buffer.clear();
            self.status = ParseStatus::InBackground;
            Ok(())

        // This makes only a foreground color valid
        } else if c == END_COLOR {
            self.current_slice.foreground = self.get_int_from_buffer()?;
            self.buffer.clear();
            self.status = ParseStatus::FinishColorBlock;
            Ok(())
        } else {
            Err(format!("Invalid character sequence: {}", c))
        }
    }

    // We've started the first part of a color block
    fn start_color_status(&mut self, c: char) -> Result<(), String> {
        // When we reach the second patr of a color block we know we want to make
        // a new text slice
        if c == START_COLOR_CONTROL {
            // But we need to see if the slice is empty because we start with an
            // empty one, and we don't want to have blank ones all the time
            if !self.current_slice.text.is_empty() {
                let current_slice = self.current_slice.to_owned();
                self.current_slice = TextSlice::new();
                self.block.text_slices.push(current_slice);
            }

            self.buffer.clear();
            self.status = ParseStatus::InForeground;
            Ok(())
        } else {
            self.push(c);
            self.status = ParseStatus::Text;
            Ok(())
        }
    }

    // We've begun parsing
    fn start_parsing(&mut self, c: char) -> Result<(), String> {
        if c == START_COLOR_CONTROL {
            self.buffer.clear();
            self.buffer.push(START_COLOR_CONTROL);
            self.status = ParseStatus::StartColorBlock;
            Ok(())
        } else {
            self.push(c);
            self.status = ParseStatus::Text;
            Ok(())
        }
    }

    // Typical text state - simply scanning characters
    fn text_status(&mut self, c: char) -> Result<(), String> {
        if c == START_COLOR_CONTROL {
            self.buffer.push(c);
            self.status = ParseStatus::StartColorBlock;
            Ok(())
        } else {
            self.current_slice.text.push(c);
            Ok(())
        }
    }

    /// Parses a color-tagged string and turns it into a TextBlock with the appropriate
    /// amount of TextSlices
    pub fn parse(mut self, incoming_string: &String) -> TextBlock {
        self.block = TextBlock {
            text_slices: Vec::new(),
        };

        for c in incoming_string.chars() {
            let status = self.status;

            let result = match status {
                ParseStatus::Start => self.start_parsing(c),
                ParseStatus::Text => self.text_status(c),
                ParseStatus::StartColorBlock => self.start_color_status(c),
                ParseStatus::InForeground => self.in_foreground_status(c),
                ParseStatus::InBackground => self.in_background_status(c),
                ParseStatus::FinishColorBlock => self.finish_color_status(c),
            };

            if result.is_err() {
                let msg = format!(
                    "Unable to parse string: '{}' - Error: {:?}",
                    incoming_string,
                    result.err()
                );
                error!(msg);
            }
        }

        self.block.text_slices.push(self.current_slice.to_owned());
        self.block
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::TextBlock;

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
        let test_string = "<<15:8>>Warning, you're in <<2:0>>hot <<15:8>>danger!";
        let text_block = TextBlock::from_str(test_string).unwrap();

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
        let test_string = String::from("<<15:8>>Warning, you're in <<2:0>>hot <<15:8>>danger!");
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

    #[test]
    fn format_works() {
        let mut test_block: TextBlock = "".parse().unwrap();
        test_block.text_slices[0].foreground = 32;
        test_block.text_slices[0].background = 4;
        test_block.text_slices[0].text = String::from("Butts");

        assert_eq!("<<32:4>>Butts", test_block.to_string());
    }

    #[test]
    fn can_parse_not_quite_color_blocks() {
        let strn = "<5:0>What would you like your character's name to be?";
        let test_block: TextBlock = strn.parse().unwrap();

        assert_eq!(test_block.text_slices[0].background, 0);
        assert_eq!(test_block.text_slices[0].foreground, 7);
        assert_eq!(test_block.text_slices[0].text, String::from(strn));
    }
}
