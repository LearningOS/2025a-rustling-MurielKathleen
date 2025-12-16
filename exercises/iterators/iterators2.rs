// Step 1：首字母大写
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2：对切片中的每个单词调用 capitalize_first，收集成 Vec<String>
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&w| capitalize_first(w)).collect()
}

// Step 3：同上，但把结果拼成一个 String
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&w| capitalize_first(w)).collect()
}
