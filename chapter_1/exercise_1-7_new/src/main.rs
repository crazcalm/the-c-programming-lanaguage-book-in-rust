use std::io;

const IN_WHITESPACE: bool = true;
const OUT_WHITESPACE: bool = false;

fn main() {
    let mut state = OUT_WHITESPACE;

    for line in io::stdin().lines() {
        for character in line.expect("unable to get line from stdin").chars() {
            if !character.eq(&' ') {
                print!("{}", character);

                if state == IN_WHITESPACE {
                    state = OUT_WHITESPACE;
                }
            }

            if character.eq(&' ') {
                if state == OUT_WHITESPACE {
                    print!("{}", character);

                    state = IN_WHITESPACE;
                }
            }
        }
        print!("\n") // Adding back the new lines
    }
}
