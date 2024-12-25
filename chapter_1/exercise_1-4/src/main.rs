/* print Celsius-Fahrenheit table
for cels = 0, 20, ..., 300 */

fn main() {
    let lower = 0.0; // lower limit of temparature table
    let upper = 300.0; // upper limit
    let step = 20.0; // step size

    let mut celsius = lower;

    println!("{:>4} {:>6}", "C", "F");

    while celsius <= upper {
        let fahr = (celsius * (9.0 / 5.0)) + 32.0;
        println!("{:4.0} {:6.1}", celsius, fahr);
        celsius = celsius + step;
    }
}
