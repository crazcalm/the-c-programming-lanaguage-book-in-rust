/// Problem:
/// -------
/// Write a program to count blanks, tabs and newlines.
///
/// Note: I want to use the vistor pattern... for real this time.
use refactor::{program, CharacterVisitor};

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
            self.new_line_count += 1;
        }
    }

    fn results(&self) -> String {
        format!(
            "blanks spaces: {} tabs: {} new lines: {}",
            self.blank_count, self.tab_count, self.new_line_count
        )
    }
}

fn main() {
    let mut visitor = BlanksTabsNewLinesVisitor {
        blank_count: 0,
        tab_count: 0,
        new_line_count: 0,
    };
    program(&mut visitor);
}
