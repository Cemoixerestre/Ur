mod game_manager;
use game_manager::*;

fn main() {
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

    for dice in 0..=4 {
        println!("Dice {}: possibles moves = {:?}",
                 dice, board.possible_moves(dice));
    }

    println!("After move (dice=2, place=3):");
    let mut copy = board.clone();
    copy.perform_move(2, 3);
    copy.disp();

    println!("");
    board.turn = 0;
    board.disp();
    for dice in 0..=4 {
        println!("Dice {}: possibles moves = {:?}",
                 dice, board.possible_moves(dice));
    }
}
