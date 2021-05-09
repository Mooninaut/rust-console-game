use text_io::scan;
use std::fmt;

mod board;
mod game;
mod tictactoe;
mod connectn;

use board::{
    Board,
    Player,
};

use tictactoe::Tictactoe;
use connectn::ConnectN;

use game::{
    Game,
    GameError,
    GameStatus,
};

fn play_tic_tac_toe() {
    println!("Pick a board size: ");
    let board_size: usize;
    {
        scan!("{}", board_size);
    }

    let board = Board::new(board_size);
    let mut game = Tictactoe::new(board);
    play(&mut game);
}

fn play_connect_n() {
    println!("Pick a board size (width height): ");
    let cols: usize;
    let rows: usize;
    {
        scan!("{} {}", cols, rows);
    }

    let mut game = ConnectN::new(cols, rows, 4);
    play(&mut game);
}

fn get_input(length: usize) -> Vec<usize> {
    let row: usize;
    let column: usize;

    match length {
        2 => {
            scan!("{} {}", column, row);
            println!("You played column {}, row {}", column, row);
            return vec![column, row];
        }
        1 => {
            scan!("{}", column);
            println!("You played column {}", column);
            return vec![column];
        }
        _ => unimplemented!("Multidimensional games not supported.")
    }
}

fn play(game: &mut dyn Game) {
    let mut player = Player::X;
    let num_inputs = game.num_inputs();
    loop {
        print!("{}", game);

        let input = get_input(num_inputs);
        match game.play(player, &input) {
            Ok(GameStatus::InProgress) => {
                player = match player {
                    Player::X => Player::O,
                    Player::O => Player::X
                };
            },
            Ok(GameStatus::Drawn) => {
                print!("{}", game);
                println!("A draw! Try again.");
                game.reset();
                player = Player::X;
                continue
            },
            Ok(GameStatus::Won(winner)) => {
                print!("{}", game);
                println!("{} has won!", match winner {
                    Player::X => "X",
                    Player::O => "O"
                });
                std::process::exit(0)
            },
            Err(GameError::SquareNotEmpty) =>
                println!("{} is already occupied. Choose again.", join(", ", input)),
            Err(GameError::OutOfBounds) =>
                println!("{} is out of bounds. Choose again.", join(",", input)),
        }
    }
}

fn join<T: fmt::Display>(separator: &str, vec: Vec<T>) -> String {
    vec.iter().map(T::to_string).collect::<Vec<String>>().join(separator)
}

fn main() {
    println!("Pick a game. 1: Tic-Tac-Toe. 2: ConnectN.");
    let game_choice: usize;
    {
        scan!("{}", game_choice);
    }

    match game_choice {
        1 => play_tic_tac_toe(),
        2 => play_connect_n(),
        _ => panic!("Invalid choice"),
    }
}
