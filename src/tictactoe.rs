use std::fmt;

use crate::board::{Board, Player, Square};

use crate::game::{
    Game,
    GameStatus,
    GameError,
};

pub struct Tictactoe {
    board: Board
}

impl Tictactoe {

    pub fn new(board: Board) -> Tictactoe {
        Tictactoe { board }
    }

    fn is_winning_row(&self, row: usize) -> Option<Player> {
        self.is_winning(&mut self.board.row_iterator(row))
    }

    fn is_winning(&self, iterator: &mut dyn Iterator<Item = Square>) -> Option<Player> {
        let first = iterator.next().unwrap();

        match first {
            Square::Empty => None,
            Square::Played(player) => {
                for square in iterator {
                    if square != first {
                        return None
                    }
                }
                Some(player)
            }
        }
    }

    fn is_winning_column(&self, column: usize) -> Option<Player> {
        self.is_winning(&mut self.board.column_iterator(column))
    }

    fn is_winning_forward_diagonal(&self) -> Option<Player> {
        self.is_winning(&mut self.board.diagonal_iterator(0, true))
    }

    fn is_winning_reverse_diagonal(&self) -> Option<Player> {
        self.is_winning(&mut self.board.diagonal_iterator(self.board.columns() -1, false))
    }

    fn detect_win(&self) -> Option<Player> {
        let len = self.board.columns();
        for i in 0..len {
            if let Some(winner) = self.is_winning_row(i) {
                println!("Winning row {}", i);
                return Some(winner)
            }

            if let Some(winner) = self.is_winning_column(i) {
                println!("Winning column {}", i);
                return Some(winner)
            }
        }

        if let Some(winner) = self.is_winning_forward_diagonal() {
            println!("Winning forward diagonal");
            return Some(winner)
        }

        if let Some(winner) = self.is_winning_reverse_diagonal() {
            println!("Winning reverse diagonal");
            return Some(winner)
        }

        None
    }

    fn detect_draw(&self) -> bool {
        for column in 0..self.board.columns() {
            for row in 0..self.board.columns() {
                if Some(Square::Empty) == self.board.get_square(column, row) {
                    return false
                }
            }
        }
        true
    }
}

impl Game for Tictactoe {

    fn get_status(&self) -> GameStatus {
        if let Some(player) = self.detect_win() {
            GameStatus::Won(player)
        } else if self.detect_draw() {
            GameStatus::Drawn
        } else {
            GameStatus::InProgress
        }
    }

    fn play(&mut self, player: Player, input: &Vec<usize>) -> Result<GameStatus, GameError> {
        let column = input[0];
        let row = input[1];
        match self.board.get_square(column, row) {
            Some(Square::Played(_)) => Err(GameError::SquareNotEmpty),
            Some(Square::Empty) => {
                self.board.set_square(column, row, Square::Played(player));
                Ok(self.get_status())
            },
            None => Err(GameError::OutOfBounds),
        }

    }

    fn reset(&mut self) {
        self.board.reset();
    }

    fn num_inputs(&self) -> usize {
        return 2;
    }
}

impl fmt::Display for Tictactoe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
