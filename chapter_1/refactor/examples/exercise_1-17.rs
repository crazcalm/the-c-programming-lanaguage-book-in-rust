/*
Problem: Write a program to print all input lines that are longer than
         80 characters.
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

        let results: Vec<&str> = file_content
            .lines()
            .filter(|line| line.len() > 80)
            .collect();

        if results.len() > 0 {
            println!("\nfile: {}\n\n\n{}", results.join("\n\n"), &file_path);
        }
    }
}
