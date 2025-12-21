// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 如果启用了 "pass" 特性，直接返回（测试通过）
        #[cfg(feature = "pass")]
        return;

        // 如果没有启用 "pass" 特性，panic（测试失败）
        panic!("no cfg set");
    }
}
