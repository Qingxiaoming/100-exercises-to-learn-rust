
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn Rust!"
}

// Your solutions will be automatically verified by a set of tests.
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// from the root of this exercise's directory. That's what the `wr` command does for you
// under the hood.
//
// Rust lets you write tests alongside your code.
// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
// You'll learn more about attributes and testing later in the course.
// For now, just know that you need to look for the `#[cfg(test)]` attribute to find the tests
// that will be verifying the correctness of your solutions!
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// tested, not the tests themselves.
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
