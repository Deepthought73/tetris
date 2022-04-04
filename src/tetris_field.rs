use rand::Rng;
use termion::color;
use termion::color::Rgb;
use termion::event::Key::Null;
use crate::Drawing;
use crate::stone::Stone;

pub struct TetrisField {
    field: Vec<Vec<bool>>,
    color_matrix: Vec<Vec<Option<Rgb>>>,
    flying_stone: Stone,
}

impl TetrisField {
    pub fn new(width: usize, height: usize) -> TetrisField {
        let mut field = vec![];
        let mut color_matrix = vec![];
        for _ in 0..height {
            let mut row = vec![];
            let mut color_row = vec![];
            for _ in 0..width {
                row.push(false);
                color_row.push(None);
            }
            field.push(row);
            color_matrix.push(color_row);
        }
        TetrisField {
            field,
            color_matrix,
            flying_stone: TetrisField::generate_next_stone(0, 0),
        }
    }

    pub fn draw_block_at(&mut self, drawing: &mut Drawing, x:usize, y:usize, color: Rgb) {
        drawing.draw_block_at(
            x,
            y,
            *Box::from(color)
        );
        self.color_matrix[y][x] = Some(color);
    }

    pub fn clear_block_at(&mut self, drawing: &mut Drawing, x:usize, y:usize) {
        drawing.clear_block_at(
            x,
            y,
        );
        self.color_matrix[y][x] = None;
    }

    pub fn render_stone(&mut self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    self.draw_block_at(
                        drawing,
                        self.flying_stone.x + column,
                        self.flying_stone.y + row,
                        *Box::from(self.flying_stone.color.clone())
                    );
                }
            }
        }
    }

    fn remove_stone(&mut self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    self.clear_block_at(
                        drawing,
                        self.flying_stone.x + column,
                        self.flying_stone.y + row,
                    )
                }
            }
        }
    }

    pub fn move_stone(&mut self, drawing: &mut Drawing) {
        if !self.is_on_ground(&self.flying_stone) {
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

    pub fn jump_one_step(&mut self, drawing: &mut Drawing) {
        self.move_stone(drawing);
        self.move_stone(drawing);
    }

    pub fn jump_to_ground(&mut self, drawing: &mut Drawing) {
        self.remove_stone(drawing);
        while !self.is_on_ground(&self.flying_stone) {
            self.flying_stone.y += 1
        }
        self.render_stone(drawing);
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
                return false;
            }
        }
        true
    }

    fn clear_row(&mut self, row: usize, drawing: &mut Drawing) {
        for column in 0..self.field[0].len() {
            self.clear_block_at(
                drawing,
                column,
                row,
            );
        }
        for index in (1..row+1).rev() {
            for column in 0..self.field[0].len() {
                if self.field[index - 1][column] {
                    if let Some(color) = self.color_matrix[index - 1][column] {
                        self.draw_block_at(drawing, column, index, color)
                    }
                } else {
                    self.clear_block_at(drawing, column, index)
                }
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
        if !self.is_on_ground(&self.flying_stone) {
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

    fn is_on_ground(&self, stone: &Stone) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if stone.block_mask()[row][column] {
                    if stone.x + column < self.field.first().unwrap().len() - 1 {
                        if stone.y + row + 1 >= self.field.len() ||
                            self.field[stone.y + row + 1][stone.x + column] {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}