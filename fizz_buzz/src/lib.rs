///! Fizz-Buzz is a game of division

/// # Examples
///
/// ```
/// use fizz_buzz::fizz_buzz;
///
/// assert_eq!("1", fizz_buzz(1));
/// assert_eq!("fizz", fizz_buzz(6));
/// assert_eq!("buzz", fizz_buzz(10));
/// assert_eq!("fizz-buzz", fizz_buzz(60));
/// ```
///
pub fn fizz_buzz(i: u32) -> String {
    match i {
        n if n % 15 == 0 => "fizz-buzz".to_string(),
        n if n % 3 == 0 => "fizz".to_string(),
        n if n % 5 == 0 => "buzz".to_string(),
        _ => i.to_string()
    }
}

#[cfg(test)]
mod fizzbuzz_tests {
    use super::*;

    #[test]
    fn one_is_not_fizz() {
        assert_eq!("1", fizz_buzz(1))
    }

    #[test]
    fn three_is_fizz() {
        assert_eq!("fizz", fizz_buzz(3))
    }

    #[test]
    fn five_is_buzz() {
        assert_eq!("buzz", fizz_buzz(5))
    }

    #[test]
    fn fifteen_is_fizzbuzz() {
        assert_eq!("fizz-buzz", fizz_buzz(15))
    }
}