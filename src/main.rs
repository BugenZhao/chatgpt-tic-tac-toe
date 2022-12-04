use std::io;
use std::str::FromStr;

use crate::ai::AI;
use crate::board::{Board, Space};

mod ai;
mod board;

fn main() {
    let mut board = Board::new();
    let ai = AI::new(4);

    loop {
        println!("{}", board);

        if let Some(winner) = board.winner() {
            println!("Player {} wins!", winner);
            break;
        }

        if board.is_game_over() {
            println!("Tie game!");
            break;
        }

        let next = match board.turn {
            Space::Player1 => player_move(&board),
            Space::Player2 => ai_move(&board, &ai),
            _ => unreachable!(),
        };

        board.play(next);
    }
}

fn player_move(board: &Board) -> usize {
    loop {
        println!("Player {}'s turn:", board.turn);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number = match usize::from_str(&input.trim()) {
            Ok(n) => n,
            Err(_) => continue,
        };

        if number > 0 && number < 10 && board.spaces[number - 1] == Space::Empty {
            return number - 1;
        }
    }
}

fn ai_move(board: &Board, ai: &AI) -> usize {
    match ai.best_move(board) {
        Some(m) => m,
        None => panic!("AI couldn't find a move"),
    }
}
