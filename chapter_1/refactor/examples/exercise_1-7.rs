/// Problem:
/// --------
///
/// Write a program to copy its input to its output,
/// replacing each string of one or more blanks with a
/// single blank.
///
use std::env;

use refactor::{get_content, Character, CharacterVisitee, CharacterVisitor};

struct SingleBlankVisitor {
    data: String,
    last_char: Option<char>,
}

impl CharacterVisitor for SingleBlankVisitor {
    fn visit<T: CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        if self.last_char.is_some() {
            if current_char.eq(&' ') && self.last_char.unwrap().eq(&' ') {
                // Do nothing -- Skipping the space
                return;
            }
        }

        self.data.push(current_char.clone());
        self.last_char = Some(current_char.clone())
    }
}

fn main() {
    let args = env::args();
    let files: Vec<String>;

    if args.len() > 1 {
        files = args.skip(1).collect();
    } else {
        files = vec!["-".to_string()];
    }

    for (count, file_path) in files.iter().enumerate() {
        if count > 0 {
            println!();
        }

        let file_content = get_content(&file_path).expect("unable to get lines from file_path");

        if file_path.eq(&"-") {
            // Creating a space between the input and output
            println!();
        }
        let mut visitor = SingleBlankVisitor {
            data: String::new(),
            last_char: None,
        };

        for c in file_content.chars() {
            let character = Character(c);
            character.accept(&mut visitor);
        }

        if files.len() > 1 {
            println!("File: {}\n", file_path);
        }
        print!("{}", visitor.data);
    }
}
