# read-stdin

## About
read-stdin is a small set of functions I wrote once I got sick of writing code for handling user input in a terminal so I decided to publish it online for anyone to enjoy.

## Examples
You can use `read_stdin` to ask the user for input. This function will return a `Result` that will be `Ok` if the user enters something that successfuly parses into a generic type of your choice, and `Err` if it fails parsing.

```rust
use read_stdin::read_stdin;

let Ok(n) = read_stdin::<i32>() else {
println!("You entered an incorrect data type!");
return;
};

println!("You entered: {}", n)
```

You can also use `read_stdin_util_ok` if you wish to annoy the user until their data parses correctly.

```rust
use read_stdin::read_stdin_until_ok;

let n = read_stdin_until_ok::<i32>();

println!("You entered: {}", n)
```