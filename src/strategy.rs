use crate::game_manager::*;

// A heuristic gives an evaluation to each board. The highest the heuristic is,
// the better the board is supposed to be. It then can be plugged into an
// an expectimax algorithm.
// The evaluation is always computed in the side of the player that is supposed
// to play. The evaluation of the other player is the opposite.
pub trait Heuristic {
    fn victory() -> f32; // The special value for the victory.
    fn eval(&self, board: &Board) -> f32;
    // fn move(&self, board: &Board);
}

// The heuristic currently used on the website. This heuristic is the
// sum of the advancement of the player's pieces minus the sum of the
// advancement of the adversary's pieces.
pub struct SimpleHeuristic {}

impl Heuristic for SimpleHeuristic {
    fn victory() -> f32 {
        7.0 * 15.0
    }

    fn eval(&self, board: &Board) -> f32 {
        let player = board.turn;
        let adv = 1 - player;
        let mut res =
            15.0 * (board.out[player] as f32 - board.out[adv] as f32);
        for i in 0..14 {
            if board.cells[player][i] {
                res += (i + 1) as f32;
            }
            if board.cells[adv][i] {
                res -= (i + 1) as f32;
            }
        }

        res
    }
}

pub fn expectimax<H: Heuristic>(h: &H, board: &Board, depth: u32) -> f32 {
    if depth == 0 {
        return h.eval(board);
    }

    let mut res = 0.0;
    for (dice, &proba) in PROBABILITIES.iter().enumerate() {
        let moves = board.possible_moves(dice);
        let mut max;
        if moves.is_empty() {
            max = eval_no_move(h, board, depth - 1);
        }
        else {
            max = eval_move(h, board, dice, moves[0], depth - 1);
            for &place in moves.iter().skip(1) {
                let val = eval_move(h, board, dice, place, depth - 1);
                if val > max {
                    max = val;
                }
            }
        }

        res += proba * max;
    }

    res
}

// Given a dice roll and the place of a piece that is going to be played,
// returns the expectimax evaluation of the next board.
pub fn eval_move<H>(
    h: &H,
    board: &Board,
    dice: usize,
    place: usize,
    depth: u32
) -> f32
where H: Heuristic
{
    let mut copy = board.clone();
    if copy.perform_move(dice, place) {
        return H::victory();
    }
    let val = expectimax(h, &copy, depth);
    if copy.turn == board.turn {
        val
    }
    else {
        -val
    }
}

// Return the evaluation of a board after a roll dice that prevents the player
// from doing anything (for example, after a 0 roll).
fn eval_no_move<H: Heuristic>(h: &H, board: &Board, depth: u32) -> f32 {
    let mut copy = board.clone();
    copy.change_turn();
    -expectimax(h, &copy, depth)
}
