use std::fmt;
use std::fs;
use std::io;

pub trait CharacterVisitor {
    fn visit<T: CharacterVisitee + fmt::Debug>(&mut self, node: &T);
}

pub trait CharacterVisitee: CharacterGetter {
    fn accept<T: CharacterVisitor>(&self, visitor: &mut T);
}

pub trait CharacterGetter {
    fn get_char(&self) -> &char;
}

#[derive(Debug)]
pub struct Character(pub char);

impl CharacterGetter for Character {
    fn get_char(&self) -> &char {
        &self.0
    }
}

impl CharacterVisitee for Character {
    fn accept<T: CharacterVisitor>(&self, char_visitor: &mut T) {
        char_visitor.visit(self)
    }
}

pub fn get_content(file_path: &str) -> io::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    struct CountCharsVisitor {
        pub count: u32,
    }

    impl CharacterVisitor for CountCharsVisitor {
        fn visit<T: CharacterVisitee + fmt::Debug>(&mut self, node: &T) {
            println!("{}", node.get_char());
            self.count += 1
        }
    }

    #[test]
    fn test_visitor() {
        let test_string = String::from("I am a test\n");
        let mut count_visitor = CountCharsVisitor { count: 0 };

        for c in test_string.chars() {
            let character = Character(c);
            character.accept(&mut count_visitor);
        }

        assert_eq!(test_string.len(), count_visitor.count as usize)
    }
}
