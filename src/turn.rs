use std::fmt;

pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn new() -> Turn {
        Turn::Black
    }

    pub fn change(&mut self) {
        match self {
            Turn::Black => *self = Turn::White,
            Turn::White => *self = Turn::Black
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Turn::Black => write!(f, "黒"),
            Turn::White => write!(f, "白"),
        }
    }
}
