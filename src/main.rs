use crate::drawing::clear_screen;
use crate::tetris_field::TetrisField;

pub mod tetris_field;
pub mod stone;
pub mod drawing;

fn main() {
    let field = TetrisField::new(10, 30);

    clear_screen();

    loop {
        field.move_stone()
    }
}
