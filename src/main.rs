use std::collections::HashMap;
use std::env::args;
use std::ffi::OsStr;
use std::path::Path;

use once_cell::sync::Lazy;

static LANGUAGES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("c", "c"),
        ("cpp", "cpp"),
        ("h", "cpp"),
        ("py", "python"),
        ("rs", "rust"),
        ("toml", "toml"),
    ])
});

fn main() {
    args().skip(1).for_each(format);
}

pub fn format(file: impl AsRef<Path>) {
    match std::fs::read_to_string(file.as_ref()) {
        Ok(text) => print(file.as_ref(), &text),
        Err(code) => eprintln!("Error reading file: {code}"),
    }
}

fn print(file: &Path, text: &String) {
    //FIXME: escape the text properly, or use indentation
    file.to_str().map_or_else(
        || eprintln!("Could not extract file name."),
        |filename| {
            println!("`{filename}`:");
            println!(
                "```{}\n{text}{}```",
                language(file),
                if text.ends_with('\n') { "" } else { "\n" }
            );
        },
    );
}

fn language(file: &Path) -> &'static str {
    LANGUAGES
        .get(file.extension().and_then(OsStr::to_str).unwrap_or_default())
        .unwrap_or(&"")
}
