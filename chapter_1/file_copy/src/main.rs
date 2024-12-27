use std::io;

fn main() {
    for line in io::stdin().lines() {
        for character in line.expect("unable to get line from stdin").chars() {
            print!("{}", character);
        }
        print!("\n");
    }
}
