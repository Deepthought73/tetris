use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::tetris_field::TetrisField;
use termion::event::{Key, Event};
use termion::event::Key::Char;
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
                Event::Key(Char('q')) => {
                    *is_running.lock().unwrap() = false;
                    break;
                }
                Event::Key(Char('a')) | Event::Key(Char('A')) | Event::Key(Key::Left) => {}
                Event::Key(Char('d')) | Event::Key(Char('D')) | Event::Key(Key::Right) => {}
                Event::Key(Char('w')) | Event::Key(Char('W')) | Event::Key(Key::Up) => {}
                Event::Key(Char('s')) | Event::Key(Char('S')) | Event::Key(Key::Down) => {}
                _ => {}
            }
        }
    });

    field.render_stone(&mut drawing);
    thread::sleep(TICK_DURATION);
    while *is_running_main.lock().unwrap() {
        field.move_stone(&mut drawing);
        thread::sleep(TICK_DURATION);
    }

    drawing.clear_screen();
    drawing.show_cursor();
}