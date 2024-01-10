/// Problem:
/// --------
///
/// Write a program to print a histogram of the lengths of words in its input. It is easy to draw the histogram with the bars horizontal; a vertical orientation is more challenging.
///
use refactor::{program, CharacterVisitor};
use std::collections::HashMap;

#[derive(Debug)]
enum WordState {
    InWord,
    NotInWord,
}

struct WordHistogramVisitor {
    word_state: WordState,
    count: u32,
    current_word: String,
    words: HashMap<String, u32>,
}

impl CharacterVisitor for WordHistogramVisitor {
    fn visit<T: refactor::CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        match current_char {
            '\n' | ' ' | '\t' => match self.word_state {
                WordState::InWord => {
                    self.word_state = WordState::NotInWord;

                    //Save current word to buffer
                    self.words
                        .entry(self.current_word.clone())
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
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
        let mut results = String::new();
        fn bar(num: u32) -> String {
            "x".repeat(num as usize)
        }
        let mut width = 0;

        for (key, _) in self.words.iter() {
            match key.len() > width {
                true => {
                    width = key.len();
                }
                false => {}
            }
        }

        for (key, value) in self.words.iter() {
            results += &format!("{:width$}: {}\n", key, bar(value.clone()), width = width);
        }
        format!("{}", results)
    }
}

fn main() {
    let mut visitor = WordHistogramVisitor {
        word_state: WordState::NotInWord,
        count: 0,
        current_word: String::new(),
        words: HashMap::new(),
    };

    program(&mut visitor);
}
