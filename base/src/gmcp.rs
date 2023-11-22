/// Telnet protocol constants

pub const IAC: u8 = 255; // Byte to signal subchannel negotiation
pub const WILL: u8 = 251; // Client WILL do something. Mostly GMCP
pub const WONT: u8 = 252; // Client WONT do something. Mostly GMCP
pub const DO: u8 = 253; // Client requests server to DO something
pub const DONT: u8 = 254; // Client requests server to DONT do something
pub const GMCP: u8 = 201; // GMCP byte flag
pub const SB: u8 = 250; // Subnegotiation Begin
pub const SE: u8 = 240; // Sub-negotiation End
