use std::{io};
use rug::{Assign, Integer};

// 26 lowercase letters, period, comma, space = 29 characters
// 3200 characters per book
// could be reversible LCG

// generate a 

// base-29 has 1-9 and a-s

fn main() {

    let a:u128 = 91;
    let c:u128 = 83;
    let base:u128 = 2;
    let m = Integer::from(100);


    let charset = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', ',', '.'];

    println!("Enter seed: ");
    let mut seedinput = String::new();

    io::stdin()
        .read_line(&mut seedinput)
        .expect("what");

    let seed = seedinput.trim().parse::<u128>().expect("NAN");

    let rng:u128 = (a * seed + c)%m;
    let character:&char = &charset[rng as usize];

    println!("{rng} {character}");
    
}
