use std::io::{stdin, stdout, Write};
use std::ops::Deref;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use crate::drawing::{clear_screen, draw_block_at};
use crate::tetris_field::TetrisField;

pub mod tetris_field;
pub mod stone;
pub mod drawing;

const TICK_DURATION: Duration = Duration::new(0, 200_000_000);

fn main() {
    let mut field = TetrisField::new(10, 30);

    let stdin = stdin();
    let mut out = stdout().into_raw_mode().unwrap();

    clear_screen(&mut out);

    let mut is_running = Arc::new(Mutex::new(true));
    let mut is_running_outer = Arc::clone(&is_running);

    thread::spawn(move || {
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

    while *is_running_outer.lock().unwrap() {
        field.move_stone(&mut out);
        thread::sleep(TICK_DURATION);
    }
}