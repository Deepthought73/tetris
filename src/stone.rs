use termion::color;

#[derive(Clone)]
pub struct Stone {
    pub x: usize,
    pub y: usize,
    pub block_mask: [[[bool; 4]; 4]; 4],
    pub rotation: usize,
    pub color: color::Rgb,
}

impl Stone {
    pub fn i(x: usize, y: usize, color: color::Rgb) -> Stone {
        Stone {
            x,
            y,
            block_mask: [
                [
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                ],
                [
                    [true, true, true, true],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                ],
                [
                    [true, true, true, true],
                    [false, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
            ],
            rotation: 0,
            color,
        }
    }

    pub fn l(x: usize, y: usize, color: color::Rgb) -> Stone {
        Stone {
            x,
            y,
            block_mask: [
                [
                    [true, false, false, false],
                    [true, false, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                ],
                [
                    [false, false, true, false],
                    [true, true, true, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, true, false],
                    [true, false, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]
            ],
            rotation: 0,
            color,
        }
    }

    pub fn o(x: usize, y: usize, color: color::Rgb) -> Stone {
        Stone {
            x,
            y,
            block_mask: [
                [
                    [true, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]
            ],
            rotation: 0,
            color,
        }
    }

    pub fn j(x: usize, y: usize, color: color::Rgb) -> Stone {
        Stone {
            x,
            y,
            block_mask: [
                [
                    [false, true, false, false],
                    [false, true, false, false],
                    [true, true, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, true, false],
                    [false, false, true, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, false, false],
                    [true, false, false, false],
                    [true, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, false, false, false],
                    [true, true, true, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ]
            ],
            rotation: 0,
            color,
        }
    }

    pub fn t(x: usize, y: usize, color: color::Rgb) -> Stone {
        Stone {
            x,
            y,
            block_mask: [
                [
                    [false, true, false, false],
                    [true, true, true, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [false, true, false, false],
                    [true, true, false, false],
                    [false, true, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, true, true, false],
                    [false, true, false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ],
                [
                    [true, false, false, false],
                    [true, true, false, false],
                    [true, false, false, false],
                    [false, false, false, false],
                ]
            ],
            rotation: 0,
            color,
        }
    }
}