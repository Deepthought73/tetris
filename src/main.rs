use std::borrow::BorrowMut;
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

const TICK_DURATION: Duration = Duration::new(0, 500_000_000);

fn main() {
    let width = 10;
    let height = 20;

    let field = TetrisField::new(width, height);
    let field = Arc::new(Mutex::from(field));

    let drawing = Drawing::new(width, height);
    let drawing = Arc::new(Mutex::from(drawing));

    drawing.lock().unwrap().hide_cursor();
    drawing.lock().unwrap().clear_screen();
    drawing.lock().unwrap().draw_border();
    drawing.lock().unwrap().draw_score(0);

    let is_running = Arc::new(Mutex::new(true));
    let is_running_main = Arc::clone(&is_running);

    let field_copy = Arc::clone(&field);
    let drawing_copy = Arc::clone(&drawing);

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Char('q')) => {
                    *is_running.lock().unwrap() = false;
                    break;
                }
                Event::Key(Char('a')) | Event::Key(Char('A')) | Event::Key(Key::Left) => {
                    field.lock().unwrap().move_stone_left(
                        drawing.lock().unwrap().borrow_mut()
                    );
                }
                Event::Key(Char('d')) | Event::Key(Char('D')) | Event::Key(Key::Right) => {
                    field.lock().unwrap().move_stone_right(
                        drawing.lock().unwrap().borrow_mut()
                    );
                }
                Event::Key(Char('w')) | Event::Key(Char('W')) | Event::Key(Key::Up) => {
                    field.lock().unwrap().rotate(drawing.lock().unwrap().borrow_mut());
                }
                Event::Key(Char('s')) | Event::Key(Char('S')) | Event::Key(Key::Down) => {
                    field.lock().unwrap().jump_one_step(drawing.lock().unwrap().borrow_mut());
                }
                Event::Key(Char(' ')) => {
                    field.lock().unwrap().jump_to_ground(drawing.lock().unwrap().borrow_mut());
                }
                _ => {}
            }
        }
    });

    field_copy.lock().unwrap().render_stone(drawing_copy.lock().unwrap().borrow_mut());
    thread::sleep(TICK_DURATION);
    while *is_running_main.lock().unwrap() {
        if field_copy.lock().unwrap().game_over() {
            drawing_copy.lock().unwrap().draw_game_over();
        } else {
            field_copy.lock().unwrap().move_stone(drawing_copy.lock().unwrap().borrow_mut());
        }

        thread::sleep(TICK_DURATION);
    }

    drawing_copy.lock().unwrap().clear_screen();
    drawing_copy.lock().unwrap().show_cursor();

    println!(
        "{}Score was: {}{}",
        termion::color::Fg(termion::color::Reset),
        field_copy.lock().unwrap().score(),
        termion::cursor::Goto(0, 1)
    );
    println!();
}