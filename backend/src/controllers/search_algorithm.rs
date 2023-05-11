fn extract_tags(text: &str) -> (Vec<String>, Vec<String>) {
    let mut hashtags = Vec::new();
    let mut cashtags = Vec::new();

    for token in text.split_whitespace() {
        if token.starts_with('#') {
            hashtags.push(token[1..].to_string());
        } else if token.starts_with('$') {
            cashtags.push(token[1..].to_string());
        }
    }
    (hashtags, cashtags)
}
