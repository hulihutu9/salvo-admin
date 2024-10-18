pub fn to_pascal_case(s: &str) -> String {
    let mut result= String::new();
    let mut capitalize_next_char = true;
    for c in s.chars() {
        if c.is_whitespace() || c.is_ascii_punctuation() {
            capitalize_next_char = true;
        } else if capitalize_next_char {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next_char = false;
        } else {
            result.push(c);
        }
    }
    result
}

// 返回2个字符之间的子字符串
pub fn sub_string_between(s: &str, start_char: &str, end_char: &str) -> String {
    let start = match s.find(start_char) {
        Some(i) => i,
        None => return "".to_string(),
    };
    if start > 0 {
        let end = s.find(end_char).unwrap_or(0);
        s[start + 1..end].to_string()
    } else {
        "".to_string()
    }
}