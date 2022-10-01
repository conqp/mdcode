use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;

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

fn read_file(file: &Path) -> Result<String, Error> {
    let fh = File::open(file)?;
    let mut buf_reader = BufReader::new(fh);
    let mut content = String::new();

    match buf_reader.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(code) => Err(code),
    }
}

fn format_file(file: &Path) {
    match read_file(file) {
        Ok(text) => {
            let extension = match file.extension() {
                Some(suffix) => suffix.to_str().unwrap_or(""),
                None => "",
            };

            let language = LANGUAGES.get(extension).unwrap_or(&"");

            match file.to_str() {
                Some(filename) => {
                    println!("`{}`:", filename);
                    println!("```{}\n{}```", language, text);
                }
                None => eprintln!("Could not extract file name."),
            }
        }
        Err(code) => eprintln!("Error reading file: {}", code),
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    args.iter()
        .enumerate()
        .filter(|&(index, _)| 0 < index)
        .map(|(_, filename)| Path::new(filename))
        .for_each(format_file)
}
