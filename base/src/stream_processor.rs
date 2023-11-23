use bevy::utils::Uuid;

use crate::{enums::*, gmcp::*, models::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkCommandType {
    User,
    Gmcp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkCommand {
    pub command_type: NetworkCommandType,
    pub command_name: String,
    pub data: Option<Vec<u8>>,
}

pub trait Step {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkEvent>);
}

pub struct BufferProcessor {
    current_command: Option<NetworkEvent>,
    current_step: Box<dyn Step>,
}

impl BufferProcessor {
    fn next(&mut self, byte: u8) -> Option<NetworkEvent>{
        (next_step, command_result) = self.current_step.next(byte);

        self.current_step = next_step;
        None
    }

    fn get_current_command(mut self) -> Option<NetworkEvent> {
        let command_to_send = self.current_command;
        self.current_command = None;
        command_to_send
    }
}

#[derive(Debug)]
struct InitialState {}

impl Step for InitialState {
    fn next(&mut self, processor: &mut BufferProcessor, byte: u8) ->  (Box<dyn Step>, Option<NetworkEvent>) {
        match byte {
            IAC => (Box::new(IACState {}), None),
            _ => (Box::new(InitialState {}), None),
        }
    }
}

#[derive(Debug)]
struct IACState {}

impl Step for IACState {
    fn next(&mut self, processor: &mut BufferProcessor, byte: u8) ->  (Box<dyn Step>, Option<NetworkEvent>) {
        match byte {
            DO => (Box::new(DoState {}), None),
            _ => (Box::new(InitialState {}), None),
        }
    }
}

#[derive(Debug)]
struct DoState {}

impl Step for DoState {
    fn next(&mut self, processor: &mut BufferProcessor, byte: u8) ->  (Box<dyn Step>, Option<NetworkEvent>) {
        match byte {
            GMCP => (Box::new(InitialState {}), None)
            _ => (Box::new(InitialState {}), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{TcpStream, TcpListener};

    use super::*;
    use crate::enums::NetworkEventType;

    #[test]
    fn processor_switched_to_iac_state_from_inital() {
        let mut processor = BufferProcessor {
            current_command: None,
            current_step: Box::new(InitialState {}),
        };

        processor.next(IAC);
        processor.next(DO);
        processor.next(GMCP);

        let command = processor.get_current_command();

        assert!(command.is_some());
        assert_eq!(command.unwrap().event_type, NetworkEventType::GmcpReceived);
    }
}
