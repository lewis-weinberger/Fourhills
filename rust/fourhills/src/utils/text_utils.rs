/// Wrap any of lines which are too long, indenting subsequent lines.
///
/// # Parameters
///
/// * `lines: &[String]` - the lines of the string to wrap.
/// * `line_width: usize` - the maximum width a line can be.
///
/// # Returns
///
/// * `Vec<String>` - the wrapped lines of text.
pub fn wrap_lines_paragraph(lines: &[String], line_width: usize) -> Vec<String> {
    let mut wrapped: Vec<String> = Vec::new();
    for line in lines.iter() {
        wrapped.append(&mut wrap_line(line, line_width));
    }
    wrapped
}

/// Pad a string with spaces, centre-aligned.
///
/// # Parameters
///
/// * `s: &str` - string to be centred.
/// * `line_width: usize` - maximum width of a line.
///
/// # Returns
///
/// * `String` - the centred string.
pub fn centre_pad(s: &str, line_width: usize) -> String {
    format!("{:^width$}", s, width = line_width)
}

/// Returns a capitalised string.
/// [https://stackoverflow.com/a/38406885]
///
/// # Parameters
///
/// * `text: &str` - string to be capitalised.
///
/// # Returns
///
/// * `String` - the capitalised string.
pub fn capitalise(text: &str) -> String {
    let mut c = text.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Return a vector of strings representing a wrapped line, with
/// subsequent lines indented by 4 spaces.
///
/// # Parameters
///
/// * `line: &str` - line to be wrapped.
/// * `line_width: usize` - maximum line width.
///
/// # Returns
///
/// * `Vec<String>` - the wrapped lines.
pub fn wrap_line(line: &str, line_width: usize) -> Vec<String> {
    let mut wrapped: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut count = 0;
    for word in line.split_whitespace() {
        // Length of next word and a space
        let length = word.chars().count() + 1;

        // Check if we need to wrap
        if count + length > line_width {
            wrapped.push(buffer);

            // Start new indented line
            buffer = String::from("    ");
            count = 4;
        }

        // Add next word to line
        buffer.push_str(word);
        buffer.push('n');
        count += length
    }
    wrapped
}

// TODO: format_indented_paragraph, format_list, display_panes, slugify
