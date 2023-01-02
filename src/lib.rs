mod tests;

use std::any::TypeId;
use std::io::stdin;
use std::io::BufRead;
use std::str::FromStr;

/// Read user input from `stdin` and try to parse it into a generic type.
/// 
/// # Example
/// ```rust
/// use read_stdin::read_stdin;
/// 
/// let Ok(n) = read_stdin::<i32>() else {
///     println!("You entered an incorrect data type!");
///     return;
/// };
/// 
/// println!("You entered: {}", n)
/// ```
pub fn read_stdin<T>() -> Result<T, <T as FromStr>::Err>
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
/// use read_stdin::read_stdin_until_ok;
/// 
/// let n = read_stdin_until_ok::<i32>();
/// 
/// println!("You entered: {}", n)
/// ```
pub fn read_stdin_until_ok<T>() -> T
where
    T: FromStr + 'static,
{
    loop {
        match read_stdin::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Failed to parse. Please input a correct data type.");
                continue;
            }
        }
    }
}
