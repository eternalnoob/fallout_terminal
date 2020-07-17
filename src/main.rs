use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::collections::HashSet;
use std::io::stdin;
use maplit::hashmap;

fn main() -> std::io::Result<()> {
    let (word_count, word_length) = get_choices();

    let mut file = File::open("enable1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let words:Vec<&str>= contents.split("\n").collect();
    let chosen = choose_words(words, word_count, word_length);

    process_game(&chosen, 5);

    Ok(())
}

fn process_game(words: &Vec<&str>, max_guesses: u32) {
    let answer = words[rand::thread_rng().gen_range(0, words.len())];
    let mut found = false;
    let mut guesses = 0;
    while guesses < 5 && !found {
        print_choices(words);

        println!("Please choose number from 0 to {}", words.len()-1);
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Failed to gather string!");

        let choice: usize = match s.trim().parse() {
            Ok(num) => {
                if num <= 0 || num >= words.len() {
                    continue
                }
                num
            },
            Err(_) => continue,
        };

        let choice = words[choice];
        println!("you have chosen: {}", choice);
        let common = cmp(choice, answer);
        if common == choice.len() {
            println!("A winrar is you!");
            return
        }
        println!("{}/{} correct characters", common, choice.len());

        guesses = guesses + 1;

    }
    println!("you have lost the game, sorry");

}

fn cmp(a: &str, b: &str) -> usize {
    // compares two strings and returns the count of shared letter positions in them
    let mut common: usize = 0;
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char == b_char {
            common = common + 1;
        }
    }

    common
}

fn print_choices(words: &Vec<&str>) {
    let mut idx = 0;
    for word in words.iter() {
        println!("{}: {}", idx, word);
        idx = idx + 1;
    }
}

fn choose_words(words: Vec<&str>, count: u32, len: u32) -> Vec<&str> {
    let mut chosen = HashSet::new();
    while chosen.len() < count as usize {
        let num = rand::thread_rng().gen_range(0, words.len());
        if words[num].len() == len as usize {
            chosen.insert(words[num]);
        }
    }
    chosen.iter().map(|x| *x).collect()
}

fn get_choices() -> (u32, u32) {
    // returns word count, word length in that order
    let mut difficulty = 0;
    let mut length: u32 = 0;

    while difficulty > 5 || difficulty == 0 {
        let mut s = String::new();

        println!("Please choose a difficulty from 1-5");
        stdin().read_line(&mut s).expect("Failed to gather string!");

        difficulty = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    let diff_map = hashmap!{
        1 => 7 as u32,
        2 => 9 as u32,
        3 => 11 as u32,
        4 => 13 as u32,
        5 => 15 as u32,
    };

    let word_count: u32 = *diff_map.get(&difficulty).expect("could not find value in map");

    while length > 10 || length < 5 {
        let mut s = String::new();
        println!("Please choose a word length from 5-10");
        stdin().read_line(&mut s).expect("Failed to gather string!");

        length = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
    }

    (word_count, length)
}
