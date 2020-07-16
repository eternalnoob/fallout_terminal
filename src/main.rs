use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut file = File::open("enable1.txt")?;
    let mut contents = String::new();
    let words:Vec<&str>= contents.split("\n").collect();
    file.read_to_string(&mut contents)?;
    println!("test {}", contents);
    Ok(())
}

fn choose_words(words: Vec<&str>, count: i32) -> Vec<&str> {
    let mut chosen = HashSet::new();
    while chosen.len() < count as usize {
        let num = rand::thread_rng().gen_range(0, words.len());
        chosen.insert(words[num]);
    }
    chosen.iter().map(|x| *x).collect();
}
