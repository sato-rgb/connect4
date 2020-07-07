use std::fmt;
use std::convert::TryFrom;

use crate::config::{FIELD_SIZE_X, FIELD_SIZE_Y, END_CONDITION};
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

enum MyError {
    None
}

impl Field {
    pub fn new() -> Field {
        Field { field: [[BlockKind::Empty; FIELD_SIZE_X]; FIELD_SIZE_Y] }
    }
    #[allow(dead_code)]
    pub fn from_array(f: [[BlockKind; FIELD_SIZE_X]; FIELD_SIZE_Y]) -> Field {
        Field { field: f }
    }
    pub fn put_block(&mut self, pos: &usize, turn: &Turn) -> Result<(usize, usize), ()> {
        let put_block = match turn {
            Turn::Black => BlockKind::Black,
            Turn::White => BlockKind::White,
        };
        for (height, fi) in self.field.iter_mut().enumerate().rev() {
            if fi[pos - 1] == BlockKind::Empty {
                fi[pos - 1] = put_block;
                return Ok((*pos - 1, height));
            }
        }
        //ここ来たら積めない
        Err(())
    }

    //フィールドの全マスをcheck_to_vectorに右、下、右下のベクトルとともに投げる

    pub fn game_end_process(&self) -> bool {
        let checking_vector = [(1, 0), (0, 1), (1, 1)];
        for (x1, x2) in self.field.iter().enumerate() {
            for (y1, _y2) in x2.iter().enumerate() {
                for vs in checking_vector.iter() {
                    match Self::check_to_vector(self, x1 as isize, y1 as isize, vs.0, vs.1) {
                        Ok(true) => { return true; }
                        Ok(false) | Err(_) => { continue; }
                    }
                }
            }
        }
        false
    }
    fn check_to_vector(&self, fx: isize, fy: isize, vx: isize, vy: isize) -> Result<bool, MyError> {
        let mut array: [BlockKind; END_CONDITION] = [BlockKind::Empty; END_CONDITION];
        let empty: [BlockKind; END_CONDITION] = [BlockKind::Empty; END_CONDITION];
        let black: [BlockKind; END_CONDITION] = [BlockKind::Black; END_CONDITION];
        let white: [BlockKind; END_CONDITION] = [BlockKind::White; END_CONDITION];
        for len in 0..END_CONDITION {
            let x = fx + (len as isize) * (vx);
            let y = fy + (len as isize) * (vy);
            let x = Self::cast_or_err(x)?;
            let y = Self::cast_or_err(y)?;
            let i = self.field.get(x).ok_or(MyError::None)?;
            let i = i.get(y).ok_or(MyError::None)?;
            array[len] = *i;
        }

        match array {
            a if a == black => { Ok(true) }
            a if a == white => { Ok(true) }
            a if a == empty => { Ok(false) }
            _ => { Ok(false) }
        }
    }
    fn cast_or_err(num: isize) -> Result<usize, MyError> {
        let u = usize::try_from(num);
        match u {
            Ok(u) => Ok(u),
            Err(_) => Err(MyError::None)
        }
    }
}
/*いらないかも
fn check_to_vector(&self, fx: usize, fy: usize, vx: isize, vy: isize) -> bool {
    let start = self.field[fx][fy].borrow();
    let fx = fx as isize;
    let fy = fy as isize;
    for i in 0..END_CONDITION as isize {
        get_slice();
        let a = self.field.get([fx + i * (vx)][fy + i * (vy)]);
        match a {
            Some(a) => {
                if start != a { return false; };
                continue;
            }
            None(_) => { return false; }
        }
    }
    true
}*/
//test code
//END_CONDITIONぶんのフィールド


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