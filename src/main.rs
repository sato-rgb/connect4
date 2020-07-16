mod test;

use crate::input::input_put_pos;

mod turn;
mod input;
mod field;
mod config;

fn main() {
    let mut field = field::Field::new();
    let mut turn = turn::Turn::new();
    print!("{}", field);
    println!("{}のターンです", turn);
    loop {
        let input = match input_put_pos() {
            Ok(i) => { i }
            Err(()) => {
                println!("無効な入力です");
                continue;
            }
        };
        match field.put_block(&input, &turn) {
            Ok(i) => { println!("{:?}", i) }
            Err(()) => {
                println!("もう積めません");
                continue;
            }
        };
        if field.game_end_process() == true {
            println!("{}の勝ちです", turn);
            print!("{}", field);
            break;
        };
        if field.check_draw() == true {
            println!("引き分けです");
            print!("{}", field);
            break;
        };
        turn.change();
        print!("{}", field);
        println!("{}のターンです", turn);
    }
}
