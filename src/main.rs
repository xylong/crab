use base::function::get_level_by_score;

use crate::base::str::check_password_length;

mod game;
mod base;

fn main() {
    // game::mini::guess_number();
    // base::cycle::countdown();
    // base::my_loop();
    // println!("grade is {}", get_level_by_score(100)) 
    println!("{}",check_password_length("password"))
}


