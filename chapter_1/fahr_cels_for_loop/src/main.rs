fn main() {
    for fahr in (0..=300).step_by(20) {
        println!("{:4} {:6.1}", fahr, (5.0 / 9.0) * (fahr - 32) as f32);
    }
}
