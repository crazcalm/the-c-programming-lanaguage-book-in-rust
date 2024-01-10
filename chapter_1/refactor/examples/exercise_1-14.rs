/// Problem:
/// --------
///
/// Write a program to print a histogram of the frequencies of different characters in its input.
///
use refactor::{program, CharacterVisitor};
use std::collections::HashMap;

struct CharFreqVisitor {
    chars: HashMap<char, usize>,
}

impl CharacterVisitor for CharFreqVisitor {
    fn visit<T: refactor::CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        self.chars
            .entry(current_char.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    fn results(&self) -> String {
        let mut results = String::new();
        fn bar(num: usize) -> String {
            "x".repeat(num)
        }

        for (key, value) in self.chars.iter() {
            results += &format!("{:width$}: {}\n", key, bar(value.clone()), width = 1);
        }
        format!("{}", results)
    }
}

fn main() {
    let mut visitor = CharFreqVisitor {
        chars: HashMap::new(),
    };

    program(&mut visitor);
}
