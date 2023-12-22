use std::env;
use std::fmt;
use std::fs;
use std::io;

pub trait CharacterVisitor {
    fn visit<T: CharacterVisitee + fmt::Debug>(&mut self, node: &T);
    fn results(&self) -> String;
}

pub trait CharacterVisitee: CharacterGetter {
    fn accept<T: CharacterVisitor>(&self, visitor: &mut T);
}

pub trait CharacterGetter {
    fn get_char(&self) -> &char;
}

#[derive(Debug, Copy, Clone)]
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

pub fn apply_visitor_algo(
    iterator: impl std::iter::Iterator<Item = Character>,
    visitor: &mut impl CharacterVisitor,
) {
    for c in iterator {
        c.accept(visitor);
    }
}

pub fn program(visitor: &mut impl CharacterVisitor) {
    let args = env::args();
    let files: Vec<String>;
    if args.len() > 1 {
        files = args.skip(1).collect();
    } else {
        files = vec!["-".to_string()];
    }

    for file_path in files {
        let file_content = get_content(&file_path).expect("unable to get lines from file_path");

        apply_visitor_algo(file_content.chars().map(|x| Character(x)), visitor);

        println!("{} file: {}", visitor.results(), &file_path);
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

        fn results(&self) -> String {
            format!("total count: {}", self.count)
        }
    }

    #[test]
    fn test_apply_visitor_algo() {
        let test_string = String::from("I am a test\n");
        let mut count_visitor = CountCharsVisitor { count: 0 };

        apply_visitor_algo(
            test_string.chars().map(|x| Character(x)),
            &mut count_visitor,
        );

        assert_eq!(test_string.len(), count_visitor.count as usize);
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
