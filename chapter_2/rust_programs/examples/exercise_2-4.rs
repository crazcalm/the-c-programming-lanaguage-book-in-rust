/*
Write an alternative version of squeeze(s1, s2) that deletes each character in s1 that matches any character in string s2.
*/
use std::collections::HashSet;
use std::env;

fn main() {
    let mut args = env::args();
    let string_1: String;
    let string_2: String;

    if args.len() > 1 {
        let _ = args.next();
        string_1 = args.next().expect("Need value for string 1").to_string();
        string_2 = args.next().expect("Need value for string 2").to_string();
    } else {
        panic!("Must pass in values for string_1 and string_2")
    }

    println!("string 1: {}, string 2: {}", string_1, string_2);

    let unwanted: HashSet<char, std::collections::hash_map::RandomState> =
        string_2.chars().collect();
    let result: String = string_1
        .chars()
        .filter(|c| unwanted.get(c).is_none())
        .collect();

    println!("result: {}", result);
}
