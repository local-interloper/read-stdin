#[cfg(test)]
mod tests {
    use crate::{read_stdin_until_ok, read_stdin};

    #[test]
    fn get_u32() {
        let Ok(n) = read_stdin::<u32>() else {
            panic!("Incorrect data type!");
        };
        assert_eq!(n, 42);
    }

    #[test]
    fn get_i32() {
        let Ok(n) = read_stdin::<i32>() else {
            panic!("Incorrect data type!");
        };
        assert_eq!(n, -42);
    }

    #[test]
    fn get_u32_loop() {
        let n = read_stdin_until_ok::<u32>();
        assert_eq!(n, 42);
    }

    #[test]
    fn get_i32_loop() {
        let n = read_stdin_until_ok::<i32>();
        assert_eq!(n, -42);
    }
}
