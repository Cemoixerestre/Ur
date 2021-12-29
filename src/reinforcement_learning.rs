use crate::game_manager::*;

use crate::strategy::*;

#[derive(Default)]
pub struct LinearEval0 {
    pub val_ready: f32,
    pub val_cells: [f32; 14],
    pub val_out: f32,
    pub player_adv: f32,
}

impl Heuristic for LinearEval0 {
    fn victory() -> f32 {
        1.0
    }

    fn eval(&self, board: &Board) -> f32 {
        let player = board.turn;
        let adv = 1 - player;
        let mut val = self.player_adv;
        val += board.ready[player] as f32 * self.val_ready;
        val -= board.ready[adv] as f32 * self.val_ready;
        for i in 0..14 {
            if board.cells[player][i] {
                val += self.val_cells[i];
            }
            if board.cells[adv][i] {
                val -= self.val_cells[i];
            }
        }
        val += board.out[player] as f32 * self.val_out;
        val -= board.out[adv] as f32 * self.val_out;

        val
    }
}

impl LinearEval0 {
    pub fn disp(&self) {
        println!("READY: {}", self.val_ready);
        for i in 0..4 {
            println!("A{0}-C{0}: {1}", 4 - i, self.val_cells[i]);
        }
        for i in 4..12 {
            println!("B{}   : {}", i - 3, self.val_cells[i]);
        }
        for i in 12..14 {
            println!("A{0}-C{0}: {1}", 20 - i, self.val_cells[i]);
        }
        println!("OUT  : {}", self.val_out);
        println!("ADV  : {}", self.player_adv);
    }

    pub fn step(&mut self, board: &Board, to: f32, alpha: f32) {
        let diff = alpha * (to - self.eval(board));

        let player = board.turn;
        let adv = 1 - player;
        self.val_ready += board.ready[player] as f32 * diff;
        self.val_ready -= board.ready[adv] as f32 * diff;
        for i in 0..14 {
            if board.cells[player][i] {
                self.val_cells[i] += diff;
            }
            if board.cells[adv][i] {
                self.val_cells[i] -= diff;
            }
        }
        self.val_out += board.out[player] as f32 * diff;
        self.val_out -= board.out[adv] as f32 * diff;
        self.player_adv += diff;
    }
}
