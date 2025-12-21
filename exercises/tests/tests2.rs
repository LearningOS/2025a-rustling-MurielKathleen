// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 让测试通过：两个相等的值
        // 让测试失败：两个不相等的值
        assert_eq!(42, 42);  // 通过
        // assert_eq!(42, 43);  // 失败
    }
}
