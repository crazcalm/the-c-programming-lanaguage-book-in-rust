use std::io;

const YES: bool = true;
const NO: bool = false;

fn main() {
    let mut in_word = NO;

    for line in io::stdin().lines() {
        for character in line.expect("unable to get line from stdin").chars() {
            if character.eq(&' ') || character.eq(&'\t') {
                if in_word == YES {
                    // We have reached the end of a word. Add a new line.
                    print! {"\n"};
                }
                in_word = NO;
            } else if in_word == NO {
                in_word = YES;

                // print the first character of the "word"
                print!("{}", character);
            } else {
                // print the rest of the word
                print!("{}", character);
            }
        }
        // Handles "\n" case
        if in_word == YES {
            // We have reached the end of a word. Add a new line.
            print! {"\n"};
        }
        in_word = NO;
    }
}
