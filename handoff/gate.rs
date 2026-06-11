use std::io::{self, Read};
use std::process::exit;

const SECTIONS: [&str; 10] = [
    "task_context",
    "tone_context",
    "background",
    "task_description",
    "examples",
    "conversation_history",
    "immediate_task",
    "thinking",
    "output_format",
    "prefill",
];

fn rank(name: &str) -> Option<usize> {
    SECTIONS.iter().position(|&s| s == name)
}

fn parse_tag(s: &str) -> Option<(bool, &str)> {
    let b = s.as_bytes();
    if b.len() < 3 || b[0] != b'<' || b[b.len() - 1] != b'>' {
        return None;
    }
    let inner = &s[1..s.len() - 1];
    let (closing, name) = match inner.strip_prefix('/') {
        Some(rest) => (true, rest),
        None => (false, inner),
    };
    if name.is_empty() || !name.bytes().all(|c| c.is_ascii_lowercase() || c == b'_') {
        return None;
    }
    Some((closing, name))
}

fn fail(reason: String, line: usize) -> ! {
    println!("WRONG: {reason} (line {line})");
    exit(1);
}

fn main() {
    let mut doc = String::new();
    io::stdin().read_to_string(&mut doc).unwrap();

    let mut open: Option<(&str, usize)> = None;
    let mut seen: Vec<&str> = Vec::new();
    let mut high: i32 = -1;
    let mut in_fence = false;

    for (i, raw) in doc.lines().enumerate() {
        let n = i + 1;

        if raw.trim_start().starts_with("```") || raw.trim_start().starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence || raw.starts_with(char::is_whitespace) {
            continue;
        }

        let (closing, name) = match parse_tag(raw.trim_end()) {
            Some(t) => t,
            None => continue,
        };
        let r = match rank(name) {
            Some(r) => r,
            None => continue,
        };

        if closing {
            match open {
                Some((cur, _)) if cur == name => open = None,
                _ => fail(format!("</{name}> closes nothing open"), n),
            }
            continue;
        }

        if let Some((cur, _)) = open {
            fail(format!("<{name}> opened before <{cur}> closed"), n);
        }
        if seen.contains(&name) {
            fail(format!("duplicate <{name}>"), n);
        }
        if (r as i32) < high {
            fail(format!("<{name}> out of order"), n);
        }
        open = Some((name, n));
        seen.push(name);
        high = r as i32;
    }

    if let Some((cur, line)) = open {
        fail(format!("<{cur}> never closed"), line);
    }
    println!("OK");
}
