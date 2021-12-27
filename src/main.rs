extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod game_manager;
use game_manager::*;

mod strategy;
use strategy::*;

mod tournament;
use tournament::*;

fn last_move(board: &Board, dice: usize) -> usize {
    *board.possible_moves(dice).last().unwrap()
}

fn rand_move(board: &Board, dice: usize) -> usize {
    let mut rng = thread_rng();
    *board.possible_moves(dice).choose(&mut rng).unwrap()
}

fn greedy(board: &Board, dice: usize) -> usize {
    let moves = board.possible_moves(dice);
    let adv = 1 - board.turn;
    for &place in moves.iter().rev() {
        if is_central(place + dice) && board.cells[adv][place + dice] {
            return place;
        }
    }
    for &place in moves.iter().rev() {
        if is_rosetta(place + dice) || (place == ENTER && dice == 4) {
            return place;
        }
    }
    *moves.last().unwrap()
}

const DEPTH: u32 = 3;
fn expectimax_choose(board: &Board, dice: usize) -> usize {
    let moves = board.possible_moves(dice);
    let mut best = moves[0];
    let h = SimpleHeuristic {};
    let mut best_val = eval_move(&h, board, dice, moves[0], DEPTH - 1);
    for &place in moves.iter().skip(1) {
        let val = eval_move(&h, board, dice, place, DEPTH - 1);
        if val > best_val {
            best_val = val;
            best = place;
        }
    }

    best
}

fn main() {
    let res = showdown(expectimax_choose, greedy, 500);
    println!("Expectimax : {}/1000", res[0]);
    println!("Greedy move: {}/1000", res[1]);
    // TODO: move in test module
    /*
    let mut board = Board {
        ready: [1, 0],
        cells: [[false; 14]; 2],
        out: [2, 1],
        turn: 1,
    };
    for &i in &[1, 3, 5, 10] {
        board.cells[0][i] = true;
    }
    for &i in &[0, 1, 3, 7, 12, 13] {
        board.cells[1][i] = true;
    }
    board.disp();

    let h = SimpleHeuristic {};
    println!("Evaluation: {}", h.eval(&board));

    for dice in 0..=4 {
        println!("Dice {}", dice);
        for &place in board.possible_moves(dice).iter() {
            let val = eval_move(&h, &board, dice, place, 0);
            println!("  move {}: val = {}", place, val);
        }
    }
    for i in 0..=5 {
        println!("Eval at depth {}: {}", i, expectimax(&h, &board, i));
    }

    // Printing evaluation for the other player
    println!("\nSwitching");
    board.turn = 0;
    for dice in 0..=4 {
        println!("Dice {}", dice);
        for &place in board.possible_moves(dice).iter() {
            let val = eval_move(&h, &board, dice, place, 0);
            println!("  move {}: val = {}", place, val);
        }
    }
    println!("Eval at depth 1: {}", expectimax(&h, &board, 1));
    */
}
