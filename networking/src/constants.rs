pub(crate) const GA: u8 = 249; // Go Ahead
pub(crate) const IAC: u8 = 255; // Byte to signal subchannel negotiation
pub(crate) const WILL: u8 = 251; // Client WILL do something. Mostly GMCP
pub(crate) const WONT: u8 = 252; // Client WONT do something. Mostly GMCP
pub(crate) const DO: u8 = 253; // Client requests server to DO something
pub(crate) const DONT: u8 = 254; // Client requests server to DONT do something
pub(crate) const GMCP: u8 = 201; // GMCP byte flag
pub(crate) const SB: u8 = 250; // Subnegotiation Begin
pub(crate) const SE: u8 = 240; // Sub-negotiation End

// All good MUDs have a banner!
pub(crate) const GREETING: &str = "
 _____  _                       __  __ _    _ _____  
|  __ \\(_)  Welcome to:        |  \\/  | |  | |  __ \\ 
| |__) |_ _ __   ___  _ __ __ _| \\  / | |  | | |  | |
|  _  /| | '_ \\ / _ \\| '__/ _` | |\\/| | |  | | |  | |
| | \\ \\| | | | | (_) | | | (_| | |  | | |__| | |__| |
|_|  \\_\\_|_| |_|\\___/|_|  \\__,_|_|  |_|\\____/|_____/ 


";
