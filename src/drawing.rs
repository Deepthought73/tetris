use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Drawing {
    out: RawTerminal<Stdout>,
    root_x: usize,
    root_y: usize,
    field_width: usize,
    field_height: usize,
}

impl Drawing {
    pub fn new(field_width: usize, field_height: usize) -> Drawing {
        let size = termion::terminal_size().unwrap();
        Drawing {
            out: stdout().into_raw_mode().unwrap(),
            root_x: size.0 as usize / 2 - field_width,
            root_y: (size.1 as usize - field_height) / 2,
            field_width: field_width * 2,
            field_height,
        }
    }

    pub fn draw_border(&mut self) {
        for i in 0..self.field_height {
            self.draw_char_at(
                self.root_x - 1,
                self.root_y + i,
                "┃",
            );
            self.draw_char_at(
                self.root_x + self.field_width,
                self.root_y + i,
                "┃",
            )
        }

        for i in 0..self.field_width {
            self.draw_char_at(
                self.root_x + i,
                self.root_y - 1,
                "━",
            );
            self.draw_char_at(
                self.root_x + i,
                self.root_y + self.field_height,
                "━",
            )
        }

        self.draw_char_at(self.root_x - 1, self.root_y - 1, "┏");
        self.draw_char_at(self.root_x + self.field_width, self.root_y - 1, "┓");
        self.draw_char_at(self.root_x - 1, self.root_y + self.field_height, "┗");
        self.draw_char_at(self.root_x + self.field_width, self.root_y + self.field_height, "┛");
    }

    pub fn hide_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.out, "{}", termion::cursor::Show).unwrap();
    }

    pub fn clear_screen(&mut self) {
        write!(self.out, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
        self.out.flush().unwrap();
    }

    pub fn draw_block_at(&mut self, x: usize, y: usize, color: String) {
        self.draw_char_at(x, y, "X");
    }

    pub fn clear_block_at(&mut self, x: usize, y: usize) {
        self.draw_char_at(x, y, " ")
    }

    fn draw_char_at(&mut self, x: usize, y: usize, seq: &str) {
        write!(self.out, "{}{}",
               termion::cursor::Goto(
                   x as u16 + 1,
                   y as u16 + 1,
               ), seq).unwrap();
        self.out.flush().unwrap();
    }
}