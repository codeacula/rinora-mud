use crate::constants::*;

/// Specifies the type of command that was received from the network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkCommandType {
    TurnOnGmcp,
    UserCommand,
    GmcpCommand,
}

/// Represents a command that was received from the network.
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
            SB => (Box::new(InitialState {}), None),
            WILL => (Box::new(InitialState {}), None),
            _ => (Box::new(CollectingText { buffer: vec![byte] }), None),
        }
    }
}

#[derive(Debug)]
struct IACState {}

impl Step for IACState {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            DO => (Box::new(DoState {}), None),
            SB => (Box::new(SubnegotiationStart {}), None),
            SE => (Box::new(InitialState {}), None),
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

#[derive(Debug)]
struct CollectingText {
    buffer: Vec<u8>,
}

impl Step for CollectingText {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            IAC => (Box::new(IACState {}), None),
            b'\n' => {
                self.buffer.push(byte);
                (
                    Box::new(InitialState {}),
                    Some(NetworkCommand {
                        command_type: NetworkCommandType::UserCommand,
                        command_name: String::from(""),
                        data: Some(self.buffer.clone()),
                    }),
                )
            }
            b'\r' => (
                Box::new(CollectingText {
                    buffer: self.buffer.clone(),
                }),
                None,
            ),
            _ => {
                self.buffer.push(byte);
                (
                    Box::new(CollectingText {
                        buffer: self.buffer.clone(),
                    }),
                    None,
                )
            }
        }
    }
}

#[derive(Debug)]
struct SubnegotiationStart {}

impl Step for SubnegotiationStart {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            GMCP => (Box::new(ReadingGmcpCommand { buffer: Vec::new() }), None),
            _ => (Box::new(SubnegotiationStart {}), None),
        }
    }
}

#[derive(Debug)]
struct ReadingGmcpCommand {
    buffer: Vec<u8>,
}

impl Step for ReadingGmcpCommand {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            IAC => (Box::new(IACState {}), None),
            SE => (
                Box::new(InitialState {}),
                Some(NetworkCommand {
                    command_type: NetworkCommandType::GmcpCommand,
                    command_name: String::from(""),
                    data: Some(self.buffer.clone()),
                }),
            ),
            b' ' => (
                Box::new(ReadingGmcpCommandData {
                    buffer: Vec::new(),
                    name: String::from_utf8_lossy(&self.buffer).to_string(),
                }),
                None,
            ),
            _ => {
                self.buffer.push(byte);
                (
                    Box::new(ReadingGmcpCommand {
                        buffer: self.buffer.clone(),
                    }),
                    None,
                )
            }
        }
    }
}

#[derive(Debug)]
struct ReadingGmcpCommandData {
    buffer: Vec<u8>,
    name: String,
}

impl Step for ReadingGmcpCommandData {
    fn next(&mut self, byte: u8) -> (Box<dyn Step>, Option<NetworkCommand>) {
        match byte {
            IAC => (
                Box::new(IACState {}),
                Some(NetworkCommand {
                    command_type: NetworkCommandType::GmcpCommand,
                    command_name: self.name.clone(),
                    data: Some(self.buffer.clone()),
                }),
            ),
            SE => (
                Box::new(InitialState {}),
                Some(NetworkCommand {
                    command_type: NetworkCommandType::GmcpCommand,
                    command_name: String::from(""),
                    data: Some(self.buffer.clone()),
                }),
            ),
            _ => {
                self.buffer.push(byte);
                (
                    Box::new(ReadingGmcpCommandData {
                        buffer: self.buffer.clone(),
                        name: self.name.clone(),
                    }),
                    None,
                )
            }
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
    fn returns_a_properly_formatted_user_command_when_given_one() {
        let test = String::from("This is a test.\r\n");

        let mut processor = BufferProcessor::new();
        let mut network_command: Option<NetworkCommand> = None;

        let mut commands: Vec<NetworkCommand> = Vec::new();

        for byte in test.bytes() {
            network_command = processor.next(byte);

            if network_command.is_some() {
                commands.push(network_command.clone().unwrap());
            }
        }

        assert_eq!(commands.len(), 1);

        match network_command {
            Some(cmd) => assert_eq!(
                cmd,
                NetworkCommand {
                    command_type: NetworkCommandType::UserCommand,
                    command_name: String::from(""),
                    data: Some(String::from("This is a test.\n").bytes().collect()),
                }
            ),
            None => panic!("Expected a command, but got None"),
        }
    }

    #[test]
    fn returns_a_properly_formatted_gmcp_command_when_given_one() {
        let test = [
            255, 250, 201, 67, 111, 114, 101, 46, 72, 101, 108, 108, 111, 32, 123, 32, 34, 99, 108,
            105, 101, 110, 116, 34, 58, 32, 34, 77, 117, 100, 108, 101, 116, 34, 44, 32, 34, 118,
            101, 114, 115, 105, 111, 110, 34, 58, 32, 34, 52, 46, 49, 55, 46, 50, 34, 125, 255,
            240,
        ];

        let mut processor = BufferProcessor::new();
        let mut network_command: Option<NetworkCommand> = None;

        for byte in test {
            let returned_command = processor.next(byte);

            if returned_command.is_some() {
                network_command = returned_command;
            }
        }

        match network_command {
            Some(cmd) => assert_eq!(
                cmd,
                NetworkCommand {
                    command_type: NetworkCommandType::GmcpCommand,
                    command_name: String::from("Core.Hello"),
                    data: Some(
                        String::from("{ \"client\": \"Mudlet\", \"version\": \"4.17.2\"}")
                            .bytes()
                            .collect()
                    ),
                }
            ),
            None => panic!("Expected a command, but got None"),
        }
    }

    #[test]
    fn returns_two_network_commands_when_back_to_back() {
        let test = String::from("This is a test.\r\nAnd so is this.\r\n");

        let mut processor = BufferProcessor::new();
        let mut commands: Vec<NetworkCommand> = Vec::new();

        for byte in test.bytes() {
            let network_command = processor.next(byte);

            if network_command.is_some() {
                commands.push(network_command.clone().unwrap());
            }
        }

        assert_eq!(commands.len(), 2);

        assert_eq!(commands[0].command_type, NetworkCommandType::UserCommand);
        assert_eq!(commands[1].command_type, NetworkCommandType::UserCommand);

        assert_eq!(
            commands[0].data,
            Some(String::from("This is a test.\n").bytes().collect())
        );
        assert_eq!(
            commands[1].data,
            Some(String::from("And so is this.\n").bytes().collect())
        );
    }
}
