/// Problem:
/// -------
///
/// Write a program to count blanks, tabs and newlines.
///
/// Note: I tried to model the behavior off of wc
use std::env;
use std::fs;
use std::io;

fn do_counting(input: &str) -> (u32, u32, u32) {
    let mut blanks: u32 = 0;
    let mut tabs: u32 = 0;
    let mut newlines: u32 = 0;

    for line in input.lines() {
        newlines += 1;

        for character in line.chars() {
            if character.eq(&' ') {
                blanks += 1;
            } else if character.eq(&'\t') {
                tabs += 1;
            }
        }
    }

    return (blanks, tabs, newlines);
}

fn get_content(file_path: &str) -> io::Result<String> {
    if file_path.eq("-") {
        let mut input = String::new();
        for line in io::stdin().lines() {
            input.push_str(&line.unwrap());
            input.push_str("\n")
        }

        return Ok(input);
    } else {
        fs::read_to_string(file_path)
    }
}

fn main() {
    let args = env::args();

    if args.len() > 1 {
        let mut total_blanks: u32 = 0;
        let mut total_tabs: u32 = 0;
        let mut total_newlines: u32 = 0;

        for (count, arg) in args.skip(1).enumerate() {
            //let file_content = fs::read_to_string(&arg);

            let file_content = get_content(&arg);

            if let Ok(input) = file_content {
                let (blanks, tabs, newlines) = do_counting(&input);

                println!("{} {} {} {} ", blanks, tabs, newlines, arg);

                total_blanks += blanks;
                total_tabs += tabs;
                total_newlines += newlines;
            }

            if count > 0 {
                println!("{} {} {} totals", total_blanks, total_tabs, total_newlines);
            }
        }
    } else {
        let input = get_content("-").unwrap();
        let (blanks, tabs, newlines) = do_counting(&input);

        println!("{} {} {}", blanks, tabs, newlines);
    }
}
