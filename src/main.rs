mod regex;

use regex::matches;
use std::env;

/// Accepts a pattern and text as arguments and prints whether the input
/// text matches the pattern.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: simple-regex <pattern> <text>");
        return;
    }

    let pattern = &args[1];
    let text = &args[2];

    let matches = matches(pattern.as_bytes(), text.as_bytes());

    println!(
        "The input text [ {} ] {} the pattern [ {} ]",
        text,
        if matches {
            "does match"
        } else {
            "does not match"
        },
        pattern
    );
}
