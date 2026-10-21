//! Text extraction from an HTML page, for the reference specifications
//! that are only published that way (Go, Zig).
//!
//! The goal is a count comparable with a Markdown or plain-text
//! specification: markup is not prose and would inflate the figure. So
//! the extraction drops `<script>`, `<style>` and comments with their
//! content, drops every other tag, decodes the entities that occur in
//! such pages, and folds runs of blank lines. Nothing else: no
//! wrapping, no re-indentation, no attempt at Markdown.

pub fn strip(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let skip_to = if starts_with_ci(rest, "<script") {
            find_ci(rest, "</script>").map(|i| i + "</script>".len())
        } else if starts_with_ci(rest, "<style") {
            find_ci(rest, "</style>").map(|i| i + "</style>".len())
        } else if rest.starts_with("<!--") {
            rest.find("-->").map(|i| i + 3)
        } else {
            rest.find('>').map(|i| i + 1)
        };
        match skip_to {
            Some(i) => rest = &rest[i..],
            None => {
                out.push_str(rest);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    fold_blank_lines(&decode_entities(&out))
}

fn starts_with_ci(text: &str, prefix: &str) -> bool {
    text.len() >= prefix.len()
        && text.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

fn find_ci(haystack: &str, needle: &str) -> Option<usize> {
    haystack.to_ascii_lowercase().find(needle)
}

fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let Some(end) = rest
            .char_indices()
            .take(12)
            .find(|&(_, c)| c == ';')
            .map(|(i, _)| i)
        else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let name = &rest[1..end];
        let decoded = match name {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some('\u{a0}'),
            "laquo" => Some('«'),
            "raquo" => Some('»'),
            "ldquo" => Some('“'),
            "rdquo" => Some('”'),
            "lsquo" => Some('‘'),
            "rsquo" => Some('’'),
            "hellip" => Some('…'),
            "mdash" => Some('—'),
            "ndash" => Some('–'),
            "times" => Some('×'),
            "larr" => Some('←'),
            "rarr" => Some('→'),
            _ => numeric_entity(name),
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn numeric_entity(name: &str) -> Option<char> {
    let digits = name.strip_prefix('#')?;
    let code = match digits.strip_prefix(['x', 'X']) {
        Some(hex) => u32::from_str_radix(hex, 16).ok()?,
        None => digits.parse().ok()?,
    };
    char::from_u32(code)
}

fn fold_blank_lines(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut blank_run = 0;
    for line in text.lines() {
        let trimmed = line.trim_end();
        if trimmed.trim().is_empty() {
            blank_run += 1;
            if blank_run > 1 {
                continue;
            }
            out.push('\n');
        } else {
            blank_run = 0;
            out.push_str(trimmed);
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_tags_and_keeps_text() {
        assert_eq!(strip("<p>a <b>b</b> c</p>\n"), "a b c\n");
    }

    #[test]
    fn drops_scripts_styles_and_comments_with_their_content() {
        let page = "<style>p{}</style>x<SCRIPT>if (1 < 2) {}</script>y<!-- <p>z</p> -->\n";
        assert_eq!(strip(page), "xy\n");
    }

    #[test]
    fn survives_multibyte_text_near_an_ampersand() {
        assert_eq!(strip("&💯;x&\n"), "&💯;x&\n");
    }

    #[test]
    fn decodes_entities() {
        assert_eq!(
            strip("a &lt; b &amp;&amp; c &#x3e; d &#100; &unknown; &\n"),
            "a < b && c > d d &unknown; &\n"
        );
    }

    #[test]
    fn folds_blank_lines_and_trailing_spaces() {
        assert_eq!(strip("a  \n\n\n\n  \nb\n"), "a\n\nb\n");
    }
}
