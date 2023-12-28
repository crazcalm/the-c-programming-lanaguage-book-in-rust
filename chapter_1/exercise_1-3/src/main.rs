/*
exercise 1.3: Modify the tempature conversion program to print a heading.

exercise 1.4: Write a program to print the corresponding Celsius to Fahrenheit table.
*/

struct CelsToFahr {
    cels: f32,
    upper: f32,
    step: f32,
}

impl Default for CelsToFahr {
    fn default() -> Self {
        Self {
            cels: -20.0,
            upper: 150.0,
            step: 10.0,
        }
    }
}

impl Iterator for CelsToFahr {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.cels <= self.upper {
            true => {
                let result = (self.cels, self.cels * (9.0 / 5.0) + 32.0);
                self.cels += self.step;
                Some(result)
            }
            false => None,
        }
    }
}

struct FahrToCels {
    fahr: f32,
    upper: f32,
    step: f32,
}

impl Iterator for FahrToCels {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.fahr <= self.upper {
            true => {
                let result = (self.fahr, (5.0 / 9.0) * (self.fahr - 32.0));
                self.fahr += self.step;
                Some(result)
            }
            false => None,
        }
    }
}

impl Default for FahrToCels {
    fn default() -> Self {
        Self {
            fahr: 0.0,
            upper: 300.0,
            step: 20.0,
        }
    }
}

fn print_table(iterator: impl Iterator<Item = (f32, f32)>, header: (&str, &str)) {
    println!("{:>3} {:>6}", header.0, header.1);
    for (fahr, cels) in iterator {
        println!("{fahr:3.0} {cels:6.1}")
    }
}

fn main() {
    print_table(FahrToCels::default(), ("F", "C"));

    println!("\n\n\n");

    print_table(CelsToFahr::default(), ("C", "F"));
}
