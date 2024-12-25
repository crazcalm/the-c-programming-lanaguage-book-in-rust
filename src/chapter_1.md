# Chapter 1
## Hello World
The first chapter of The C Programming Language Book starts off as a tutorial introduction with the goal of introducing the gist of the language via a myriad of examples. Bound by programming tradition, is hello world.

In C, the program to print "hello, world" is:
```C
{{#include ../chapter_1/c-programs/hello_world/hello.c}}
```

After placing the above code into a file called `hello.c`, you can run `cc hello.c` to compile the code, which will produce and `a.out` executable file. Calling that executable (`./a.out`), will print "hello, world" to your terminal screen.

```
❯ ./a.out 
hello, world
```

For us in the Rust world, we have Cargo to aid us in the initizalization of a new project. Running `cargo new hello_world` will create a new Rust project directory named `hello_world` with the following file structure.

```
hello_world/
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

The main.rs file was generated with Rust's default hello world program, which has a slightly different output than the C program we wrote. However, with a slight tweak to the character string, we can make it the same.

```rust
{{#include ../chapter_1/hello_world/src/main.rs}}
```

## Hello World Part 2
The C book, being an introduction to the C language, takes this opportunity to explain `printf`, the `"hello, world\n"` passed into it, and other valid programs that can give you the same output.

For example, since `printf` needs `\n` to dictate new line feeds, our previous hello world program could have been written by breaking up the strings and passing it into different invocations of the `printf` function.

```c
{{#include ../chapter_1/c-programs/hello_world_part_2/hello_2.c}}
```

In Rust, we can do the same thing, but we first have to acknowledge the difference between the `println!` and `print!`. The `println!` macro prints to standard output with a newline, meaning any string passed into this macro will have a `\n` appended to the end of it. Whereas the `print!` aligns more with C's `printf` in the sense that it will print to standard out without adding a `\n` to the string.

```rust
{{#include ../chapter_1/hello_world_part_2/src/main.rs}}
```

Now the Rust and C programs look almost identical. To be fair, the copy of The C Programming language book that I own does not include the `#include <stdio.h>` in their hello world programs. I do not know of that was a mistake on their part or if, over the course of time, a change was made to the C language that made this import required. Either way, I know that their exist a world where the hello world programs in Rust and C are identical.

## Variables and Arithmetic

The next program prints a Fahrenheit to Celsius temperature table. 

```c
{{#include ../chapter_1/c-programs/fahr_to_cels/fahr_cels.c}}
```

This program is meant to display the use of comments, varible declaration, string formatting, and introduce looping via the while loop. Most of the programming concepts used in the example are explicit like the declaration of integers and floats, but the type conversions should be pointed out. For example, `fahr` is a float while `lower` is and integer, and yet we can set `fahr` equal to `lower` (`fahr = lower`). This works because there is an implicit type conversion from int to float to satisfy the type of `fahr`. A similar thing happens in the `while` loop when we compare `fahr` and `upper` (`fhar <= upper`). These are two different types, but we are able to compare them because an implicit type conversion is being done prior to the comparision being made.

The last thing that should be noted for this example is the print formating. The `%` are used for varible substitution. That is how `fahr` and `celcius` are being passed into the string. The `4.0f` and the `6.1f` are examples of width and precision formatting for floats. Reading it backwards, we have `f`, which stands for `float`, `.0` and `.1`, which means we want a precision of 0 for the first flaot and a precision of 1 for the seconds float (Note: precision is the number of decimal places that you would like to show), and we have `4` and `6`, which are the minimum number of spaces these floats can take up. Looking at the output, we can see that the Celsuis values all show a digit in the tenths place while the Fahrenheit values do not. 

```
❯ ./a.out                                                                                  
   0  -17.8                                                                                
  20   -6.7                                                                                
  40    4.4                                                                                
  60   15.6                                                                                
  80   26.7                                                                                
 100   37.8                                                                                
 120   48.9                                                                                
 140   60.0                                                                                
 160   71.1                                                                                
 180   82.2                                                                                
 200   93.3                                                                                
 220  104.4                                                                                
 240  115.6                                                                                
 260  126.7                                                                                
 280  137.8                                                                                
 300  148.9
```

We can do all this in Rust, but we can't do it all the same way. Rust does not do implcit type conversions from int to float, which means that we have to explicitly cast ints to floats or vice versa to makse sure all the compared types match or we can change all the ints in this program to floats.

We also have to handle the mutability of the `fahr` variable. In the C program, we declare the type of our variables, but we never had to make the distinction of which variables are mutable and which variables are not. In Rust, however, we do have to make this disctinction. As such, the `fahr` variable should be declared as mutable in our Rust program.

```rust
{{#include ../chapter_1/fahr_to_cels/src/main.rs}}
```

The Rust print formatting syntax is not too different from C. For example, the `4.0` in `{:4.0}` means the same thing it did in the C code. The differences are that type did not need to be specified, which resulted in the `f` being dropped. The `4.0` is behind a `:` in Rust because the value in front of the `:` specifies which argument will be used for this string substitution and, when not specified (like we are doing here), it defaults to index numbers of the passed in arguments in order. So `println!("{:4.0} {:6.1}", fahr, celsius);` is equal to `println!("{0:4.0} {1:6.1}", fahr, celsius);` where 0 points to the `fahr` argument and 1 points to the `celsius` argument.
