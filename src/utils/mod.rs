pub mod gen_hex;

// Maybe I don't need this
// {:#?} format string can do the same thing, maybe
pub fn escape_string(str: String) -> String {
    let mut escaped_str = String::new();
    for c in str.chars() {
        match c {
            '\n' => {
                escaped_str.push_str("\\n");
            }
            '\r' => {
                escaped_str.push_str("\\r");
            }
            '\t' => {
                escaped_str.push_str("\\t");
            }
            '\0' => {
                escaped_str.push_str("\\0");
            }
            '\\' => {
                escaped_str.push_str("\\\\");
            }
            _ => {
                escaped_str.push(c);
            }
        }
    }
    return escaped_str;
}
