use std::env::args;
use std::fs::File;
use std::io;
use std::io::BufRead;
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
