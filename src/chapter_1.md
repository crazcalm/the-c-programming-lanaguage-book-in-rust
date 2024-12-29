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

## Exercise 1-3
Modify the temperature conversion program to print a heading above the table.

To complete this exercise, we can add a print statement before the loop that creates the table. In C, we do that by adding a `printf` statement: 
```c
{{#include ../chapter_1/c-programs/exercise_1-3/exercise_1-3.c}}
```

```
❯ ./a.out 
  F      C
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

In Rust, I had to fiddle with the `print!` macro a bit to get everything lined up right. In the end, I used the `{:>4}` syntax, where `>4` means that the argument is right-aligned with width 4. 

```rust
{{#include ../chapter_1/exercise_1-3_new/src/main.rs}}
```

## Exercise 1-4
Write a program to print the corresponding Celsius to Fahrenheit table.

This calls for the swapping of `celsius` and `fahr` within our code as well as using the formula to convert Celsius to Fahrenheit. Outside of these changes, the C and Rust programs are more or less identical to the previous version of the code.

```C
{{#include ../chapter_1/c-programs/exercise_1-4/exercise_1-4.c}}
```

```
❯ ./a.out 
  C      F
  0   32.0
 20   68.0
 40  104.0
 60  140.0
 80  176.0
100  212.0
120  248.0
140  284.0
160  320.0
180  356.0
200  392.0
220  428.0
240  464.0
260  500.0
280  536.0
300  572.0
```

This is what it looks like in Rust:

```rust
{{#include ../chapter_1/exercise_1-4/src/main.rs}}
```

## 1.3 The For Statement
This section of the book is all about switching out the `while` loop for a `for` loop in out temperature program.

```C
{{#include ../chapter_1/c-programs/fahr_cels_for_loop/fahr_cels_for_loop.c}}
```

The for loop is broken up into three parts; the initialization part (`fahr = 0;`), the conditional part that dictates whether or not the loop ends (`fahr <= 300;`) and the step that increments of decrements the looping variable (`fahr = fahr + 20`). It should also be noted that `fahr` got changed to an int and is now represented as an digit (`d`) in the string formatting (`printf("%4d")`).

Rust does not have the same type of `for` loop as C. Our looping mechanisms take an iterator that produces the values we need. The Rust standard library does have a `Range` operator that will allow us to create and iterator with numbers from 0 to 300, but the `Range` operator does not have a step functionality. However, iterators themselves have a `step_by` method that will allow us to advance through unwanted steps of the iteration. In using this, our code will look like the C code, but unlike the C for loop, we will be iterating through all the numbers between 0 and 300. To be fair, there are other ways of writing this code that are more accurate the to C code operations, but I am choosing not to use them in an attempt to keep the code slim and comparable to the C code.

```rust
{{#include ../chapter_1/fahr_cels_for_loop/src/main.rs}}
```

Another notable difference between the C code is that we had to the `(fahr-32)` to a float to ensure that our multiplication with the float value of `(5.0/9.0)` would work.

## Symbolic Constants

This section uses symbolic constants to replace the magic number in our for loop version of the temperature program.

```C
{{#include ../chapter_1/c-programs/fahr_cels_for_loop_with_constants/fahr_cels_for_loop_with_constants.c }}
```

Rust has constants as well, but they do have to be explicitly typed. It should also be noted that all of our constants are not `i32`. The `step_by` method requires a `usize`, which makes sense because you cannot negatively/unconsume iterators. As such, the `STEP` constant is of type `usize`.

```rust
{{#include ../chapter_1/fahr_cels_for_loop_with_constants/src/main.rs}}
```

## A Collection of Useful Programs

This sections was my first stumbling block in the book because I hyper focus on C's `getchar` and `putchar` functions. For those who don't know, `getchar` allows you to fetch the next input character input (usually from the terminal -- AKA stdin). The `putchar` function is the compliment operations and, in this sections, allows you to write a character to stdout.

On this attempt of going through the book, I'm focusing on end functionality and not code operations.

### File Copying

The simpliest example of a file copy program is one that copies from the source to the destination one character at a time.

```C
{{#include ../chapter_1/c-programs/file_copy/file_copy.c}}
```

The `EOF` (End of File) constants, which is used to represent the value returned at the end of a file, is defined in [stdio.c](https://github.com/bminor/glibc/blob/7fa9e786b6e8f78675ecc30d7eaa200e1ee259b9/libio/stdio.h#L105). Traditionally, `EOF` is either -1 or 0, but more importantly, it should be noted that `EOF` is not of type `char`. This distinction between `int` and `char` is what allows `EOF` to be effective as a end of file marker. It should also be noted that out variable `c` in our program is of type `int`. This is so that it can hold both `chars`, which can be represented as itegers, and `EOF`.

The Rust version of this code does not closely mirror the C code because we handle the consumptions of strings differently. After importing `std::io` to give us access to the stdin input, we are provided with a method that gives us access to the input one line at a time. To match the C code, we then have to extract the `chars` from the line and print them out individually. Due to this unnecessary work, we are forced to manually add the `\n` back into the input after each line iteration. 

```rust
{{#include ../chapter_1/file_copy/src/main.rs }}
```

### Character counting

The character counting section is meant to introduce the `++` incrementor, which means to increment by one. You can write `nc = nc + 1` but `++nc` is more concise. There is also a `--` operator to decrement by 1. The operators `++` and `--` can be either prefix operators (`++nc`) or postfix (`nc++`); The difference between the two is explained in chapter 2.

The character counting programming accumulates its count in `long` variable instead of an `int`. On a PDP-11 the maximum value of an int is 32767, and it would take relatively little input to overflow the counter if it were declared `int`. The conversion specification `%ld` signals to `printf` that the corresponding argument is a `long` integer. To cope with even bigger numbers, you can use a `double` (double length float). We will also use a `for` loop instead of a `while`, to illustrate an laternative way to write the loop.

```C
{{#include ../chapter_1/c-programs/character_counting/character_counting.c }}
```

```
❯ ./a.out < character_counting.c
146
```

Rust has integer system is pretty nice and I probably should have it sooner. So, we have `i32`, `i64`, and `i128`, which are pretty self explanatory, but we also have `isize`. The `isize` type is the i-size that is matched to your operating system. If you are running an 32bit system, then `isize` is `i32`. If you are running a `i64` system, then `isize` is `i64`. The complement all of this also exists for unsigned integers.

For the character counting program, we are going to explicitly set out counting variable to `i128`. We will also have to manually count the `\n` characters that are consumed by the iterator splitting the text by `\n`.

```rust
{{#include ../chapter_1/character_counting/src/main.rs }}
```

### Line Counting

We get our first control flow conditional statement via an `if` statement!

```C
{{#include ../chapter_1/c-programs/line_counting/line_counting.c }}
```

```
❯ ./a.out < line_counting.c
13
```

This Rust program is extremely straight forward because we have been iterating over our content by lines this entire time.

```rust
{{#include ../chapter_1/line_counting/src/main.rs}}
```

### Excercise 1-6
Write a program to count blanks, tabs, and newlines.

This program is a reminder that if there is more than one line in the loop, you must add brackets to enclose the body of the loop...

```C
{{#include ../chapter_1/c-programs/exercise_1-6/exercise_1-6.c}}
```

```
❯ ./a.out < exercise_1-6.c
105
```

The Rust code is not all that out of the ordinary, but I can use this opportunity to state that Rust also uses single quotes to denote a char (For example: `'\n'` is a single char). I should also mention that `char` has a `eq` method to check equality and you must pass a reference to the `char` that you want to check against. So though references will not be covered in the C Programming Language Book for another few sections, you must know about them in Rust if you want to compare equalities.

```rust
{{#include ../chapter_1/exercise_1-6_new/src/main.rs}}
```

### Exercise 1-7 

Write a program to copy its input to its output, replacing each string of one or more blanks by a single blank.

This exercise is a subtle reminder that tabs and spaces are not the same thing and that we were only asked to act on the spaces. So, if you want the output to look "right", you need to not use tabs in your source code.

I do not know if this is "cheating", but I did use a state machine in my solution. With regard to programming techniques, everthing I used has already been introduced, but this techinique gets demonstrated in the next example...

```C
{{#include ../chapter_1/c-programs/exercise_1-7/exercise_1-7.c}}
```

When using the same strategy to solve the exercise in Rust, the only real difference is that I have to add back the new lines to the output so that it looks "correct".

```rust
{{#include ../chapter_1/exercise_1-7_new/src/main.rs}}
```
