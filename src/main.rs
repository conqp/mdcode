use std::collections::HashMap;
use std::env::args;
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
    args().skip(1).for_each(format)
}

fn format(file: impl AsRef<Path>) {
    let file = file.as_ref();
    match std::fs::read_to_string(file) {
        Ok(text) => {
            let extension = file
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default();

            let language = LANGUAGES.get(extension).unwrap_or(&"");
            //FIXME: escape the text properly, or use indentation
            match file.to_str() {
                Some(filename) => {
                    println!("`{}`:", filename);
                    println!(
                        "```{}\n{}{}```",
                        language,
                        text,
                        if !text.ends_with('\n') { "\n" } else { "" }
                    );
                }
                None => eprintln!("Could not extract file name."),
            }
        }
        Err(code) => eprintln!("Error reading file: {}", code),
    }
}
