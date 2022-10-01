use std::collections::HashMap;
use std::env::args;
use std::path::Path;

use ezio::FileReadable;
use lazy_static::lazy_static;

lazy_static! {
    static ref LANGUAGES: HashMap<&'static str, &'static str> = HashMap::from([
        ("c", "c"),
        ("cpp", "cpp"),
        ("h", "cpp"),
        ("py", "python"),
        ("rs", "rust"),
        ("toml", "toml"),
    ]);
}

fn main() {
    let args: Vec<String> = args().collect();
    args.iter()
        .enumerate()
        .filter(|&(index, _)| 0 < index)
        .map(|(_, filename)| Path::new(filename))
        .for_each(format)
}

fn format(file: &Path) {
    match String::read(file) {
        Ok(text) => {
            let extension = match file.extension() {
                Some(suffix) => suffix.to_str().unwrap_or(""),
                None => "",
            };

            match file.to_str() {
                Some(filename) => {
                    println!("`{}`:", filename);
                    println!(
                        "```{}\n{}```",
                        LANGUAGES.get(extension).unwrap_or(&""),
                        text
                    );
                }
                None => eprintln!("Could not extract file name."),
            }
        }
        Err(code) => eprintln!("Error reading file: {}", code),
    }
}
