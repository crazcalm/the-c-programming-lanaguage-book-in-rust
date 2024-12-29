use std::io;

fn main() {
    let mut count: i128 = 0;

    for line in io::stdin().lines() {
        for character in line.expect("unable to get line from stdin").chars() {
            if character.eq(&'\t') {
                count += 1;
            }
            if character.eq(&' ') {
                count += 1;
            }
        }
        count += 1; // counts the new lines
    }
    println!("{}", count);
}
