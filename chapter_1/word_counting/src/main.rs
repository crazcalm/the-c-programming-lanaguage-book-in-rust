use std::io;

const YES: bool = true;
const NO: bool = false;

fn main() {
    let (mut line_count, mut word_count, mut char_count, mut in_word) = (0, 0, 0, NO);

    for line in io::stdin().lines() {
        for character in line.expect("unable to get line from stdin").chars() {
            char_count += 1;

            if character.eq(&' ') || character.eq(&'\t') {
                in_word = NO;
            } else if in_word == NO {
                in_word = YES;
                word_count += 1;
            }
        }
        // Handles "\n" case
        line_count += 1;
        char_count += 1;
        in_word = NO;
    }

    println!("{} {} {}", line_count, word_count, char_count);
}
