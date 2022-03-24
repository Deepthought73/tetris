use termion::color;

pub struct Stone {
    pub x: usize,
    pub y: usize,
    pub block_mask: [[bool; 4]; 4],
    pub color: String
}

impl Stone {
    fn i(x: usize, y: usize, color: String) -> Stone {
        Stone {
            x, y,
            block_mask: [
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
                [true, false, false, false],
            ],
            color
        }
    }
}