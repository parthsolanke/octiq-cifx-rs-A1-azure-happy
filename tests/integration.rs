use fixture::{add, double};

#[test]
fn add_works_from_outside() {
    assert_eq!(add(40, 2), 42);
}

#[test]
fn double_works_from_outside() {
    assert_eq!(double(5), 10);
}
