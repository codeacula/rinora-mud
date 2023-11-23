use crate::gmcp::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkCommandType {
    TurnOnGmcp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkCommand {
    pub command_type: NetworkCommandType,
    pub command_name: String,
    pub data: Option<Vec<u8>>,
}

pub trait Step {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>);
}

pub struct BufferProcessor {
    current_step: Box<dyn Step>,
}

impl BufferProcessor {
    pub fn new() -> BufferProcessor {
        BufferProcessor {
            current_step: Box::new(InitialState {}),
        }
    }

    pub fn next(&mut self, byte: u8) -> Option<NetworkCommand> {
        let (next_step, command_result) = self.current_step.next(byte);

        self.current_step = next_step;
        command_result
    }
}

#[derive(Debug)]
struct InitialState {}

impl Step for InitialState {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            IAC => (Box::new(IACState {}), None),
            _ => (Box::new(InitialState {}), None),
        }
    }
}

#[derive(Debug)]
struct IACState {}

impl Step for IACState {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            DO => (Box::new(DoState {}), None),
            _ => (Box::new(InitialState {}), None),
        }
    }
}

#[derive(Debug)]
struct DoState {}

impl Step for DoState {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            GMCP => (
                Box::new(InitialState {}),
                Some(NetworkCommand {
                    command_type: NetworkCommandType::TurnOnGmcp,
                    command_name: String::from(""),
                    data: None,
                }),
            ),
            _ => (Box::new(InitialState {}), None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_returns_command_to_activate_gmcp_when_told_to() {
        let mut processor = BufferProcessor::new();

        processor.next(IAC);
        processor.next(DO);
        let command = processor.next(GMCP);

        match command {
            Some(cmd) => assert_eq!(
                cmd,
                NetworkCommand {
                    command_type: NetworkCommandType::TurnOnGmcp,
                    command_name: String::from(""),
                    data: None
                }
            ),
            None => panic!("Expected a command, but got None"),
        }
    }

    #[test]
    fn returns_a_properly_formatted_user_command_when_given_one() {}
}
