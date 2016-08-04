pub fn is_unreachable() {
    unreachable!()
}

pub fn might_fail_simple() -> Option<String> {
    None
}

#[derive(Debug)]
pub struct MyError {
    msg: String
}

pub fn might_fail_complex() -> Result<String, MyError> {
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
        assert_eq!("Nothing", match might_fail_simple() {
            Some(_) => "Something",
            None => "Nothing",
        })
    }

    #[test]
    fn complex() {
        assert_eq!("Error: MyError { msg: \"Oh dear!\" }".to_string(), match might_fail_complex() {
            Ok(_) => "Success".to_string(),
            Err(e) => (format!("Error: {:?}", e)),
        })
    }
}



