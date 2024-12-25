/* print Fahrenheit-Celsius table
for f = 0, 20, ..., 300 */

fn main() {
    let lower = 0.0; // lower limit of temparature table
    let upper = 300.0; // upper limit
    let step = 20.0; // step size

    let mut fahr = lower;
    while fahr <= upper {
        let celsius = (5.0 / 9.0) * (fahr - 32.0);
        println!("{:4.0} {:6.1}", fahr, celsius);
        fahr = fahr + step;
    }
}
