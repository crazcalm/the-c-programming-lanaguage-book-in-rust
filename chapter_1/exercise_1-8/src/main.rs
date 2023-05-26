/// Problem:
/// --------
///
/// Write a program to replace each tab by the
/// three character sequence >, backspace, -, which prints >- (overlapped),
/// and each backspace by the similar sequence
/// <- (overlapped). This makes tabs and backspaces visible.
///
/// I cannot figure out how to create the wanted arrows, so I am going to use these; ⇐ ⇒  ("\u{21D0}", "\u{21D2}").
/// Backspace is \u{0008}
///
/// Notes:
/// I can type backspase with ^H in stdin and have it work, but I cannot do it in my text editor...
use std::env;
use std::fs;
use std::io;

fn get_content(file_path: &str) -> io::Result<String> {
    if file_path.eq("-") {
        let mut input = String::new();
        for line in io::stdin().lines() {
            input.push_str(&line.unwrap());
            input.push_str("\n");
        }

        Ok(input)
    } else {
        fs::read_to_string(file_path)
    }
}

fn replace_tabs_and_backspaces(content: &str) {
    for line in content.lines() {
        let mut new_line = String::new();

        for character in line.chars() {
            if character.eq(&'\t') {
                new_line.push('\u{21D0}');
            } else if character.eq(&'\u{0008}') {
                new_line.push('\u{21D2}');
            } else {
                new_line.push(character);
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
            replace_tabs_and_backspaces(&content);
        }
    } else {
        let content = get_content("-").unwrap();
        replace_tabs_and_backspaces(&content);
    }
}
