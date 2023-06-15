/// Problem:
/// --------
///
/// Write a program which prints the words in its input, one per line.
///
/// Notes:
/// Modifying exercise_1-9 to solve this.
use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
enum WordState {
    InWord,
    NotInWord,
}

fn get_content(file_path: &str) -> io::Result<String> {
    if file_path.eq("-") {
        let mut input = String::new();
        for line in io::stdin().lines() {
            input.push_str(&line.unwrap());
            input.push_str("\n");
        }

        Ok(input)
    } else {
        fs::read_to_string(file_path)
    }
}

fn word_count(content: &str) {
    let mut word_state: WordState;

    let mut word_count = 0;
    let mut word = String::new();

    for character in content.chars() {
        word_state = match character {
            '\n' | ' ' | '\t' => WordState::NotInWord,
            _ => WordState::InWord,
        };

        match word_state {
            WordState::NotInWord => {
                if word_count > 0 && word.len() > 0 {
                    println!("{}", word)
                }
                word_count += 1;
                word.clear();
            }
            WordState::InWord => {
                if !character.is_whitespace() {
                    word.push(character);
                }
            }
        }
    }
}

fn main() {
    let args = env::args();

    if args.len() > 1 {
        for (count, arg) in args.skip(1).enumerate() {
            if count > 0 {
                println!("\n");
            }

            if arg.eq("-") {
                println!("Input: Stdin");
            } else {
                println!("Input: {}", arg);
            }

            let content = get_content(&arg).unwrap();

            word_count(&content);
        }
    } else {
        let content = get_content("-").unwrap();

        word_count(&content);
    }
}
