use std::fmt;

use crate::config::{FIELD_SIZE_X, FIELD_SIZE_Y};
use crate::turn::Turn;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    pub fn put_block(&mut self, pos: &usize, turn: &Turn) -> Result<(),()> {
        let put_block = match turn {
            Turn::Black => BlockKind::Black,
            Turn::White => BlockKind::White,
        };
        for fi in self.field.iter_mut().rev() {
            if fi[pos - 1] == BlockKind::Empty {
                fi[pos - 1] = put_block;
                return Ok(());
            }
        }
        //ここ来たら積めない
        Err(())
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