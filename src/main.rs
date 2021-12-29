extern crate rand;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

mod game_manager;
use game_manager::*;

mod strategy;
use strategy::*;

mod tournament;
use tournament::*;

mod reinforcement_learning;
use reinforcement_learning::*;

struct LastMove {}
impl Player for LastMove {
    fn choose_move(&mut self, board: &Board, dice: usize) -> usize {
        *board.possible_moves(dice).last().unwrap()
    }
}

struct RandMove<R: Rng> {
    rng: R,
}
impl<R: Rng> Player for RandMove<R> {
    fn choose_move(&mut self, board: &Board, dice: usize) -> usize {
        *board.possible_moves(dice).choose(&mut self.rng).unwrap()
    }
}

struct Greedy {}
impl Player for Greedy {
    fn choose_move(&mut self, board: &Board, dice: usize) -> usize {
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
}

struct ExpectimaxPlayer<H: Heuristic> {
    h: H,
    depth: u32,
}

impl<H: Heuristic> Player for ExpectimaxPlayer<H> {
    fn choose_move(&mut self, board: &Board, dice: usize) -> usize {
        let moves = board.possible_moves(dice);
        let mut best = moves[0];
        let mut best_val = eval_move(&self.h, board, dice, moves[0], self.depth - 1);
        for &place in moves.iter().skip(1) {
            let val = eval_move(&self.h, board, dice, place, self.depth - 1);
            if val > best_val {
                best_val = val;
                best = place;
            }
        }

        best
    }
}

fn main() {
    let rand_move = RandMove {rng: thread_rng()};
    let last_move = LastMove {};
    let res = showdown(rand_move, last_move, 500);
    println!("Random move: {}/1000", res[0]);
    println!("Last move  : {}/1000\n", res[1]);

    let greedy = Greedy {};
    let last_move = LastMove {};
    let res = showdown(greedy, last_move, 500);
    println!("Greedy   : {}/1000", res[0]);
    println!("Last move: {}/1000\n", res[1]);

    let linear_eval = LinearEval0 {
        val_ready: -0.11500916,
        val_cells: [
           -0.07836597,
           -0.06630784,
           -0.053295016,
           -0.06753837,
           -0.054491982,
           -0.053376794,
           -0.036632307,
           0.054832537,
           0.006475265,
           0.02226811,
           0.044687957,
           0.07274747,
           0.112757705,
           0.085373685],
        val_out: 0.12581697,
        player_adv: 0.027448557
    };

    let advancement = ExpectimaxPlayer {
        h: SimpleHeuristic {},
        depth: 4,
    };
    let linear_player = ExpectimaxPlayer {
        h: linear_eval,
        depth: 4,
    };
    let res = showdown(advancement, linear_player, 500);
    println!("Advancement: {}/1000", res[0]);
    println!("Linear     : {}/1000", res[1]);
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
