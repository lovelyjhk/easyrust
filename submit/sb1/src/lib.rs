use std::fs::File;
use std::io::prelude::*;
use rand::seq::SliceRandom;

pub fn get_string(length: usize) -> String {
    let mut random_chars: Vec<char> = (b'A'..=b'Z').map(char::from).collect();
    let mut rng = rand::thread_rng();
    random_chars.shuffle(&mut rng);
    let random_string: String = random_chars.into_iter().take(length).collect();
    random_string
}

fn main() {
    let lengths = vec![10, 20, 30];
    
    for length in lengths {
        let random_string = get_string(length);
        println!("Random string of length {}: {}", length, random_string);
    }
}
