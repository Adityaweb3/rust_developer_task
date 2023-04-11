fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = String::new();
    let first_str = strs[0];
    for (i, c) in first_str.chars().enumerate() {
        for str in strs.iter().skip(1) {
            if let Some(ch) = str.chars().nth(i) {
                if ch != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }
    prefix
}

fn main() {
    let strs = vec!["flower", "flow", "flight"];
    let longest_prefix = longest_common_prefix(&strs);
    println!("Longest common prefix: {}", longest_prefix);
}
