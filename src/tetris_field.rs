use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::Stdout;
use termion::raw::RawTerminal;
use crate::drawing::{clear_block_at, draw_block_at};
use crate::stone::Stone;

pub struct TetrisField {
    field: Vec<Vec<bool>>,
    flying_stone: Stone,
}

impl TetrisField {
    pub fn new(width: usize, height: usize) -> TetrisField {
        let mut field = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(false)
            }
            field.push(row)
        }
        TetrisField {
            field,
            flying_stone: Stone::i(0, 0, "".to_string()),
        }
    }

    fn render_stone(&self, stdout: &mut RawTerminal<Stdout>) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask[row][column] {
                    draw_block_at(stdout,column, row, self.flying_stone.color.clone())
                }
            }
        }
    }

    fn remove_stone(&self) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask[row][column] {
                    clear_block_at(column, row)
                }
            }
        }
    }

    pub fn move_stone(&mut self) {
        if !self.is_on_ground() {
            self.remove_stone();

            self.flying_stone.y += 1;
            println!("{}", self.flying_stone.y);

            self.render_stone()
        } else {
            for row in 0..4 {
                for column in 0..4 {
                    if self.flying_stone.block_mask[row][column] {
                        self.field[self.flying_stone.y + row][self.flying_stone.x + column] = true
                    }
                }
            }
        }
    }

    fn is_on_ground(&self) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask[row][column] {
                    if self.flying_stone.y + row + 2 >= self.field.len() || self.field[self.flying_stone.y + row + 1][self.flying_stone.x + column] {
                        return true;
                    }
                }
            }
        }
        false
    }
}