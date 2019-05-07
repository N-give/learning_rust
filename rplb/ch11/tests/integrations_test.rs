use ch11;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch11::add_two(2));
}
