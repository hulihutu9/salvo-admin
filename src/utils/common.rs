// 字符串转换为CamelCase
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
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