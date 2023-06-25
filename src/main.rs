use std::env::args;
use std::fs::read_to_string;

const INDENT: &str = "    ";

fn main() {
    args().skip(1).for_each(|s| format(&s));
}

pub fn format(file: &str) {
    read_to_string(file).map_or_else(
        |_| {
            eprintln!("Error reading file: {file}");
        },
        |text| {
            println!("`{file}`\n\n{}", indent_lines(&text));
        },
    );
}

fn indent_lines(text: &str) -> String {
    let mut out = String::new();

    for line in text.lines() {
        out.push_str(INDENT);
        out.push_str(line);
        out.push('\n');
    }

    out
}
