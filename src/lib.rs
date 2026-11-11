//! Trivial, dependency-free library so `cargo build` and `cargo test` are fast.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn double(a: i32) -> i32 {
    a * 2
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
}
