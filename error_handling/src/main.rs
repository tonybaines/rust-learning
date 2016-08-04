pub fn is_unreachable() {
    unreachable!()
}

pub fn will_fail_simple() -> Option<String> {
    None
}

#[derive(Debug)]
pub struct MyError {
    msg: String
}

pub fn will_fail_complex() -> Result<String, MyError> {
    return Err(MyError {
        msg: "Oh dear!".to_string()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn panic() {
        is_unreachable()
    }

    #[test]
    fn simple() {
        assert_eq!("Nothing", match will_fail_simple() {
            Some(_) => "Something",
            None => "Nothing",
        })
    }

    #[test]
    fn complex() {
        assert_eq!("Error: MyError { msg: \"Oh dear!\" }".to_string(), match will_fail_complex() {
            Ok(_) => "Success".to_string(),
            Err(e) => (format!("Error: {:?}", e)),
        })
    }

    #[test]
    #[should_panic]
    fn complex_unwrap() {
        let _x = will_fail_complex().unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected")]
    fn complex_ok_expect() {
        let _x = will_fail_complex()
            .ok()
            .expect("Expected");
    }

    // Always succeed
    fn will_succeed_complex() -> Result<String, MyError> {
        Ok("Success!".to_string())
    }

    fn wrap_with_try() -> Result<String, MyError> {
        // Any failure will result in an error
        try!(will_succeed_complex());
        try!(will_fail_complex());
        try!(will_succeed_complex());

        Ok("Eh?".to_string())
    }

    #[test]
    fn complex_try() {
        assert_eq!("Failure", match wrap_with_try() {
            Ok(_) => "Success",
            Err(_) => "Failure",
        })
    }
}



