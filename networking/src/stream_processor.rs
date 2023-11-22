pub const IAC: u8 = 255; // Byte to signal subchannel negotiation
pub const WILL: u8 = 251; // Client WILL do something. Mostly GMCP
pub const WONT: u8 = 252; // Client WONT do something. Mostly GMCP
pub const DO: u8 = 253; // Client requests server to DO something
pub const DONT: u8 = 254; // Client requests server to DONT do something
pub const GMCP: u8 = 201; // GMCP byte flag
pub const SB: u8 = 250; // Subnegotiation Begin
pub const SE: u8 = 240; // Sub-negotiation End

/// Holds everything we need to identify a network connection
pub struct NetworkConnection {
    pub id: Uuid,         // We use a UUID so we don't have to worry about integer rollover
    pub conn: TcpStream,  // The TCP stream we use to communicate
    pub gmcp: bool,       // Whether or not the client has GMCP turned on
    pub send_room: bool,  // Does the user want room info via GMCP?
    pub send_chat: bool,  // Does the user want chat info via GMCP?
    pub send_stats: bool, // Should we send character stats via GMCP?
    pub send_time: bool, //Send time changes via GMCP. This should be morning, afternoon, dusk, night, and midnights
}

pub trait Step {
    pub fn push(&mut self, &mut network_connection: NetworkConnection, byte: u8) -> Step;
}

struct StartingStep;

impl Step for StartingStep {
    fn push(&mut self, byte: u8) -> Step {
        if byte == IAC {
            return StartingNegotiation::new();
        }
    }
}

struct StartingNegotiation;

impl Step for StartingNegotiation {
    fn push(&mut self, byte: u8) -> Step {
        if byte == IAC {
            return Negotiating::new();
        }
    }
}

pub struct StreamProcessor {
    connection: NetworkConnection,
    current_step: Box<dyn Step>,
}

impl StreamProcessor {
    pub fn process(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let mut buffer = [0; 1024];
        let mut stream = stream.try_clone()?;
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            stream.write(&buffer[..bytes_read])?;
        }
        Ok(())
    }
}
