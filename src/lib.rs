//! Trivial, dependency-free library so `cargo build` and `cargo test` are fast.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn double(a: i32) -> i32 {
    a * 2
}

pub fn addition(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn doubles() {
        assert_eq!(double(21), 42);
    }

    #[test]
    fn addition_returns_sum_of_two_numbers() {
        assert_eq!(addition(20, 22), 42);
    }
}
