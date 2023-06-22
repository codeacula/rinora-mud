use main::*;
use tokio;

#[tokio::main]
async fn main() {
    let mut game_server = GameServer::new();
    game_server.start_server().await;
    game_server.start_game_loop();
}
