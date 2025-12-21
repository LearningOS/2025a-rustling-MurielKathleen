// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    // 问题1: 对 None 调用 unwrap() 会导致 panic
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap(); // 这会 panic!
        // 应该删除这行或使用其他处理方式
    }

    // 问题2: 数组初始化语法错误，缺少逗号
    let my_arr = &[
        -1, -2, -3,  // 这里需要逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 问题3: resize() 方法不返回值，它会原地修改
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);  // resize 返回 ()，需要分开写
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 问题4: 交换两个变量的值，但逻辑错误
    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;  // 这会丢失 value_a 的原始值
    // value_b = value_a;  // 这里 value_b 会被赋值为自己
    std::mem::swap(&mut value_a, &mut value_b);  // 正确的交换方式
    println!("value a: {}; value b: {}", value_a, value_b);
}
