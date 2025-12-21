//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    
    // 设置环境变量 TEST_FOO，值为时间戳字符串
    // Cargo 构建脚本使用特殊格式的输出来与 Cargo 通信
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 重新运行构建脚本的条件：当这个文件改变时
    println!("cargo:rerun-if-changed=build.rs");

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
