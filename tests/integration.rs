use fixture::{add, addition, double};

#[test]
fn add_works_from_outside() {
    assert_eq!(add(40, 2), 42);
}

#[test]
fn double_works_from_outside() {
    assert_eq!(double(5), 10);
}

#[test]
fn addition_works_from_outside() {
    assert_eq!(addition(40, 2), 42);
}
