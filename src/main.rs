use text_io::scan;

mod board;
mod game;
mod tictactoe;

use board::{
    Board,
    Player,
};

use tictactoe::Tictactoe;

use game::{
    Game,
    GameError,
    GameStatus,
};

fn main() {
    println!("Pick a game. 1: Tic-Tac-Toe."); //  2: Connect-4.
    let game_choice: usize;
    {
    scan!("{}", game_choice);
    assert_eq!(game_choice, 1);
    }

    println!("Pick a board size: ");
    let board_size: usize;
    {
        scan!("{}", board_size);
    }

    let board = Board::new(board_size);
    let mut game = Tictactoe::new(board);

    let mut player = Player::X;
    loop {
        print!("{}", game);

        let row: usize;
        let column: usize;

        scan!("{} {}", column, row);

        match game.play(player, column, row) {
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
                println!("{}, {} is already occupied. Choose again.", column, row),
            Err(GameError::OutOfBounds) =>
                println!("{}, {} is out of bounds. Choose again.", column, row)
        }
    }
}
