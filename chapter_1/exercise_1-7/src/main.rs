/// Problem:
/// --------
///
/// Write a program to copy its input to its output,
/// replacing each string of one or more blanks with a
/// single blank.
///
use std::env;
use std::fs;
use std::io;

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

fn replace_blanks_and_print(content: &str) {
    for line in content.lines() {
        let mut new_line = String::new();

        let mut blank_count = 0;
        for character in line.chars() {
            if character.eq(&' ') {
                match blank_count {
                    0 => {
                        new_line.push(character);
                        blank_count += 1;
                    }
                    _ => {
                        blank_count += 1;
                    }
                }
            } else {
                new_line.push(character);
                blank_count = 0;
            }
        }

        println!("{}", new_line);
    }
}

fn main() {
    let args = env::args();

    if args.len() > 1 {
        for (count, arg) in args.skip(1).enumerate() {
            if count > 0 {
                println!("\n");
            }

            if arg.eq("-") {
                println!("Input: Stdin");
            } else {
                println!("Input: {}", arg);
            }

            let content = get_content(&arg).unwrap();
            replace_blanks_and_print(&content);
        }
    } else {
        let content = get_content("-").unwrap();
        replace_blanks_and_print(&content);
    }
}
