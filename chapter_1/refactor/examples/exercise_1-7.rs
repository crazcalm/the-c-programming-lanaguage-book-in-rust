/// Problem:
/// --------
///
/// Write a program to copy its input to its output,
/// replacing each string of one or more blanks with a
/// single blank.
///
use refactor::{program, CharacterVisitee, CharacterVisitor};

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

    fn results(&self) -> String {
        self.data.to_string()
    }
}

fn main() {
    let mut visitor = SingleBlankVisitor {
        data: String::new(),
        last_char: None,
    };

    program(&mut visitor);
}
