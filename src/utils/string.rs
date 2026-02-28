pub fn sanitize_filename(filename: &str) -> String {
    filename
        .replace(['\'', '\"'], "")
        .replace(['/', '\\', ':', '*', '?', '<', '>', '|'], "_")
}

pub fn extract_list_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("<unknown>")
        .to_string()
}

pub fn cut_at(input: &str, delim: char) -> &str {
    input.split(delim).next().unwrap_or(input).trim()
}

pub fn remove_extension(input: &str) -> &str {
    input
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(input)
}

pub fn clean_all(input: &str) -> String {
    let input = cut_at(input, '(');
    let input = cut_at(input, '[');
    let input = remove_extension(input);

    input.trim().to_string()
}
