use std::{io,cmp::Ordering};

use rand::Rng;


pub fn guess_number() {
    println!("猜数字");

    let secret_num=rand::thread_rng().gen_range(1..100);

    loop {
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜的数字：{}",guess);

        let guess:u32=match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("please enter a number");
                continue;
            },
        };

        match guess.cmp(&secret_num) {
            Ordering::Less=>println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}