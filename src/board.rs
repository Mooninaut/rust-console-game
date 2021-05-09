use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

// Basically Option<Player>, but can implement Display for it
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Square {
    Played(Player),
    Empty,
}

impl Square {
    pub fn to_option(&self) -> Option<Player> {
        match self {
            Square::Empty => None,
            Square::Played(player) => Some(*player)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<Vec<Square>>
}

#[derive(Debug)]
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
        let columns = self.board.columns();
        let rows = self.board.rows();
        if self.column >= columns || self.row >= rows {
            return None
        }

        let square = self.board.get_square(self.column, self.row);
        self.column = add_usize_i8(self.column, self.column_direction);
        self.row = add_usize_i8(self.row, self.row_direction);

        square
    }
}

#[derive(Debug)]
pub struct DiagonalIteratorIterator<'a> {
    board: &'a Board,
    minimum_size: usize,
    row: usize,
    column: usize,
    forward: bool,
}

impl <'a> DiagonalIteratorIterator<'a> {
    pub fn new(board: &'a Board, minimum_size: usize) -> Option<DiagonalIteratorIterator<'a>> {
        let mut dii = DiagonalIteratorIterator {
            board,
            minimum_size,
            row: board.rows() - 1,
            column: 0,
            forward: true,
        };

        loop {
            match dii.calculate_diagonal_length() {
                None =>
                    return None,
                Some(length) if length >= minimum_size =>
                    return Some(dii),
                Some(_) => {
                    let next = dii.next();

                    if next.is_none() {
                        return None
                    }
                }
            }
        }
    }

    fn calculate_diagonal_length(&self) -> Option<usize> {
        //println!("calculating diagonal length column {}, row {}, board width {}, board height {}",
        //    self.column, self.row, self.board.columns(), self.board.rows());
        let mut total = 0;
        let mut column = self.column;
        let mut row = self.row;
        let columns = self.board.columns();
        let rows = self.board.rows();

        if column >= columns || row >= rows {
            //("Off the board");
            return None
        }
        if self.forward {
            while column < columns && row < rows {
                column += 1;
                row += 1;
                total += 1;
            }
            //println!("Forward: {}", total);
            return Some(total);
        } else {
            while column > 0 && row < rows {
                column -= 1;
                row += 1;
                total += 1;
            }
            if column == 0 && row < rows {
                //println!("Backward: {}", total + 1);
                return Some(total + 1)
            }
            //println!("Backward: {}", total);
            return Some(total)
        }
    }

    fn increment_origin(&mut self) -> bool {
        //println!("incrementing origin {} {}", self.column, self.row);
        if self.column > self.board.columns() || self.row > self.board.rows() {
            //println!("Incrementing failed: Off the board");
            return false
        }

        if self.row > 0 {
            self.row -= 1;
        } else {
            if self.forward {
                self.column += 1;
                if self.column >= self.board.columns() {
                    self.forward = false;
                    self.column -= 1;
                    self.row = self.board.rows() - 1;
                }
            } else {
                if self.column == 0 {
                    self.column = usize::MAX;
                    //println!("Reached 0,0 for the second time, iteration completed.");
                    return false
                }
                self.column -= 1;
            }
        }
        //println!("Incrementing succeeded, new value: {} {}", self.column, self.row);
        true
    }
}

impl <'a> Iterator for DiagonalIteratorIterator<'a> {
    type Item = BoardIterator<'a>;

    fn next(&mut self) -> Option<BoardIterator<'a>> {
        if !self.increment_origin() {
            return None;
        }

        loop {
            match self.calculate_diagonal_length() {
                None =>
                    return None,
                Some(length) => {
                    //println!("Diagonal length is {}, minimum size is {}", length, self.minimum_size);
                    if length >= self.minimum_size {
                        //println!("Retrieving BoardIterator: column {}, row {}", self.column, self.row);
                        let result = Some(BoardIterator {
                            board: self.board,
                            row: self.row,
                            column: self.column,
                            row_direction: 1,
                            column_direction: if self.forward { 1 } else { -1 }
                        });
                        return result;
                    } else if !self.increment_origin() {
                        //println!("End of the line");
                        return None;
                    }
                }
            }
        }

    }
}

impl Board {
    // indexing is [x, y] which is to say [column, row] not [row, column]
    pub fn new(size: usize) -> Board {
        Board { cells: vec![vec![Square::Empty; size]; size] }
    }

    pub fn new_rectangle(columns: usize, rows: usize) -> Board {
        Board { cells: vec![vec![Square::Empty; rows]; columns] }
    }

    pub fn columns(&self) -> usize {
        self.cells.len()
    }

    pub fn rows(&self) -> usize {
        self.cells.get(0).unwrap().len()
    }

    pub fn get_square(&self, column: usize, row: usize) -> Option<Square> {
        if column >= self.columns() || row >= self.rows() {
            None
        } else {
            Some(self.cells[column][row])
        }
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
                column, 0,
                -1, 1
            )
        }
    }

    pub fn diagonal_iterator_iterator(&self, minimum_size: usize) -> Option<DiagonalIteratorIterator> {
        DiagonalIteratorIterator::new(self, minimum_size)
    }

    pub fn reset(&mut self) {
        let size = self.columns();
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
        for row in 0..self.rows() {
            for col in 0..self.columns() {
                write!(f, "[{}] ", self.cells[col][row])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn diagonal_length_1() {
    let board = Board::new(1);
    let di = board.diagonal_iterator_iterator(1).unwrap();
    let length = di.calculate_diagonal_length();
    assert_eq!(length, Some(1));
}
#[test]
fn test_diagonal_iterations() {
    let board = Board::new(2);
    let mut diagonal_iterator_iterator = board.diagonal_iterator_iterator(1).unwrap();
    let mut counter = 0;
    while let Some(_length) = diagonal_iterator_iterator.calculate_diagonal_length() {
        //println!("test_diagonal_iterations(): length = {}", _length);
        diagonal_iterator_iterator.next();
        counter += 1;
    }
    assert_eq!(counter, 6);
}
#[test]
fn diagonal_length_2() {
    let board = Board::new(2);
    let mut diagonal_iterator_iterator = board.diagonal_iterator_iterator(1).unwrap();
    let mut length = diagonal_iterator_iterator.calculate_diagonal_length();
    assert_eq!(length, Some(1));
    diagonal_iterator_iterator.next();
    length = diagonal_iterator_iterator.calculate_diagonal_length();
    assert_eq!(length, Some(2));
    diagonal_iterator_iterator.next();
    length = diagonal_iterator_iterator.calculate_diagonal_length();
    assert_eq!(length, Some(1));
}
#[test]
fn diagonal_length_2a() {
    let board = Board::new_rectangle(2, 3);
    let dii = board.diagonal_iterator_iterator(2).unwrap();
    let length = dii.calculate_diagonal_length();
    assert_eq!(length, Some(2));
}
#[test]
fn diagonal_length_3x() {
    let board = Board::new_rectangle(2, 3);
    assert!(board.diagonal_iterator_iterator(3).is_none());
}
#[test]
fn diagonal_length_3y() {
    let board = Board::new_rectangle(3, 2);
    assert!(board.diagonal_iterator_iterator(3).is_none());
}
