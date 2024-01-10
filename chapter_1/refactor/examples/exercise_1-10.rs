/// Problem:
/// --------
///
/// Write a program which prints the words in its input, one per line.
///
/// Notes:
/// Modifying exercise_1-9 to solve this.
use std::fmt::Write as _;

use refactor::{program, CharacterVisitor};

#[derive(Debug)]
enum WordState {
    InWord,
    NotInWord,
}

struct WordPrintVisitor {
    word_state: WordState,
    count: u32,
    current_word: String,
    buffer: String,
}

impl CharacterVisitor for WordPrintVisitor {
    fn visit<T: refactor::CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        match current_char {
            '\n' | ' ' | '\t' => match self.word_state {
                WordState::InWord => {
                    self.word_state = WordState::NotInWord;

                    //Save current word to buffer
                    write!(&mut self.buffer, "{}\n", self.current_word).unwrap();
                    self.current_word = String::new();
                }
                WordState::NotInWord => {}
            },
            _ => match self.word_state {
                WordState::InWord => {
                    // Continue to build up word
                    self.current_word.push(current_char.clone());
                }
                WordState::NotInWord => {
                    self.count += 1;
                    self.word_state = WordState::InWord;

                    // Start building current word
                    self.current_word.push(current_char.clone());
                }
            },
        }
    }

    fn results(&self) -> String {
        format!("{}", self.buffer)
    }
}

fn main() {
    let mut visitor = WordPrintVisitor {
        word_state: WordState::NotInWord,
        count: 0,
        current_word: String::new(),
        buffer: String::new(),
    };

    program(&mut visitor);
}
