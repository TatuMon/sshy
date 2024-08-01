pub fn del_word(str: String) -> String {
    let last_whitespace_idx = str.rfind(' ');
    match last_whitespace_idx {
        None => String::new(),
        Some(idx) => (&str[..idx]).to_string()
    }
}
