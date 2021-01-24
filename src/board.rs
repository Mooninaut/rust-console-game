use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

// Basically Option<Player>, but can implement Display for it
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Square {
    Played(Player),
    Empty,
}

#[derive(Clone)]
pub struct Board {
    cells: Vec<Vec<Square>>
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    row: usize,
    column: usize,
    row_direction: i8,
    column_direction: i8,
}

fn add_usize_i8(u: usize, i: i8) -> usize {
    (u as isize + i as isize) as usize
}

impl Iterator for BoardIterator<'_> {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let size = self.board.size();
        if self.column >= size || self.row >= size {
            return None
        }

        let square = self.board.get_square(self.column, self.row);
        self.column = add_usize_i8(self.column, self.column_direction);
        self.row = add_usize_i8(self.row, self.row_direction);

        Some(square)
    }
}

impl Board {
    // indexing is [x, y] which is to say [column, row] not [row, column]
    pub fn new(size: usize) -> Board {
        Board { cells: vec![vec![Square::Empty; size]; size] }
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn get_square(&self, column: usize, row: usize) -> Square {
        self.cells[column][row]
    }

    pub fn set_square(&mut self, column: usize, row: usize, square: Square) -> &mut Self {
        self.cells[column][row] = square;

        self
    }

    fn iterator(&self, column: usize, row: usize, column_direction: i8, row_direction: i8) -> BoardIterator {
        BoardIterator {
            board: self,
            column,           row,
            column_direction, row_direction
        }
    }

    pub fn row_iterator(&self, row: usize) -> BoardIterator {
        self.iterator(
            0, row,
            1, 0
        )
    }

    pub fn column_iterator(&self, column: usize) -> BoardIterator {
        self.iterator(
            column, 0,
            0,      1)
    }

    pub fn diagonal_iterator(&self, column: usize, forward: bool) -> BoardIterator {
        if forward {
            self.iterator(
                column, 0,
                1, 1
            )
        } else {
            self.iterator(
                column, self.size() - 1,
                1, -1
            )
        }
    }

    pub fn reset(&mut self) {
        let size = self.size();
        for column in 0..size {
            for row in 0..size {
                self.set_square(column, row, Square::Empty);
            }
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self { Square::Played(Player::X) => "X", Square::Played(Player::O) => "O", Square::Empty => " "})
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.cells.len() {
            for col in 0..self.cells.len() {
                write!(f, "[{}] ", self.cells[col][row])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
