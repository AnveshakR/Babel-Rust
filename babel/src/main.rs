use std::{io};

// 26 lowercase letters, period, comma, space = 29 characters
// 3200 characters per book
// could be reversible LCG

// base-29 has 1-9 and a-s

fn main() {

    let a:u128 = 91;
    let c:u128 = 83;
    let m:u128 = 29;

    println!("Enter seed: ");
    let mut seedinput = String::new();

    io::stdin()
        .read_line(&mut seedinput)
        .expect("what");

    let seed = seedinput.trim().parse::<u128>().expect("NAN");

    let rng:u128 = (a * seed + c)%m;

    println!("{rng}");
    
}
