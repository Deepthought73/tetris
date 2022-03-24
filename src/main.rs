use std::thread;
use std::time::Duration;
use crate::drawing::clear_screen;
use crate::tetris_field::TetrisField;

pub mod tetris_field;
pub mod stone;
pub mod drawing;

fn main() {
    let mut field = TetrisField::new(10, 30);

    clear_screen();

    loop {

    }
}
