use crate::board::Board;
use crate::board::Space::{Empty, Player1, Player2};
use std::cmp::Ordering;

pub struct AI {
    max_depth: i32,
}

impl AI {
    pub fn new(max_depth: i32) -> AI {
        AI { max_depth }
    }

    pub fn best_move(&self, board: &Board) -> Option<usize> {
        let mut best_score = -1;
        let mut best_move = None;

        for (i, space) in board.spaces().iter().enumerate() {
            if *space == Empty {
                let mut next = board.clone();
                next.play(i);
                let score = self.minimax(&next, 0, false);

                match score.cmp(&best_score) {
                    Ordering::Greater => {
                        best_score = score;
                        best_move = Some(i);
                    }
                    _ => {}
                }
            }
        }

        best_move
    }

    fn minimax(&self, board: &Board, depth: i32, is_maximizing: bool) -> i32 {
        if depth == self.max_depth || board.is_game_over() {
            return self.evaluate(board);
        }

        let mut best_score = if is_maximizing {
            std::i32::MIN
        } else {
            std::i32::MAX
        };

        for (i, space) in board.spaces().iter().enumerate() {
            if *space == Empty {
                let mut next = board.clone();
                next.play(i);
                let score = self.minimax(&next, depth + 1, !is_maximizing);

                if is_maximizing {
                    best_score = best_score.max(score);
                } else {
                    best_score = best_score.min(score);
                }
            }
        }

        best_score
    }

    fn evaluate(&self, board: &Board) -> i32 {
        match board.winner() {
            Some(Player1) => 1,
            Some(Player2) => -1,
            _ => 0,
        }
    }
}
