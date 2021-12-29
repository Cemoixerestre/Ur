use rand::thread_rng;
use crate::game_manager::*;

pub trait Player {
    fn choose_move(&mut self, board: &Board, dice: usize) -> usize;
}

// Plays 2 * nb_games games between two players.
// Each player the same number of games as the light and the blue player.
// Returns the array [a, b] where a and b are the number of games won by
// players a and b.
// TODO: returns the number of games won when playing light or blue.
pub fn showdown(
    mut player_a: impl Player,
    mut player_b: impl Player,
    nb_games: u32
) -> [u32; 2] {
    let mut res = [0, 0];
    let mut rng = thread_rng();
    for _ in 0..nb_games {
        let mut board = Board::default();
        loop {
            // Plays a game where player_a starts.
            let dice = roll_dices(&mut rng);
            let places = board.possible_moves(dice);
            if places.is_empty() {
                board.change_turn();
                continue;
            }
            let place = if places.len() == 1 {
                // No need to call a choice function if there is only one
                // possible move.
                places[0]
            }
            else if board.turn == 0 {
                player_a.choose_move(&board, dice)
            }
            else {
                player_b.choose_move(&board, dice)
            };
            assert!(places.contains(&place));
            if board.perform_move(dice, place) {
                res[board.turn] += 1;
                break;
            }
        }

        loop {
            let dice = roll_dices(&mut rng);
            let places = board.possible_moves(dice);
            if places.is_empty() {
                board.change_turn();
                continue;
            }
            let place = if places.len() == 1 {
                places[0]
            }
            else if board.turn == 0 {
                player_b.choose_move(&board, dice)
            }
            else {
                player_a.choose_move(&board, dice)
            };
            assert!(places.contains(&place));
            if board.perform_move(dice, place) {
                res[1 - board.turn] += 1;
                break;
            }
        }
    }

    res
}
