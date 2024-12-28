use std::io;

fn main() {
    let mut count: i128 = 0;

    for line in io::stdin().lines() {
        for _character in line.expect("unable to get line from stdin").chars() {
            count += 1;
        }
        count += 1;
    }
    println!("{}", count);
}
