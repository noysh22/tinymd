use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn parse_md_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename = Path::new(_filename);
    let file = File::open(&input_filename)
        .expect("[ ERROR ] Failed to open file!");

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of h1 tags

    // Create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line by line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // For each line, unwrap it
        let content = line.unwrap();
        let mut first_char: Vec<char> = content.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&content[2..]);
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&content);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&_filename[.._filename.len()-3]);
    output_filename.push_str(".html");
    println!("{}", output_filename);

    let mut outfile = File::create(output_filename)
        .expect("[ ERROR ] Could not create output file!");

    // iterate over output
    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (V");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title // return title
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <file>.md",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn usage() {
    print_long_banner();
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_md_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        },
    }
}
