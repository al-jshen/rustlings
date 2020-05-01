// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    fn calculate_apple_price(price: i64) -> bool {
        true
    }
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(true, calculate_apple_price(35));
    }
}
