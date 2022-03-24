use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

pub fn clear_screen(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    stdout.flush().unwrap();
}

pub fn draw_block_at(stdout: &mut RawTerminal<Stdout>, x: usize, y: usize, color: String) {
    draw_char_at(stdout, x, y, "X");
}

pub fn clear_block_at(stdout: &mut RawTerminal<Stdout>, x: usize, y: usize) {
    draw_char_at(stdout, x, y, " ")
}

fn draw_char_at(stdout: &mut RawTerminal<Stdout>, x: usize, y: usize, seq: &str) {
    write!(stdout, "{}{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1), seq);
    stdout.flush().unwrap();
}
