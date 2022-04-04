use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use termion::color;
use termion::color::Rgb;
use crate::{Drawing, tetris_field};
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
            flying_stone: Stone::t(0, 0, color::Rgb(200, 100, 0)),
        }
    }

    pub fn render_stone(&self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    drawing.draw_block_at(
                        self.flying_stone.x + column,
                        self.flying_stone.y + row,
                        *Box::from(self.flying_stone.color.clone()),
                    )
                }
            }
        }
    }

    fn remove_stone(&self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    drawing.clear_block_at(
                        self.flying_stone.x + column,
                        self.flying_stone.y + row,
                    )
                }
            }
        }
    }

    pub fn move_stone(&mut self, drawing: &mut Drawing) {
        if !self.is_on_ground() {
            self.remove_stone(drawing);
            self.flying_stone.y += 1;
            self.render_stone(drawing);
        } else {
            for row in 0..4 {
                for column in 0..4 {
                    if self.flying_stone.block_mask()[row][column] {
                        self.field[self.flying_stone.y + row][self.flying_stone.x + column] = true
                    }
                }
                if self.flying_stone.y + row < self.field.len() {
                    if self.row_is_complete(self.flying_stone.y + row) {
                        self.clear_row(self.flying_stone.y + row, drawing)
                    }
                }
            }
            self.flying_stone = TetrisField::generate_next_stone(0, 0);
        }
    }

    fn generate_next_stone(x: usize, y: usize) -> Stone {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..=1) * 255;
        let g = rng.gen_range(0..=1) * 255;
        let b =
            if r == 255 && g == 255 {
                0
            } else if r == 0 && g == 0 {
                255
            } else {
                rng.gen_range(0..=1) * 255
            };
        let color = Rgb(r, g, b);
        match rng.gen_range(0..7) {
            0 => Stone::i(x, y, color),
            1 => Stone::j(x, y, color),
            2 => Stone::l(x, y, color),
            3 => Stone::o(x, y, color),
            4 => Stone::s(x, y, color),
            5 => Stone::t(x, y, color),
            _ => Stone::z(x, y, color),
        }
    }

    fn row_is_complete(&mut self, row: usize) -> bool {
        for column in 0..self.field[0].len() {
            if !self.field[row][column] {
                return false
            }
        }
        true
    }

    fn clear_row(&mut self, row: usize, drawing: &mut Drawing) {
        for column in 0..self.field[0].len() {
            drawing.clear_block_at(
                column,
                row,
            );
            self.field[row][column] = false
        }
        for index in (1..row).rev() {
            for column in 0..self.field[index].len() {
                self.field[index][column] = self.field[index - 1][column]
            }
        }
    }

    pub fn move_stone_right(&mut self, drawing: &mut Drawing) {
        self.remove_stone(drawing);
        if !self.collision_right() {
            self.flying_stone.x += 1;
        }
        self.render_stone(drawing)
    }

    pub fn move_stone_left(&mut self, drawing: &mut Drawing) {
        self.remove_stone(drawing);
        if !self.collision_left() {
            self.flying_stone.x -= 1;
        }
        self.render_stone(drawing)
    }

    fn collision_right(&self) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    if self.flying_stone.x + column >= self.field.first().unwrap().len() - 1 {
                        return true;
                    }
                    if self.field[self.flying_stone.y + row][self.flying_stone.x + column + 1] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn collision_left(&self) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    if self.flying_stone.x <= 0 {
                        return true;
                    }
                    if self.field[self.flying_stone.y + row][self.flying_stone.x + column - 1] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn has_collision(&self) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    if self.flying_stone.x + column > self.field.first().unwrap().len() - 1 {
                        return true;
                    }
                    if self.flying_stone.y + row > self.field.len() - 1 {
                        return true;
                    }
                    if self.field[self.flying_stone.y + row][self.flying_stone.x + column] {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn rotate(&mut self, drawing: &mut Drawing) {
        if !self.is_on_ground() {
            self.remove_stone(drawing);
            self.flying_stone.rotate();
            if self.has_collision() {
                self.flying_stone.rotate();
                self.flying_stone.rotate();
                self.flying_stone.rotate();
            }
            self.render_stone(drawing);
        }
    }

    fn is_on_ground(&self) -> bool {
        let block_mask = self.flying_stone.block_mask();
        for row in 0..4 {
            for column in 0..4 {
                if block_mask[row][column] {
                    if self.flying_stone.x + column < self.field.first().unwrap().len() - 1 {
                        if self.flying_stone.y + row + 1 >= self.field.len() ||
                            self.field[self.flying_stone.y + row + 1][self.flying_stone.x + column] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}