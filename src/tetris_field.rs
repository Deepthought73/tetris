use termion::color;
use crate::Drawing;
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
            flying_stone: Stone::t(0, 0, color::Rgb(100, 100, 0)),
        }
    }

    pub(crate) fn render_stone(&self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask[row][column] {
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
                if self.flying_stone.block_mask[row][column] {
                    drawing.clear_block_at(
                        self.flying_stone.x + column,
                        self.flying_stone.y + row,
                    )
                }
            }
        }
    }

    pub fn move_stone(&mut self, drawing: &mut Drawing) {
        return
        if !self.is_on_ground() {
            self.remove_stone(drawing);
            self.flying_stone.y += 1;
            self.render_stone(drawing)
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
                    if self.flying_stone.y + row + 1 >= self.field.len() ||
                        self.field[self.flying_stone.y + row + 1][self.flying_stone.x + column] {
                        return true;
                    }
                }
            }
        }
        false
    }
}