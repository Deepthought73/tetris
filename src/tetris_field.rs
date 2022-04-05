use rand::Rng;
use termion::color::Rgb;
use crate::Drawing;
use crate::stone::Stone;

pub struct TetrisField {
    field: Vec<Vec<bool>>,
    color_matrix: Vec<Vec<Option<Rgb>>>,
    pub flying_stone: Stone,
    preview: Stone,
    score: usize,
    is_game_over: bool,
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
        let flying_stone = TetrisField::generate_next_stone();
        let preview = flying_stone.clone();
        TetrisField {
            field,
            color_matrix,
            flying_stone,
            preview,
            score: 0,
            is_game_over: false,
        }
    }

    pub fn draw_block_at(&mut self, drawing: &mut Drawing, x: usize, y: usize, color: Rgb) {
        drawing.draw_block_at(
            x,
            y,
            *Box::from(color),
        );
        self.color_matrix[y][x] = Some(color);
    }

    pub fn clear_block_at(&mut self, drawing: &mut Drawing, x: usize, y: usize) {
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
                        *Box::from(self.flying_stone.color.clone()),
                    );
                }
            }
        }
    }

    pub fn render_preview(&mut self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                if self.flying_stone.block_mask()[row][column] {
                    self.draw_block_at(
                        drawing,
                        self.preview.x + column,
                        self.preview.y + row,
                        *Box::from(self.flying_stone.color.clone()),
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

    fn remove_preview(&mut self, drawing: &mut Drawing) {
        for row in 0..4 {
            for column in 0..4 {
                //self.draw_block_at(drawing, self.preview.x, self.preview.y, Rgb(100,100,100));
                if self.flying_stone.block_mask()[row][column] {
                    self.clear_block_at(
                        drawing,
                        self.preview.x + column,
                        self.preview.y + row,
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
            self.update_preview(drawing);
        } else {
            let mut cleared = 0;
            for row in 0..4 {
                for column in 0..4 {
                    if self.flying_stone.block_mask()[row][column] {
                        self.field[self.flying_stone.y + row][self.flying_stone.x + column] = true
                    }
                }
                if self.flying_stone.y + row < self.field.len() {
                    if self.row_is_complete(self.flying_stone.y + row) {
                        cleared += 1;
                        self.clear_row(self.flying_stone.y + row, drawing)
                    }
                }
            }
            self.flying_stone = TetrisField::generate_next_stone();
            if self.has_collision() {
                self.is_game_over = true;
            }
            self.preview = self.flying_stone.clone();
            if cleared > 0 {
                self.score += match cleared {
                    1 => 100,
                    2 => 400,
                    3 => 800,
                    _ => 1600
                };
                drawing.draw_score(self.score);
            }
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

    fn generate_next_stone() -> Stone {
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
            0 => Stone::i(4, 0, color),
            1 => Stone::j(4, 0, color),
            2 => Stone::l(4, 0, color),
            3 => Stone::o(4, 0, color),
            4 => Stone::s(4, 0, color),
            5 => Stone::t(4, 0, color),
            _ => Stone::z(4, 0, color),
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
        for index in (1..row + 1).rev() {
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

    pub fn update_preview(&mut self, drawing: &mut Drawing) {
        if !self.has_preview_collision() {
            self.remove_preview(drawing);
        }
        self.preview.x = self.flying_stone.x;
        self.preview.y = self.flying_stone.y;
        while !self.is_on_ground(&self.preview) {
            self.preview.y += 1;
        }
        self.render_preview(drawing);
    }

    pub fn move_stone_right(&mut self, drawing: &mut Drawing) {
        if !self.collision_right() {
            self.remove_stone(drawing);
            self.flying_stone.x += 1;
            self.render_stone(drawing);
            self.update_preview(drawing);
        }
    }

    pub fn move_stone_left(&mut self, drawing: &mut Drawing) {
        if !self.collision_left() {
            self.remove_stone(drawing);
            self.flying_stone.x -= 1;
            self.render_stone(drawing);
            self.update_preview(drawing);
        }
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

    fn has_preview_collision(&self) -> bool {
        if (self.flying_stone.y + self.preview.y) > 4 {
            return false;
        }
        true
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
            if !self.has_preview_collision() {
                self.remove_preview(drawing);
            }

            self.remove_stone(drawing);
            self.flying_stone.rotate();
            if self.has_collision() {
                if self.flying_stone.x > 4 {
                    self.flying_stone.x -= 1;
                } else if self.flying_stone.x < 4 {
                    self.flying_stone.x += 1;
                }
                if self.has_collision() {
                    if self.flying_stone.x > 4 {
                        self.flying_stone.x += 1;
                    } else if self.flying_stone.x < 4 {
                        self.flying_stone.x -= 1;
                    }
                    self.flying_stone.derotate();
                    self.preview.derotate();
                }
            }
            self.preview = self.flying_stone.clone();
            self.update_preview(drawing);
            self.render_stone(drawing);
        }
    }

    fn is_on_ground(&self, stone: &Stone) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                if stone.block_mask()[row][column] {
                    if stone.x + column < self.field.first().unwrap().len() {
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

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn game_over(&self) -> bool {
        self.is_game_over
    }
}