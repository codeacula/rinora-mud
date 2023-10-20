/// Telnet protocol constants
/// Byte to signal subchannel negotiation
pub const IAC: i32 = 255;

/// Client WILL do something. Mostly GMCP
pub const WILL: i32 = 251;

/// Client WONT do something. Mostly GMCP
pub const WONT: i32 = 252;

/// Client requests server to DO something
pub const DO: i32 = 253;

/// Client requests server to DONT do something
pub const DONT: i32 = 254;

/// GMCP byte flag
pub const GMCP: i32 = 201;
