// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Any<T> {
    value: T,
}

impl<T> Any<T> {
    pub fn new(value: T) -> Self {
        Any { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Any::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Any::new("Foo").value, "Foo");
    }
}
