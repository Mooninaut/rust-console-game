use std::fmt;

use crate::board::{
    Player,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Won(Player),
    Drawn,
    InProgress,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameError {
    SquareNotEmpty,
    OutOfBounds,
}

pub trait Game : fmt::Display {
    fn get_status(&self) -> GameStatus;
    fn play(&mut self, player: Player, input: &Vec<usize>) -> Result<GameStatus, GameError>;
    fn reset(&mut self);
    fn num_inputs(&self) -> usize;
}
