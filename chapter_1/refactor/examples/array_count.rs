/// Problem:
/// -------
/// Count digits, white space, others
use refactor::{program, CharacterVisitor};

#[derive(Debug)]
struct CountVisitor {
    whitespace: u32,
    others: u32,
    digits: [u32; 10],
}

impl CharacterVisitor for CountVisitor {
    fn visit<T: refactor::CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        if current_char.is_digit(10) {
            self.digits[current_char.to_digit(10).unwrap() as usize] += 1;
        } else if current_char.is_whitespace() {
            self.whitespace += 1;
        } else {
            self.others += 1;
        }
    }

    fn results(&self) -> String {
        format!(
            "digits = {:?}, white space = {}, other = {}",
            self.digits, self.whitespace, self.others
        )
    }
}

fn main() {
    let mut visitor = CountVisitor {
        whitespace: 0,
        others: 0,
        digits: [0; 10],
    };
    program(&mut visitor);
}
