use rand::Rng;
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

const GREEN: &str = " \x1B[42m";
const YELLOW: &str = " \x1B[43m";
const RESET: &str = "\x1B[0m";
const GUESSES: usize = 6;
const LETTERS: usize = 5;

fn get_words() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(BufReader::new(File::open("/usr/share/dict/words.sorted")?)
        .lines()
        .filter_map(Result::ok)
        .filter(|word| word.len() == LETTERS)
        .collect())
}

fn has_word(words: &Vec<String>, word: &String) -> bool {
    let mut beg: usize = 0;
    let mut end: usize = words.len();
    let mut mid: usize;
    if word.len() != LETTERS {
        println!("Incorrect word length!");
        return false;
    }
    while beg <= end {
        mid = (beg + end) / 2;
        match words[mid].cmp(word) {
            Ordering::Equal => return true,
            Ordering::Greater => end = mid - 1,
            Ordering::Less => beg = mid + 1,
        }
    }
    false
}

fn put_word(guess: &str, secret: &str) {
    let mut copy: String = String::from(secret);
    let mut color: &str;
    let mut pos: Option<usize>;

    print!("\t");
    for (i, ch) in guess.chars().enumerate() {
        color = " ";
        pos = copy.find(ch);
        if let Some(posnum) = pos {
            color = if i == posnum { GREEN } else { YELLOW };
            copy.replace_range(posnum..posnum+1, ".");
        }
        print!("{color}{ch}{RESET}");
    }
    println!();
}

fn display(guesses: &Vec<String>, secret: &str) {
    print!("\x1B[2J\x1B[1;1H");
    println!("\t~$ wordle-rs");
    println!("{YELLOW} {RESET} - right letter, wrong place");
    println!("{GREEN} {RESET} - right letter, right place");

    for guess in guesses {
        put_word(guess.as_str(), secret);
    }

    if guesses.len() < GUESSES {
        println!("\t - - - - -");
        for _i in 0..(GUESSES - guesses.len() - 1) {
            println!();
        }
    }
    println!("\t__________");
}

fn parse_guess(
    guess: &str,
    guesses: &mut Vec<String>,
    words: &Vec<String>,
    secret: &String,
    rot: &mut bool,
    won: &mut bool,
) {
    let guess: String = guess.to_lowercase();
    if has_word(words, &guess) {
        guesses.push(guess.to_string());
        if &guess == secret {
            *won = true;
        }
    } else {
        println!("Not a valid word!");
        *rot = false;
    }
}

fn main() {
    let words: Vec<String> = get_words().unwrap();
    let secret: &String = &words[rand::thread_rng().gen_range(0..=words.len())];

    let mut guesses: Vec<String> = Vec::new();
    let mut won: bool = false;
    let mut rot: bool = true;
    let mut guess: String;

    while !won && guesses.len() < GUESSES {
        if rot {
            display(&guesses, secret);
        } else {
            rot = true;
        }

        print!("\t");
        guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid input");
        guess.pop();

        parse_guess(&guess, &mut guesses, &words, secret, &mut rot, &mut won);
    }

    display(&guesses, secret);
    
    if won {
        println!("\nYou won!");
    } else {
        println!("\nYou lost! The word was: {secret}");
    }
}
