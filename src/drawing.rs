pub fn clear_screen() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{} test", termion::clear::All, termion::cursor::Goto(1, 1));
    stdout.flush().unwrap();
}

pub fn draw_block_at(x: usize, y: usize, color: String) {
    // println!("{}", termion::cursor::Goto(x as u16, y as u16))
}

pub fn clear_block_at(x: usize, y: usize) {

}