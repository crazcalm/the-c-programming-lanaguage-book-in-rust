/*
exercise 1.3: Modify the tempature conversion program to print a heading.

exercise 1.4: Write a program to print the corresponding Celsius to Fahrenheit table.

exercise 1.5: modify the temparature conversion program to print the table in reverse
              order, that is, from 300 degrees to 0.
*/

#[derive(Debug, Clone)]
struct CelsToFahr {
    lower: f32,
    upper: f32,
    step: f32,
}

impl Default for CelsToFahr {
    fn default() -> Self {
        Self {
            lower: -20.0,
            upper: 150.0,
            step: 10.0,
        }
    }
}

impl Iterator for CelsToFahr {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.lower <= self.upper {
            true => {
                let result = (self.lower, self.lower * (9.0 / 5.0) + 32.0);
                self.lower += self.step;
                Some(result)
            }
            false => None,
        }
    }
}

impl DoubleEndedIterator for CelsToFahr {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.lower <= self.upper {
            true => {
                let result = (self.upper, self.upper * (9.0 / 5.0) + 32.0);
                self.upper -= self.step;
                Some(result)
            }
            false => None,
        }
    }
}

#[derive(Debug, Clone)]
struct FahrToCels {
    lower: f32,
    upper: f32,
    step: f32,
}

impl Iterator for FahrToCels {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.lower <= self.upper {
            true => {
                let result = (self.lower, (5.0 / 9.0) * (self.lower - 32.0));
                self.lower += self.step;
                Some(result)
            }
            false => None,
        }
    }
}

/*
In order to get reverse, I had to implement a double ended iterator trait.
Doing so allowed me to use the same print function for both forward and backward iteration.
*/
impl DoubleEndedIterator for FahrToCels {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.lower <= self.upper {
            true => {
                let result = (self.upper, (5.0 / 9.0) * (self.upper - 32.0));
                self.upper -= self.step;
                Some(result)
            }
            false => None,
        }
    }
}

impl Default for FahrToCels {
    fn default() -> Self {
        Self {
            lower: 0.0,
            upper: 300.0,
            step: 20.0,
        }
    }
}

fn print_table(iterator: impl DoubleEndedIterator<Item = (f32, f32)>, header: (&str, &str)) {
    println!("{:>3} {:>6}", header.0, header.1);
    for (fahr, cels) in iterator {
        println!("{fahr:3.0} {cels:6.1}")
    }
}

fn main() {
    /*
    Notes:
    - The iterators are updating themselves during iteration, so they
    have to be mutable.
    - This also means that they should only be consumed once.

    Though you can pass a mut reference of your iterator to your function
    to prevent that function from taking ownership of your iterator, the
    internal state of the struct has been modified via the first iteration.
    As as result, the second iteration will use the new state, which is
    not what you want.

    A work around is to "clone" the struct before using it as an iterator
    so that the initial internal state of the struct is preserved for each
    iteration. This works because each clone is a new instatance of the
    original struct.

    - If I want to chain iterators, then I first need to call "into_iter"
    on my struct. This will return the iterator, which will then give me
    access to all the iterator methods. We can do this because all
    iterators implement the IntoIterator trait.
    */
    let fahr_to_cels = FahrToCels::default();
    let cels_to_fahr = CelsToFahr::default();

    print_table(&mut fahr_to_cels.clone(), ("F", "C"));

    println!("\n\n\n");

    print_table(&mut fahr_to_cels.into_iter().rev(), ("F", "C"));

    println!("\n\n\n");

    print_table(&mut cels_to_fahr.clone(), ("C", "F"));

    println!("\n\n\n");

    print_table(&mut cels_to_fahr.into_iter().rev(), ("C", "F"));
}
