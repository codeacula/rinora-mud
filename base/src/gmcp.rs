/// Telnet protocol constants
/// Byte to signal subchannel negotiation
pub const IAC: u8 = 255;

/// Client WILL do something. Mostly GMCP
pub const WILL: u8 = 251;

/// Client WONT do something. Mostly GMCP
pub const WONT: u8 = 252;

/// Client requests server to DO something
pub const DO: u8 = 253;

/// Client requests server to DONT do something
pub const DONT: u8 = 254;

/// GMCP byte flag
pub const GMCP: u8 = 201;
