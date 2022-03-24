use std::borrow::BorrowMut;
use crate::drawing::{clear_block_at, draw_block_at};
use crate::stone::Stone;

pub struct TetrisField {
    field: Vec<Vec<bool>>,
    flying_stone: Option<Box<Stone>>,
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
            flying_stone: None,
        }
    }

    fn render_stone(&self) {
        if let Some(flying_stone) = &self.flying_stone {
            for row in 0..4 {
                for column in 0..4 {
                    if flying_stone.block_mask[row][column] {
                        draw_block_at(column, row, flying_stone.color.clone())
                    }
                }
            }
        }
    }

    fn remove_stone(&self) {
        if let Some(flying_stone) = &self.flying_stone {
            for row in 0..4 {
                for column in 0..4 {
                    if flying_stone.block_mask[row][column] {
                        clear_block_at(column, row)
                    }
                }
            }
        }
    }

    pub fn move_stone(&mut self) {
        if let Some(flying_stone) = &self.flying_stone {
            if !self.is_on_ground() {
                self.remove_stone();

                let fs = &mut self.flying_stone;
                fs.unwrap();
                self.render_stone()
            } else {
                for row in 0..4 {
                    for column in 0..4 {
                        if flying_stone.block_mask[row][column] {
                            self.field[flying_stone.y + row][flying_stone.x + column] = true
                        }
                    }
                }
            }
        }
    }

    fn is_on_ground(&self) -> bool {
        if let Some(flying_stone) = &self.flying_stone {
            for row in 0..4 {
                for column in 0..4 {
                    if flying_stone.block_mask[row][column] {
                        if self.field[flying_stone.y + row + 1][flying_stone.x + column] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}