use termion::color;

#[derive(Clone)]
pub struct Stone {
    pub x: usize,
    pub y: usize,
    pub block_mask: [[bool; 4]; 4],
    pub color: String
}

impl Stone {
    pub fn i(x: usize, y: usize, color: String) -> Stone {
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

    pub fn l(x: usize, y: usize, color: String) -> Stone {
        Stone {
            x, y,
            block_mask: [
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
            ],
            color
        }
    }

    pub fn square(x: usize, y: usize, color: String) -> Stone {
        Stone {
            x, y,
            block_mask: [
                [false, false, false, false],
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
            ],
            color
        }
    }

    pub fn rev_l(x: usize, y: usize, color: String) -> Stone {
        Stone {
            x, y,
            block_mask: [
                [false, false, false, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, true, true, false],
            ],
            color
        }
    }

    pub fn triangle(x: usize, y: usize, color: String) -> Stone {
        Stone {
            x, y,
            block_mask: [
                [false, false, false, false],
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
            ],
            color
        }
    }

}