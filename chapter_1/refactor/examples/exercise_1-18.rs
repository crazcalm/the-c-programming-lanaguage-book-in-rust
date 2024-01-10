/*
Problem: Write a program to remove trailing blanks and tabs from each
         line of input and to delete entirely blank lines.

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
            .map(|line| line.trim_end())
            .filter(|line| line.len() > 0)
            .collect();

        if results.len() > 0 {
            println!("\nfile: {}\n\n\n{}", results.join("\n\n"), &file_path);
        }
    }
}
