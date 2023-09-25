
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGray,
    LightGray,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub const fn rgb(&self) -> RGB {
        match self {
            Self::Black      => RGB { r: 0,   g: 0,   b: 0   },
            Self::DarkBlue   => RGB { r: 29,  g: 43,  b: 83  },
            Self::DarkPurple => RGB { r: 126, g: 37,  b: 83  },
            Self::DarkGreen  => RGB { r: 0,   g: 135, b: 81  },
            Self::Brown      => RGB { r: 171, g: 82,  b: 54  },
            Self::DarkGray   => RGB { r: 95,  g: 87,  b: 79  },
            Self::LightGray  => RGB { r: 194, g: 195, b: 199 },
            Self::White      => RGB { r: 255, g: 241, b: 232 },
            Self::Red        => RGB { r: 255, g: 0,   b: 77  },
            Self::Orange     => RGB { r: 255, g: 163, b: 0   },
            Self::Yellow     => RGB { r: 255, g: 236, b: 39  },
            Self::Green      => RGB { r: 0,   g: 228, b: 54  },
            Self::Blue       => RGB { r: 41,  g: 173, b: 255 },
            Self::Lavender   => RGB { r: 131, g: 118, b: 156 },
            Self::Pink       => RGB { r: 255, g: 119, b: 168 },
            Self::LightPeach => RGB { r: 255, g: 204, b: 170 }
        }
    }
}

pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool
}

impl Input {
    pub const fn new() -> Self {
        Self {
            up: false, down: false, left: false, right: false, a: false, b: false
        }
    }
}

pub type Sprite = [[Color; 8]; 8];