use crate::board::Space::{Empty, Player1, Player2};
use std::fmt;

#[derive(Clone, PartialEq, Copy)]
pub enum Space {
    Empty,
    Player1,
    Player2,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Empty => " ",
            Player1 => "X",
            Player2 => "O",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone)]
pub struct Board {
    pub spaces: [Space; 9],
    pub turn: Space,
}

impl Board {
    pub fn new() -> Board {
        Board {
            spaces: [Empty; 9],
            turn: Player1,
        }
    }

    pub fn play(&mut self, index: usize) {
        self.spaces[index] = self.turn;
        self.turn = match self.turn {
            Player1 => Player2,
            Player2 => Player1,
            Empty => panic!("Invalid turn"),
        }
    }

    pub fn winner(&self) -> Option<Space> {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in lines.iter() {
            let spaces = self.spaces();
            let first = spaces[line[0]];
            let second = spaces[line[1]];
            let third = spaces[line[2]];

            if first != Empty && first == second && first == third {
                return Some(first);
            }
        }

        None
    }

    pub fn is_game_over(&self) -> bool {
        self.winner().is_some() || self.spaces().iter().all(|x| *x != Empty)
    }

    pub fn spaces(&self) -> &[Space; 9] {
        &self.spaces
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        writeln!(
            f,
            " {} | {} | {}",
            self.spaces[0], self.spaces[1], self.spaces[2]
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {} | {} | {}",
            self.spaces[3], self.spaces[4], self.spaces[5]
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {} | {} | {}",
            self.spaces[6], self.spaces[7], self.spaces[8]
        )?;
        writeln!(f)?;

        Ok(())
    }
}
