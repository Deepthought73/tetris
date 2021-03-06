use std::borrow::Borrow;
use std::io::{stdout, Stdout, Write};
use termion::{clear, color, cursor};
use termion::color::Rgb;
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

    pub fn draw_score(&mut self, score: usize) {
        write!(self.out, "{}Score: {}", cursor::Goto((self.root_x + self.field_width + 3) as u16, (self.root_y + 1) as u16), score).unwrap();
    }

    pub fn draw_border(&mut self) {
        for i in 0..self.field_height {
            self.draw_char_at(
                self.root_x - 1,
                self.root_y + i,
                "┃",
                Box::from(color::Reset),
            );
            self.draw_char_at(
                self.root_x + self.field_width,
                self.root_y + i,
                "┃",
                Box::from(color::Reset),
            )
        }

        for i in 0..self.field_width {
            self.draw_char_at(
                self.root_x + i,
                self.root_y - 1,
                "━",
                Box::from(color::Reset),
            );
            self.draw_char_at(
                self.root_x + i,
                self.root_y + self.field_height,
                "━",
                Box::from(color::Reset),
            )
        }

        self.draw_char_at(self.root_x - 1, self.root_y - 1, "┏", Box::from(color::Reset));
        self.draw_char_at(self.root_x + self.field_width, self.root_y - 1, "┓", Box::from(color::Reset));
        self.draw_char_at(self.root_x - 1, self.root_y + self.field_height, "┗", Box::from(color::Reset));
        self.draw_char_at(self.root_x + self.field_width, self.root_y + self.field_height, "┛", Box::from(color::Reset));

        write!(
            self.out, "{}Tetris",
            termion::cursor::Goto((self.root_x + self.field_width / 2 - 2) as u16, self.root_y as u16)
        ).unwrap();

        for y in 0..self.field_height {
            for x in 0..(self.field_width / 2) {
                self.clear_block_at(x, y);
            }
        }
    }

    pub fn hide_cursor(&mut self) {
        write!(self.out, "{}", cursor::Hide).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.out, "{}", cursor::Show).unwrap();
    }

    pub fn clear_screen(&mut self) {
        write!(self.out, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        self.out.flush().unwrap();
    }

    pub fn draw_block_at(&mut self, x: usize, y: usize, block_color: color::Rgb) {
        self.draw_char_at(
            self.root_x + x * 2,
            self.root_y + y,
            "██",
            Box::from(block_color),
        );
    }

    pub fn clear_block_at(&mut self, x: usize, y: usize) {
        self.draw_char_at(
            self.root_x + x * 2,
            self.root_y + y,
            " .",
            Box::from(color::Reset),
        )
    }

    pub fn draw_game_over(&mut self) {
        let game_over_string = vec![
            r"                                                                         ",
            r"░██████╗░░█████╗░███╗░░░███╗███████╗    ░█████╗░██╗░░░██╗███████╗██████╗░",
            r"██╔════╝░██╔══██╗████╗░████║██╔════╝    ██╔══██╗██║░░░██║██╔════╝██╔══██╗",
            r"██║░░██╗░███████║██╔████╔██║█████╗░░    ██║░░██║╚██╗░██╔╝█████╗░░██████╔╝",
            r"██║░░╚██╗██╔══██║██║╚██╔╝██║██╔══╝░░    ██║░░██║░╚████╔╝░██╔══╝░░██╔══██╗",
            r"╚██████╔╝██║░░██║██║░╚═╝░██║███████╗    ╚█████╔╝░░╚██╔╝░░███████╗██║░░██║",
            r"░╚═════╝░╚═╝░░╚═╝╚═╝░░░░░╚═╝╚══════╝    ░╚════╝░░░░╚═╝░░░╚══════╝╚═╝░░╚═╝",
            r"                                                                         ",
        ];

        let size = termion::terminal_size().unwrap();
        let root_x = (size.0 - game_over_string.first().unwrap().len() as u16) / 2;
        let root_y = (size.1 - game_over_string.len() as u16) / 2;

        for (j, row) in game_over_string.iter().enumerate() {
            for (i, chr) in row.chars().enumerate() {
                write!(
                    self.out,
                    "{}{}{}",
                    color::Fg(Rgb(255, 50, 50)),
                    cursor::Goto(root_x + i as u16 + 2, root_y + j as u16 + 1),
                    chr
                ).unwrap();
            }
        }
    }

    fn draw_char_at(&mut self, x: usize, y: usize, seq: &str, block_color: Box<dyn color::Color>) {
        write!(self.out, "{}{}{}",
               cursor::Goto(
                   x as u16 + 1,
                   y as u16 + 1,
               ),
               color::Fg(block_color.borrow()),
               seq).unwrap();
        self.out.flush().unwrap();
    }
}