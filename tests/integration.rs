use fixture::{add, double};

#[test]
fn add_works_from_outside() {
    assert_eq!(add(40, 2), 42);
}

#[test]
fn double_works_from_outside() {
    assert_eq!(double(5), 10);
}

#[test]
fn add_handles_negative_numbers() {
    assert_eq!(add(-3, -4), -7);
}

#[test]
fn double_handles_negative_numbers() {
    assert_eq!(double(-5), -10);
}

#[test]
fn add_and_double_zero_are_zero() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(double(0), 0);
}
