use std::io;
use super::config::FIELD_SIZE_X;

pub fn input_put_pos() -> Result<usize, ()> {
    let mut put_pos = String::new();
    io::stdin()
        .read_line(&mut put_pos)
        .expect("Failed to read line");
    //validate check
    match put_pos.trim().parse::<usize>() {
        Ok(i) => match i {
            1..=FIELD_SIZE_X => { Ok(i) }
            _ => {
                Err(())
            }
        },
        Err(_) => {
            Err(())
        }
    }
}
