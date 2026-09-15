use serde::{Deserialize, Serialize};

// Represents a styled segment of text within a changelog entry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TextSegment {
    Plain(String),
    Bold(String),
    Italic(String),
}

// Data structure for a single version's release notes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: Option<String>,
    pub changes: Vec<Vec<TextSegment>>,
}

// Parses a single line of text into styled segments (bold, italic, code).
pub fn parse_line_formatting(text: &str) -> Vec<TextSegment> {
    // ...
    let mut segments = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for Bold: **bold**
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(end) = find_closing(&chars, i + 2, "**") {
                if !current.is_empty() {
                    segments.push(TextSegment::Plain(current.clone()));
                    current.clear();
                }
                segments.push(TextSegment::Bold(chars[i + 2..end].iter().collect()));
                i = end + 2;
                continue;
            }
        }

        // Check for Italic: *italic*
        if chars[i] == '*' {
            if let Some(end) = find_closing(&chars, i + 1, "*") {
                if !current.is_empty() {
                    segments.push(TextSegment::Plain(current.clone()));
                    current.clear();
                }
                segments.push(TextSegment::Italic(chars[i + 1..end].iter().collect()));
                i = end + 1;
                continue;
            }
        }

        // Check for Italic (backtick): `italic`
        if chars[i] == '`' {
            if let Some(end) = find_closing(&chars, i + 1, "`") {
                if !current.is_empty() {
                    segments.push(TextSegment::Plain(current.clone()));
                    current.clear();
                }
                segments.push(TextSegment::Italic(chars[i + 1..end].iter().collect()));
                i = end + 1;
                continue;
            }
        }

        current.push(chars[i]);
        i += 1;
    }

    if !current.is_empty() {
        segments.push(TextSegment::Plain(current));
    }

    segments
}

fn find_closing(chars: &[char], start: usize, token: &str) -> Option<usize> {
    let token_chars: Vec<char> = token.chars().collect();
    let token_len = token_chars.len();

    if start + token_len > chars.len() {
        return None;
    }

    for i in start..=(chars.len() - token_len) {
        if &chars[i..i + token_len] == &token_chars[..] {
            return Some(i);
        }
    }
    None
}

pub fn parse_changelog(content: &str) -> Vec<ChangelogEntry> {
    let mut entries = Vec::new();
    let mut current_entry: Option<ChangelogEntry> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("# ") {
            if let Some(entry) = current_entry.take() {
                entries.push(entry);
            }

            // Parse version and date
            // Format: # [v1.0](link) (2026-09-03) or # v1.0 (2026-09-03)
            let mut version = line[2..].to_string();
            let mut date = None;

            if let Some(start) = version.find('[') {
                if let Some(end) = version.find(']') {
                    version = version[start + 1..end].to_string();
                }
            } else if version.contains(' ') {
                // Try to split by space if no brackets
                let temp = version.clone();
                let parts: Vec<&str> = temp.split_whitespace().collect();
                if !parts.is_empty() {
                    version = parts[0].to_string();
                }
            }

            if let Some(end) = line.rfind(')') {
                if let Some(start) = line.rfind('(') {
                    let potential_date = &line[start + 1..end];
                    // Very basic check to avoid catching the link URL as a date
                    if potential_date.contains('-')
                        || potential_date.contains('/')
                        || potential_date.chars().any(|c| c.is_numeric())
                    {
                        date = Some(potential_date.to_string());
                    }
                }
            }

            current_entry = Some(ChangelogEntry {
                version,
                date,
                changes: Vec::new(),
            });
        } else if line.starts_with("- ") || line.starts_with("* ") || line.starts_with("• ") {
            if let Some(entry) = &mut current_entry {
                let change_text = line
                    .trim_start_matches(|c: char| c == '-' || c == '*' || c == '•')
                    .trim()
                    .to_string();
                if !change_text.is_empty() {
                    entry.changes.push(parse_line_formatting(&change_text));
                }
            }
        }
    }

    if let Some(entry) = current_entry {
        entries.push(entry);
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_formatting() {
        let segments = parse_line_formatting("Plain **Bold** *Italic* `Code` Mix");
        assert_eq!(segments.len(), 7);

        match &segments[0] {
            TextSegment::Plain(s) => assert_eq!(s, "Plain "),
            _ => panic!(),
        }
        match &segments[1] {
            TextSegment::Bold(s) => assert_eq!(s, "Bold"),
            _ => panic!(),
        }
        match &segments[2] {
            TextSegment::Plain(s) => assert_eq!(s, " "),
            _ => panic!(),
        }
        match &segments[3] {
            TextSegment::Italic(s) => assert_eq!(s, "Italic"),
            _ => panic!(),
        }
        match &segments[4] {
            TextSegment::Plain(s) => assert_eq!(s, " "),
            _ => panic!(),
        }
        match &segments[5] {
            TextSegment::Italic(s) => assert_eq!(s, "Code"),
            _ => panic!(),
        }
        match &segments[6] {
            TextSegment::Plain(s) => assert_eq!(s, " Mix"),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_changelog() {
        let content = "
# [v1.4.0](https://link) (2026-09-01)
- Added feature 1
- **Improved** performance

# v1.3.2 (2026-08-15)
• Fixed bug
";
        let entries = parse_changelog(content);
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].version, "v1.4.0");
        assert_eq!(entries[0].date.as_deref(), Some("2026-09-01"));
        assert_eq!(entries[0].changes.len(), 2);

        assert_eq!(entries[1].version, "v1.3.2");
        assert_eq!(entries[1].date.as_deref(), Some("2026-08-15"));
        assert_eq!(entries[1].changes.len(), 1);
    }
}
