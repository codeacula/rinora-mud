#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// What type of events the server will issue the game
pub enum NetworkEventType {
    NewConnection,
    InputReceived,
    ConnectionDropped,
}
