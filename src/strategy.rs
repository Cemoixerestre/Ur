use crate::game_manager::*;

pub trait Heuristic {
    fn victory() -> f32;
    fn eval(&self, board: &Board) -> f32;
    // fn move(&self, board: &Board);
}

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

fn eval_no_move<H: Heuristic>(h: &H, board: &Board, depth: u32) -> f32 {
    let mut copy = board.clone();
    copy.turn = 1 - copy.turn;
    -expectimax(h, &copy, depth)
}
