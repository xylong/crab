use base::function::get_level_by_score;

mod game;
mod base;

fn main() {
    // game::mini::guess_number();
    // base::cycle::countdown();
    // base::my_loop();
    println!("grade is {}", get_level_by_score(100)) 
}


