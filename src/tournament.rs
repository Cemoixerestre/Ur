use crate::game_manager::*;
use rand::{Rng, thread_rng};

fn roll_dice<R: Rng>(rng: &mut R) -> usize {
    let mut x = rng.gen_range(0..16);
    let mut dice = 0;
    for _ in 0..4 {
        dice += x & 1;
        x >>= 1;
    }

    dice
}

pub fn showdown(
    player_a: impl Fn(&Board, usize) -> usize,
    player_b: impl Fn(&Board, usize) -> usize,
    nb_games: u32
) -> [u32; 2] {
    let mut res = [0, 0];
    let mut rng = thread_rng();
    for _ in 0..nb_games {
        let mut board = Board::default();
        loop {
            let dice = roll_dice(&mut rng);
            let places = board.possible_moves(dice);
            if places.is_empty() {
                board.turn = 1 - board.turn;
                continue;
            }
            let place = if board.turn == 0 {
                player_a(&board, dice)
            }
            else {
                player_b(&board, dice)
            };
            if board.perform_move(dice, place) {
                res[board.turn] += 1;
                break;
            }
        }

        loop {
            let dice = roll_dice(&mut rng);
            let places = board.possible_moves(dice);
            if places.is_empty() {
                board.turn = 1 - board.turn;
                continue;
            }
            let place = if board.turn == 0 {
                player_b(&board, dice)
            }
            else {
                player_a(&board, dice)
            };
            if board.perform_move(dice, place) {
                res[1 - board.turn] += 1;
                break;
            }
        }
    }

    res
}
