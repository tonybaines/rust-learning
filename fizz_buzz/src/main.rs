fn fizz_buzz(i: u32) -> String {
    if i % 15 == 0 {
        "fizz-buzz".to_string()
    }
        else if i % 3 == 0 {
        "fizz".to_string()
    }
        else if i % 5 == 0 {
        "buzz".to_string()
    } else {
        i.to_string()
    }
}

fn main() {
    for n in 1..100 {
        println!("{0} => {1}", n, fizz_buzz(n))
    }
}

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
