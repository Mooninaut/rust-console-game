use std::fmt;

use crate::board::{Board, BoardIterator, Player, Square};

use crate::game::{
    Game,
    GameStatus,
    GameError,
};

pub struct ConnectN {
    win_length: usize,
    board: Board,
}

impl ConnectN {
    pub fn new(columns: usize, rows: usize, win_length: usize) -> ConnectN {
        ConnectN { win_length, board: Board::new_rectangle(columns, rows) }
    }

    fn incremental_count(current: Square, last: Square, count: usize) -> usize {
        match current {
            Square::Empty => 0,
            Square::Played(_) => {
                if last == current {
                    count + 1
                } else {
                    1
                }
            }
        }
    }

    fn detect_winning_line(&self, iterator: &mut BoardIterator, to_win: usize) -> Option<Player> {
        let mut last = Square::Empty;
        let mut count = 0;
        while let Some(square) = iterator.next() {
            count = Self::incremental_count(square, last, count);
            if count >= to_win {
                return square.to_option()
            }
            last = square;
        }

        None
    }

    fn detect_win(&self) -> Option<Player> {
        let mut diagonal_iterator_iterator = self.board.diagonal_iterator_iterator(self.win_length).unwrap();

        while let Some(mut diagonal_iterator) = diagonal_iterator_iterator.next() {
            if let Some(player) = self.detect_winning_line(&mut diagonal_iterator, self.win_length) {
                return Some(player);
            }
        }

        for row in 0..self.board.rows() {
            let mut horizontal_iterator = self.board.row_iterator(row);
            if let Some(player) = self.detect_winning_line(&mut horizontal_iterator, self.win_length) {
                return Some(player);
            }
        }

        for column in 0..self.board.columns() {
            let mut vertical_iterator = self.board.column_iterator(column);
            if let Some(player) = self.detect_winning_line(&mut vertical_iterator, self.win_length) {
                return Some(player);
            }
        }

        None
    }

    fn play(&mut self, column: usize, player: Player) {
        let mut row = 0;
        while self.board.get_square(column, row) == Some(Square::Empty) {
            //println!("Checking row {}", row);
            row += 1;
        }
        row -= 1;

        self.board.set_square(column, row, Square::Played(player));
    }

    fn detect_draw(&self) -> bool {
        let mut row_iterator = self.board.row_iterator(0);
        loop {
            match row_iterator.next() {
                None => return true,
                Some(Square::Empty) => return false,
                Some(Square::Played(_)) => (),
            }
        }
    }
}

impl Game for ConnectN {

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
        if column >= self.board.columns() {
            return Err(GameError::OutOfBounds)
        }

        return match self.board.get_square(column, 0) {
            Some(Square::Played(_)) => Err(GameError::SquareNotEmpty),
            Some(Square::Empty) => {
                self.play(column, player);
                Ok(self.get_status())
            },
            None => Err(GameError::OutOfBounds),
        }
    }

    fn reset(&mut self) {
        self.board.reset();
    }

    fn num_inputs(&self) -> usize {
        return 1;
    }
}

impl fmt::Display for ConnectN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}
