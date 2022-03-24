pub fn clear_screen() {
    println!("{}", termion::clear::All)
}

pub fn draw_block_at(x: usize, y: usize, color: String) {
    // println!("{}", termion::cursor::Goto(x as u16, y as u16))
}

pub fn clear_block_at(x: usize, y: usize) {

}