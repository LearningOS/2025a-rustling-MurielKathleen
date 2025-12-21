// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 注意这里的分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };  // 这里也要分号（最后一个可选）
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
