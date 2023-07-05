use guesser_testing;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, guesser_testing::add_two(2));
}