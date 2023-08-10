// don't need to use #[cfg(test)]
// cargo only compiles the test/ directory when we run `cargo test`
// each file in the tests directory is compiled as its own separate crate
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // calling our setup fn from common/mod.rs
    assert_eq!(4, adder::add_two(2));
}