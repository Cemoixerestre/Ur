use std::default::Default;
use rand::Rng;

// The probability of getting each dice results.
pub const PROBABILITIES: [f32; 5] =
    [1.0 / 16.0, 4.0 / 16.0, 6.0 / 16.0, 4.0 / 16.0, 1.0 / 16.0];

// Simulate the roll of four dices.
pub fn roll_dices<R: Rng>(rng: &mut R) -> usize {
    let mut x = rng.gen_range(0..16);
    let mut dice = 0;
    for _ in 0..4 {
        dice += x & 1;
        x >>= 1;
    }

    dice
}


// For the board, cells are indexed by their place in a path. The path is, for
// each player, the sequence of fourteen cells that must be traversed to move
// pieces out. Here is a representation of the indexes of the cells:
// 3  2  1  0        13  12
// 4  5  6  7  8  9  10  11
// 3  2  1  0        13  12
//
// There are two paths, one for each player. Notice that the cells is range
// [4, 12[ (corresponding to the central row) intersect.

pub const fn is_central(idx: usize) -> bool {
    4 <= idx && idx < 12
}

pub const fn is_rosetta(idx: usize) -> bool {
    match idx {
        3 | 7 | 13 => true,
        _ => false,
    }
}

// The special central rosetta, that allows one player to play again and
// cannot be taken.
const CENTRAL_ROSETTA: usize = 7;

// The special value used to represent a piece that enters the board.
pub const ENTER: usize = 14;

#[derive(Clone)]
pub struct Board {
    // The number of pieces that have not been played for each player.
    pub ready: [u8; 2],
    // cells[i]: the path for player i.
    // cells[i][j] is true if player i has a piece at index j.
    pub cells: [[bool; 14]; 2],
    // The number of pieces out of the board for each player.
    pub out: [u8; 2],
    // The index of the player that is to play, either 0 or 1.
    pub turn: usize,
}

// Default board: a new board.
impl Default for Board {
    fn default() -> Self {
        Board {
            ready: [7, 7],
            cells: [[false; 14]; 2],
            out: [0, 0],
            turn: 0,
        }
    }
}

impl Board {
    pub fn disp(&self) {
        match self.turn {
            0 => println!("Turn: O"),
            1 => println!("Turn: X"),
            x => panic!("Error: player's turn should be 0 or 1, not {}", x),
        };

        // Displaying light's row
        for i in (0..4).rev() {
            let symbol =
                if self.cells[0][i] { 'O' }
                else if is_rosetta(i) { '#' }
                else { '.' };
            print!("{} ", symbol);
        }
        print!("    ");
        for i in (12..14).rev() {
            let symbol =
                if self.cells[0][i] { 'O' }
                else if is_rosetta(i) { '#' }
                else { '.' };
            print!("{} ", symbol);
        }
        println!("");

        // Displaying the common row:
        for i in 4..12 {
            assert!(!self.cells[0][i] || !self.cells[1][i],
                    "The cell nr {} should not be filled with two pieces.", i);
            let symbol =
                if self.cells[0][i] { 'O' }
                else if self.cells[1][i] { 'X' }
                else if is_rosetta(i) { '#' }
                else { '.' };
            print!("{} ", symbol);
        }
        println!("");

        // Displaying blue's row
        for i in (0..4).rev() {
            let symbol =
                if self.cells[1][i] { 'X' }
                else if is_rosetta(i) { '#' }
                else { '.' };
            print!("{} ", symbol);
        }
        print!("    ");
        for i in (12..14).rev() {
            let symbol =
                if self.cells[1][i] { 'X' }
                else if is_rosetta(i) { '#' }
                else { '.' };
            print!("{} ", symbol);
        }
        println!("\n");

        // Displaying the score:
        println!("Player O: {} ready / {} out", self.ready[0], self.out[0]);
        println!("Player X: {} ready / {} out", self.ready[1], self.out[1]);
    }

    // Given a dice result, returns the vectors of pieces that can be moved
    // given a dice result. Pieces are represented by their index. If a piece
    // can be moved in, the value ENTER is contained.
    //
    // If a piece can enter, the first element of this vector is `ENTER`. Then,
    // the indices are contained in increasing order.
    pub fn possible_moves(&self, dice: usize) -> Vec<usize> {
        if dice == 0 {
            return Vec::new();
        }

        // The first four indices are 0, 1, 2 and 3. Therefore, with dices
        // result d, a piece enters at place d - 1.
        let mut moves = Vec::new();
        if self.ready[self.turn] > 0 && !self.cells[self.turn][dice - 1] {
            moves.push(ENTER);
        }
        for i in 0..14 {
            if !self.cells[self.turn][i] {
                // No piece at place i
                continue;
            }
            if i + dice == 14 {
                // We can move out the piece at place i
                moves.push(i);
                continue;
            }
            if i + dice > 14 {
                // We cannot move out nor anywhere in the board the piece at place i
                continue;
            }
            if self.cells[self.turn][i + dice] {
                // There is already one of our pieces at place i + dice
                continue;
            }
            if i + dice == CENTRAL_ROSETTA &&
               self.cells[1 - self.turn][CENTRAL_ROSETTA] {
                // The opponent occupies the central rosetta
                continue;
            }
            moves.push(i);
        }

        moves
    }

    // Moves the piece at index `place` by `dice` cells. The player that
    // is to play may change.
    //
    // This function does not check whether the input move is legal. Please
    // ensure that this move comes from the vector `self.possible_moves(dice)`.
    //
    // Returns true if this move is a winning move, false otherwise.
    pub fn perform_move(&mut self, dice: usize, place: usize) -> bool {
        // Making a new piece enter
        if place == ENTER {
            self.cells[self.turn][dice - 1] = true;
            self.ready[self.turn] -= 1;
            if !is_rosetta(dice - 1) {
                self.change_turn();
            }
            false
        }
        // Moving out a piece
        else if place + dice == 14 {
            self.cells[self.turn][place] = false;
            self.out[self.turn] += 1;
            if self.out[self.turn] == 7 {
                true
            }
            else {
                self.change_turn();
                false
            }
        }
        // Otherwise
        else {
            self.cells[self.turn][place] = false;
            self.cells[self.turn][place + dice] = true;
            // Taking an opponent's piece
            if is_central(place + dice) &&
               self.cells[1 - self.turn][place + dice] {
               self.cells[1 - self.turn][place + dice] = false;
               self.ready[1 - self.turn] += 1;
            }
            if !is_rosetta(place + dice) {
                self.change_turn();
            }
            false
        }
    }

    // Change the player turn.
    // It may be used if a player cannot move because of the dice result.
    pub fn change_turn(&mut self) {
        self.turn = 1 - self.turn;
    }

    pub fn finished(&self) -> bool {
        self.out[1 - self.turn] == 7
    }
}
