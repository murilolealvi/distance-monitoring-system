use std::collections::VecDeque;

mod domains;
use domains::player::Player;

mod services;
use services::game_handler::GameHandler;

fn main() {
    let player1 = Player::new("Murilo".to_string(), 'X');
    let player2 = Player::new("Éder".to_string(), 'O');
    let mut players: VecDeque<Player> = VecDeque::new();
    players.push_back(player1);
    players.push_back(player2);

    let mut tic_tac_toe_game_handler = GameHandler::new(players, 3);
    tic_tac_toe_game_handler.play_game();
}
