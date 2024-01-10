/*
Problem: Write a function reverse(s) that reverses the character
string s. Use it to write a program that reverses its
input a line at a time.

*/

use std::env;

use refactor::get_content;

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

        let results: Vec<String> = file_content
            .lines()
            .map(|line| line.to_string().chars().rev().collect())
            .collect();

        if results.len() > 0 {
            println!("\nfile: {}\n\n{}", results.join("\n"), &file_path);
        }
    }
}
