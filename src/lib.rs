mod tests;

use std::any::TypeId;
use std::fmt::Display;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

/// Read user input from `stdin` and try to parse it into a generic type.
///
/// # Example
/// ```rust
/// use read_stdin;
///
/// let Ok(n) = read_stdin::once::<i32>() else {
///     println!("You entered an incorrect data type!");
///     return;
/// };
///
/// println!("You entered: {}", n)
/// ```
pub fn once<T>() -> Result<T, <T as FromStr>::Err>
where
    T: FromStr + 'static,
{
    let mut stdin = stdin().lock();
    let mut buf: String = String::new();

    while stdin.read_line(&mut buf).is_err() {
        println!("Failed to read line. Please try again.");
    }

    if TypeId::of::<T>() != TypeId::of::<String>() {
        buf = buf.trim().to_string();
    }

    buf.parse::<T>()
}

/// Read user input from `stdin` and try to parse it into a generic type. I will keep prompting
/// the user for input until they enter data that properly parses.
///
/// # Example
/// ```rust
/// use read_stdin;
///
/// let n: i32 = read_stdin::until_ok();
///
/// println!("You entered: {}", n)
/// ```
pub fn until_ok<T>() -> T
where
    T: FromStr + 'static,
{
    loop {
        match once::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Failed to parse. Please input a correct data type.");
                continue;
            }
        }
    }
}

/// Creates a CLI prompt and asks the user for input.
///
/// # Example
/// ```rust
/// use read_stdin;
///
/// let n: i32 = read_stdin::prompt("Enter a number: ");
///
/// println!("You entered: {}", n)
/// ```
pub fn prompt<T>(prompt: &(impl ToString + Display + ?Sized)) -> Result<T, <T as FromStr>::Err>
where
    T: FromStr + 'static,
{
    let mut stdout = stdout().lock();

    print!("{}", prompt);
    stdout.flush().expect("Failed to flush stdout");
    once()
}

/// Creates a CLI prompt and repeatetly asks the user for input until their input is of valid type.
///
/// # Example
/// ```rust
/// use read_stdin;
///
/// let n: i32 = read_stdin::prompt("Enter a number: ");
///
/// println!("You entered: {}", n)
/// ```
pub fn prompt_until_ok<T>(prompt: &(impl ToString + Display + ?Sized)) -> T
where
    T: FromStr + 'static,
{
    let mut stdout = stdout().lock();

    loop {
        print!("{}", prompt);
        stdout.flush().expect("Failed to flush stdout");
        match once::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Failed to parse. Please input a correct data type.");
                continue;
            }
        }
    };
}

// Compatibility
#[deprecated(since = "1.1.0", note = "Use `until_ok` instead.")]
pub fn read_stdin_until_ok<T>() -> T
where
    T: FromStr + 'static,
{
    until_ok()
}

#[deprecated(since = "1.1.0", note = "Use `once` instead.")]
pub fn read_stdin<T>() -> Result<T, <T as FromStr>::Err>
where
    T: FromStr + 'static,
{
    once()
}
