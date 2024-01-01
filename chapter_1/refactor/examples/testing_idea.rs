fn main() {
    let input = "Hi!\nMy name is Marcus.\nWhat is your name?\n";

    input
        .split('\n')
        .map(|x| x.chars().collect::<Vec<_>>())
        .flatten()
        .for_each(|x| println!("{x}"));
}
