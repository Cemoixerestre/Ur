extern crate rand;
use rand::thread_rng;

mod game_manager;
use game_manager::*;

mod strategy;
use strategy::*;

mod reinforcement_learning;
use reinforcement_learning::*;

const ALPHA: f32 = 1e-4;

fn main() {
    let mut evaluator = LinearEval0::default();
    let mut rng = thread_rng();
    // It converges in ~5000 games
    for i in 1..=10000 {
        let mut board = Board::default();
        loop {
            let expectimax_val = expectimax(&evaluator, &board, 1);
            evaluator.step(&board, expectimax_val, ALPHA);
            let dice = roll_dices(&mut rng);
            let moves = board.possible_moves(dice);
            if moves.is_empty() {
                board.change_turn();
                continue;
            }
            let mut best_move = moves[0];
            let mut best_val = eval_move(&evaluator, &board, dice, moves[0], 0);
            for &m in moves.iter().skip(1) {
                let val = eval_move(&evaluator, &board, dice, m, 0);
                if val > best_val {
                    best_move = m;
                    best_val = val;
                }
            }

            if board.perform_move(dice, best_move) {
                break;
            }
        }

        if i % 1000 == 0 {
            println!("\nAfter {} games:", i);
            evaluator.disp();
        }
    }
}
