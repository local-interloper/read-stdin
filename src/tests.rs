#[cfg(test)]
mod tests {
    use crate::{once, until_ok, prompt, prompt_until_ok};

    #[test]
    fn get_u32() {
        let Ok(n) = once::<u32>() else {
            panic!("Incorrect data type!");
        };
        assert_eq!(n, 42);
    }

    #[test]
    fn get_i32() {
        let Ok(n) = once::<i32>() else {
            panic!("Incorrect data type!");
        };
        assert_eq!(n, -42);
    }

    #[test]
    fn get_u32_loop() {
        let n = until_ok::<u32>();
        assert_eq!(n, 42);
    }

    #[test]
    fn get_i32_loop() {
        let n = until_ok::<i32>();
        assert_eq!(n, -42);
    }

    #[test]
    fn get_i32_prompt() {
        let result = prompt::<i32>("Enter 42: ");

        assert_eq!(result.unwrap(), 42);
    }
    
    #[test]
    fn get_i32_prompt_until_ok() {
        let result = prompt_until_ok::<i32>("Enter 42: ");

        assert_eq!(result, 42);
    }
}
