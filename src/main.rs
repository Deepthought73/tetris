use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::tetris_field::TetrisField;
use termion::event::{Key, Event};
use termion::input::TermRead;
use crate::drawing::Drawing;

pub mod tetris_field;
pub mod stone;
pub mod drawing;

const TICK_DURATION: Duration = Duration::new(0, 200_000_000);

fn main() {
    let width = 10;
    let height = 20;

    let mut field = TetrisField::new(width, height);

    let mut drawing = Drawing::new(width, height);

    drawing.hide_cursor();
    drawing.clear_screen();
    drawing.draw_border();

    let is_running = Arc::new(Mutex::new(true));
    let is_running_main = Arc::clone(&is_running);

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) => {
                    *is_running.lock().unwrap() = false;
                    break
                },
                _ => {}
            }
        }
    });

    while *is_running_main.lock().unwrap() {
        // field.move_stone(&mut out);
        thread::sleep(TICK_DURATION);
    }

    drawing.clear_screen();
    drawing.show_cursor();
}