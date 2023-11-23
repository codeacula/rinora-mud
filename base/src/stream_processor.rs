use crate::{gmcp::*, models::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkCommandType {
    User,
    Gmcp,
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
    fn processor_switched_to_iac_state_from_inital() {
        let mut processor = BufferProcessor {
            current_step: Box::new(InitialState {}),
        };

        processor.next(IAC);
        processor.next(DO);
        let command = processor.next(GMCP);

        assert!(command.is_some());
        assert_eq!(
            command.unwrap(),
            NetworkCommand {
                command_type: NetworkCommandType::TurnOnGmcp,
                command_name: String::from(""),
                data: None
            }
        );
    }
}
