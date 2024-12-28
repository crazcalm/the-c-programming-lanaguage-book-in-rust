use std::io;

fn main() {
    let mut count: isize = 0;

    for _line in io::stdin().lines() {
        count += 1;
    }
    println!("{}", count);
}
