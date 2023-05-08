pub mod cycle;  // cycle.rs或cycle/mod.rs
pub mod function;   // function.rs或function/mod.rs
pub mod str;

pub fn my_loop() {
    let mut i=1;

    let result= loop {
        if i==10 {
            break i*2;
        }

        i+=1;
    };

    println!("{}",result)
}