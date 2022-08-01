/// Matches input text against the provided regular expression pattern.
/// The method will return a match if given an empty pattern.
///
/// The following characters are valid as part of the pattern:
/// * `c` : any character not listed below; matches itself
/// * `.` : wildcard; matches any character
/// * `^` : begin; matches at the beginning of the text
/// * `$` : end; matches at the end of the text
/// * `*` : repeat: matches previous character zero or more times
///
/// # Arguments
///
/// * `pattern` - The pattern to match text against
/// * `text` - The input text to test
///
/// # Examples
///
/// ```
/// let pattern = b"abc";
/// let text = b"123abc456";
/// let found = matches(pattern, text);
/// assert!(found);
/// ```
pub fn matches(pattern: &[u8], text: &[u8]) -> bool {
    if !pattern.is_empty() && pattern[0] == b'^' {
        return match_position(&pattern[1..], text);
    }
    for i in 0..text.len() + 1 {
        if match_position(pattern, &text[i..]) {
            return true;
        }
    }
    false
}

fn match_position(pattern: &[u8], text: &[u8]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if pattern[0] == b'$' && pattern.get(1).is_none() {
        return text.is_empty();
    }
    if let Some(b'*') = pattern.get(1) {
        return match_repeat(pattern[0], &pattern[2..], text);
    }
    if !text.is_empty() && (pattern[0] == b'.' || pattern[0] == text[0]) {
        return match_position(&pattern[1..], &text[1..]);
    }
    false
}

fn match_repeat(repeat: u8, pattern: &[u8], text: &[u8]) -> bool {
    for i in 0..text.len() + 1 {
        if match_position(pattern, &text[i..]) {
            return true;
        }
        if i == text.len() || (repeat != text[i] && repeat != b'.') {
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn match_identifies_empty_pattern_as_match() {
        let pattern = b"";
        let text = b"012345678910abcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_detects_pattern_in_text() {
        let pattern = b"abc";
        let text = b"012345678910abcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_detects_pattern_not_in_text() {
        let pattern = b"abc";
        let text = b"012345678910bcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(!found);
    }

    #[test]
    fn match_correctly_matches_wildcard_in_text() {
        let pattern = b".bc";
        let text = b"012345678910abcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_at_beginning_of_text() {
        let pattern = b"^abc";
        let text = b"abcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_at_beginning_of_text_only() {
        let pattern = b"^";
        let text = b"abcdefghijklmnopqrstuvwxyz";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_at_end_of_text() {
        let pattern = b"abc$";
        let text = b"012345678910abc";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_at_end_of_text_only() {
        let pattern = b"$";
        let text = b"012345678910abc";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_repeater() {
        let pattern = b"ab*c";
        let text = b"abbbc";
        let found = matches(pattern, text);
        assert!(found);
    }

    #[test]
    fn match_correctly_matches_repeater_at_end() {
        let pattern = b"abc*";
        let text = b"abbb";
        let found = matches(pattern, text);
        assert!(found);
    }
}
