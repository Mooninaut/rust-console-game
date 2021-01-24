use text_io::scan;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Square {
    X,
    O,
    Empty,
}

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Square>>
}

fn main() {
    let board_size = 4;
    let mut board = Board::new(board_size);

    let mut is_x_turn = true;
    loop {
        print!("{}", board);

        let row: usize;
        let column: usize;

        scan!("{} {}", column, row);

        if column >= board_size || row >= board_size {
            println!("Out of bounds, try again.");
            continue;
        }

        let selected_square = &mut board.cells[column][row];

        if let Square::Empty = selected_square {
            *selected_square = if is_x_turn { Square::X } else { Square::O };

            is_x_turn = !is_x_turn;

            match board.detect_win() {
                Square::Empty => (),
                x_or_o => {
                    print!("{}", board);
                    println!("{} has won!", x_or_o);
                    std::process::exit(0)
                }
            }

            if board.detect_draw() {
                print!("{}", board);
                println!("A draw! Try again.");
                board = Board::new(board_size);
                is_x_turn = true;
                continue
            }

        } else {
            println!("{}, {} is already occupied. Choose again.", column, row);
        }
    }
}

struct BoardIterator<'a> {
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
        self.column = (self.column as isize + self.column_direction as isize) as usize;
        self.row = (self.row as isize + self.row_direction as isize) as usize;

        Some(square)
    }
}

impl Board {

    pub fn size(&self) -> usize {
        self.cells.len()
    }
    // indexing is [x, y] which is to say [column, row] not [row, column]
    pub fn new(size: usize) -> Board {
        Board { cells: vec![vec![Square::Empty; size]; size] }
    }

    pub fn get_square(&self, column: usize, row: usize) -> Square {
        self.cells[column][row]
    }

    fn iterator(&self, column: usize, row: usize, column_direction: i8, row_direction: i8) -> BoardIterator {
        BoardIterator {
            board: self,
            column,           row,
            column_direction, row_direction
        }
    }

    fn row_iterator(&self, row: usize) -> BoardIterator {
        self.iterator(
            0, row,
            1, 0
        )
    }

    fn column_iterator(&self, column: usize) -> BoardIterator {
        self.iterator(
            column, 0,
            0,      1)
    }

    fn diagonal_iterator(&self, forward: bool) -> BoardIterator {
        if forward {
            self.iterator(
                0, 0,
                1, 1
            )
        } else {
            self.iterator(
                0, self.size() - 1,
                1, -1
            )
        }
    }

    fn is_winning_row(&self, row: usize) -> Option<Square> {
        self.is_winning(&mut self.row_iterator(row))
    }

    fn is_winning(&self, iterator: &mut dyn Iterator<Item = Square>) -> Option<Square> {
        let first = iterator.next().unwrap();

        if first == Square::Empty {
            return None
        }

        for square in iterator {
            if square != first {
                return None
            }
        }

        Some(first)
    }

    fn is_winning_column(&self, column: usize) -> Option<Square> {
        self.is_winning(&mut self.column_iterator(column))
    }

    fn is_winning_forward_diagonal(&self) -> Option<Square> {
        self.is_winning(&mut self.diagonal_iterator(true))
    }

    fn is_winning_reverse_diagonal(&self) -> Option<Square> {
        self.is_winning(&mut self.diagonal_iterator(false))
    }

    pub fn detect_win(&self) -> Square {
        let len = self.cells.len();
        for i in 0..len {
            if let Some(winner) = self.is_winning_row(i) {
                return winner
            }

            if let Some(winner) = self.is_winning_column(i) {
                return winner
            }
        }

        if let Some(winner) = self.is_winning_forward_diagonal() {
            return winner
        }

        if let Some(winner) = self.is_winning_reverse_diagonal() {
            return winner
        }

        Square::Empty
    }

    pub fn detect_draw(&self) -> bool {
        for row in &self.cells {
            for square in row {
                if let Square::Empty = square {
                    return false
                }
            }
        }
        true
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self { Square::X => "X", Square::O => "O", Square::Empty => " "})
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
