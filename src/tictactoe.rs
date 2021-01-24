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
        self.is_winning(&mut self.board.diagonal_iterator(0, false))
    }

    fn detect_win(&self) -> Option<Player> {
        let len = self.board.size();
        for i in 0..len {
            if let Some(winner) = self.is_winning_row(i) {
                return Some(winner)
            }

            if let Some(winner) = self.is_winning_column(i) {
                return Some(winner)
            }
        }

        if let Some(winner) = self.is_winning_forward_diagonal() {
            return Some(winner)
        }

        if let Some(winner) = self.is_winning_reverse_diagonal() {
            return Some(winner)
        }

        None
    }

    fn detect_draw(&self) -> bool {
        for column in 0..self.board.size() {
            for row in 0..self.board.size() {
                if let Square::Empty = self.board.get_square(column, row) {
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

    fn play(&mut self, player: Player, column: usize, row: usize) -> Result<GameStatus, GameError> {
        if column >= self.board.size() || row >= self.board.size() {
            return Err(GameError::OutOfBounds)
        }
        match self.board.get_square(column, row) {
            Square::Played(_) => Err(GameError::SquareNotEmpty),
            Square::Empty => {
                self.board.set_square(column, row, Square::Played(player));
                Ok(self.get_status())
            }
        }

    }

    fn reset(&mut self) {
        self.board.reset();
    }
}

impl fmt::Display for Tictactoe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
