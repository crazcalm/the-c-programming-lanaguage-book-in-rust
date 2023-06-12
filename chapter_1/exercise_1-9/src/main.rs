/// Problem:
/// --------
///
/// How would you test the word count program? What are some boundaries?
///
/// Notes:
/// I have to convert that program to Rust first, then I can write tests for it.
use std::env;
use std::fs;
use std::io;

enum WordState {
    InWord,
    NotInWord,
}

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

fn word_count(content: &str) -> (u32, u32, u32) {
    let mut word_state = WordState::NotInWord;

    let mut new_line_count = 0;
    let mut char_count = 0;
    let mut word_count = 0;

    for character in content.chars() {
        char_count += 1;

        if character.eq(&'\n') {
            new_line_count += 1;
        }

        match character {
            '\n' | ' ' | '\t' => word_state = WordState::NotInWord,
            _ => match word_state {
                WordState::NotInWord => {
                    word_state = WordState::InWord;
                    word_count += 1;
                }
                WordState::InWord => {}
            },
        }
    }

    return (new_line_count, word_count, char_count);
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

            let (line_count, word_count, char_count) = word_count(&content);

            println!(
                "Line Count: {}, Word Count: {}, Char Count: {}",
                line_count, word_count, char_count
            );
        }
    } else {
        let content = get_content("-").unwrap();

        let (line_count, word_count, char_count) = word_count(&content);
        println!(
            "Line Count: {}, Word Count: {}, Char Count: {}",
            line_count, word_count, char_count
        );
    }
}

#[cfg(test)]
mod tests {
    use super::word_count;

    #[test]
    fn test_word_count() {
        let cases = vec![
            ("", 0, 0, 0),
            ("My name is Marcus\n", 1, 4, 18),
            ("  \n\n\t", 2, 0, 5),
            ("What\nis\nup\twith\nyou \n", 4, 5, 21),
        ];

        for (num, (content, ex_line_count, ex_word_count, ex_char_count)) in
            cases.iter().enumerate()
        {
            let result = word_count(&content);

            let expected = (
                ex_line_count.clone(),
                ex_word_count.clone(),
                ex_char_count.clone(),
            );
            assert_eq!(
                result, expected,
                "Case: {} -- Got {:?}, but expected {:?} for \"{}\"",
                num, result, expected, content,
            );
        }
    }
}
