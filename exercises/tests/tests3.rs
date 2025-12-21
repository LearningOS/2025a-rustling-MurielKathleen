// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数情况：应该返回 true
        assert!(is_even(2));   // 2 是偶数
        assert!(is_even(0));   // 0 是偶数
        assert!(is_even(-4));  // -4 是偶数
        assert!(is_even(100)); // 100 是偶数
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数情况：应该返回 false
        assert!(!is_even(1));   // 1 是奇数
        assert!(!is_even(-3));  // -3 是奇数
        assert!(!is_even(99));  // 99 是奇数
        // 特别测试题目要求的 is_even(5)
        assert!(!is_even(5));   // 5 是奇数，应该返回 false
    }
}
