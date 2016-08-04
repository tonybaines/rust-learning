extern crate fizz_buzz;

use fizz_buzz::*;

#[test]
fn integration_test() {
    assert_eq!("1", fizz_buzz(1));


    for n in 1..100 {
        println!("{0} => {1}", n, fizz_buzz(n))
    }
}