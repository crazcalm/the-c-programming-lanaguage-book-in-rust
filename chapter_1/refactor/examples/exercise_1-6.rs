/// Problem:
/// -------
///
/// Write a program to count blanks, tabs and newlines.
///
/// Note: I want to use the visitor pattern for this.
use std::env;

use refactor::{get_content, Character, CharacterVisitee, CharacterVisitor};

struct BlanksTabsNewLinesVisitor {
    blank_count: u32,
    tab_count: u32,
    new_line_count: u32,
}

impl CharacterVisitor for BlanksTabsNewLinesVisitor {
    fn visit<T: refactor::CharacterVisitee + std::fmt::Debug>(&mut self, node: &T) {
        let current_char = node.get_char();

        if current_char.eq(&' ') {
            self.blank_count += 1;
        } else if current_char.eq(&'\t') {
            self.tab_count += 1;
        } else if current_char.eq(&'\n') {
            self.new_line_count += 1
        }
    }
    fn results(&self) -> String {
        "Not used here".to_string()
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

    for file_path in files {
        let file_content = get_content(&file_path).expect("unable to get lines from file_path");

        let mut visitor = BlanksTabsNewLinesVisitor {
            blank_count: 0,
            tab_count: 0,
            new_line_count: 0,
        };
        for c in file_content.chars() {
            let character = Character(c);
            character.accept(&mut visitor);
        }

        println!(
            "blank spaces: {} tabs: {} new lines: {} file: {}",
            visitor.blank_count, visitor.tab_count, visitor.new_line_count, &file_path
        );
    }
}
