pub mod chess;
pub mod chess_console_player;

fn main() {
    let mut game = chess_console_player::ConsoleChessGame::new();
    game.play_loop();
}
