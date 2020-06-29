use std::fmt;

use crate::config::{FIELD_SIZE_X, FIELD_SIZE_Y};

#[derive(Copy, Clone, Debug)]
pub enum BlockKind {
    Black,
    White,
    Empty,
}

impl fmt::Display for BlockKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            BlockKind::Black => write!(f, "■"),
            BlockKind::White => write!(f, "□"),
            BlockKind::Empty => write!(f, "・")
        }
    }
}

#[derive(Debug)]
pub struct Field {
    field: [[BlockKind; FIELD_SIZE_X]; FIELD_SIZE_Y]
}

impl Field {
    pub fn new() -> Field {
        Field { field: [[BlockKind::Empty; FIELD_SIZE_X]; FIELD_SIZE_Y] }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.field.iter() {
            for j in i.iter() {
                write!(f, "{}", j)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}