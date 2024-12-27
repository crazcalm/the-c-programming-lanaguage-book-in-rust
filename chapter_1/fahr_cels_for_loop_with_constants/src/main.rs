const LOWER: i32 = 0; // lower limit of table
const UPPER: i32 = 300; // upper limit
const STEP: usize = 20; // step size

fn main() {
    for fahr in (LOWER..=UPPER).step_by(STEP) {
        println!("{:4} {:6.1}", fahr, (5.0 / 9.0) * (fahr - 32) as f32);
    }
}
