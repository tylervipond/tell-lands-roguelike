pub fn get_longest_line<'a, T: ToString>(lines: &'a Box<[T]>) -> Option<&'a T> {
    lines.iter().max_by(|x, y| {
        x.to_string()
            .chars()
            .count()
            .cmp(&y.to_string().chars().count())
    })
}

pub fn get_longest_line_length<T: ToString>(lines: &Box<[T]>) -> usize {
    match get_longest_line(lines) {
        Some(line) => line.to_string().chars().count(),
        None => 0,
    }
}

pub fn split_to_lines(text: &str, line_width: u32) -> Vec<String> {
    let words = text.split_whitespace();
    words.fold(Vec::<String>::new(), |mut acc, word| {
        if let Some(last) = acc.last_mut() {
            if last.len() + word.len() + 1 < line_width as usize {
                last.push_str(" ");
                last.push_str(word);
            } else {
                acc.push(String::from(word));
            }
        } else {
            acc.push(String::from(word));
        }
        acc
    })
}

pub fn get_offset_from_center(context_size: usize, content_size: usize) -> usize {
    return context_size / 2 - content_size / 2;
}
