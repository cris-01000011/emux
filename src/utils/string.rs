pub fn remove_parentheses(input: &str) -> String {
    input.split('(').next().unwrap_or(input).trim().to_string()
}

pub fn remove_brackets(input: &str) -> String {
    input.split('[').next().unwrap_or(input).trim().to_string()
}

pub fn clean_all(input: &str) -> String {
    let input = remove_parentheses(input);
    let input = remove_brackets(&input);
    input
}
