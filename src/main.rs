use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

const INDENT: &str = "    ";

fn main() {
    args().skip(1).for_each(|s| format(&s));
}

pub fn format(file: &str) {
    println!("\n`{file}`\n");

    read_lines(file).map_or_else(
        |error| eprintln!("could not read file: {error}"),
        |lines| {
            lines.map_while(Result::ok).for_each(|line| {
                println!("{INDENT}{line}");
            });
        },
    );
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
