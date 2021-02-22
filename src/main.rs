// Lifecycle outline
//
// Given a call to tinymd...
//   When I pass a markdown file as an argument, it should:
//     1. Open the file
//     2. Parse the file line by line into a buffer
//     3. Export the buffer to a new HTML file.
//   When I pass anything else OR no argument at all, it should:
//     1. Show the banner.

use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_title() -> String {
    let title = String::from(env!("CARGO_PKG_NAME"));
    let version = String::from(env!("CARGO_PKG_VERSION"));
    let description = String::from(env!("CARGO_PKG_DESCRIPTION"));
    let output = format!("{} (v{}), {}", title, version, description);
    return output;
}

// Called when we are passed a markdown file.
fn parse_md_file(_filename: &str) {
    print_short_banner();
    println!("Trying to parse {}...", _filename);

    // Create a path variable from the filename.
    let input_filename = Path::new(_filename);
    // Attempt to open the file.
    let file = File::open(&input_filename).expect("[ Error ] Failed to open file!");

    // The _ avoids compiler complaints about unused variables. We're setting defaults here before looping.
    let mut _ptag: bool = false; // Keep track of paragraph tags.
    let mut _h1tag: bool = false; // Keep track of h1 tags.

    // Create an empty vector object to store tokens (pre-parsed stuff).
    let mut tokens: Vec<String> = Vec::new();

    // Read the file line by line
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // This could be made more concise by using the unwrap() method.
        // let line_contents = line.unwrap();
        let line_contents = match line {
            Ok(contents) => contents,
            Err(e) => panic!("Garbage: {}", e),
        };

        // Popping this first character from the Vector actually returns an Option object
        // Some() and None() instead of Ok() and Err().
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                // Close any open ptags before starting an h1 if _ptag is true.
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }
                // Close any open h1tags before starting an h1 if _ptag is true.
                if _h1tag {
                    _h1tag = false;
                    output_line.push_str("</h1>\n");
                }

                _h1tag = true;
                output_line.push_str("\n<h1>");
                output_line.push_str(&line_contents[2..]); // Get all but the first two characters.
            } // The first character is #
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("\n<p>");
                }

                output_line.push_str(&line_contents);
            } // The first character is not #
        }
        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }
        if _h1tag {
            _h1tag = false;
            output_line.push_str("</h1>\n");
        }

        // Avoid pushing blank lines. @TODO: This doesn't work.
        if output_line != "\n<p></p>" {
            tokens.push(output_line);
        }
    }

    // Handle output
    // -------------
    let mut output_filename = String::from(&_filename[.._filename.len() - 3]); // Same as input minus the ".md" bit.
    output_filename.push_str(".html"); // Append .html

    let mut outfile = File::create(output_filename)
        .expect("[ Error ] Could not create output file for whatever reason.");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ Error ] Could not write to output file for whatever reason.");
    }

    println!("[ Info ] Parsing complete!");
}

// Print title version and description.
fn print_short_banner() {
    println!("{}", get_title());
}

// Print the short banner as well as attribution and usage example.
fn print_long_banner() {
    print_short_banner();
    let author = String::from(env!("CARGO_PKG_AUTHORS"));
    let homepg = String::from(env!("CARGO_PKG_HOMEPAGE"));

    let output = format!(
        "Written by: {}\nHomepage: {}\nUsage: tinymd <somefile.md>",
        author, homepg
    );

    println!("{}", output);
}

fn usage() {
    print_long_banner();
}

fn main() {
    // usage();
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_md_file(&args[1]),
        _ => {
            println!("[ Error ] Invalid invocation (ya dun goofed!)");
            usage();
        }
    }
}
