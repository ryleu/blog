use std::{path::Path, io::Write, fs};
use markdown;

const TEMPLATE: &str = include_str!("template.html");

fn main() {
    match compile_file("test") {
        Ok(compiled) => write_file("test", &compiled),
        Err(_) => {},
    };
}

/**
 * Compiles the file to HTML
 */
fn compile_file(s: &str) -> Result<String, std::io::Error> {
    match markdown::file_to_html(Path::new(&format!("input/{}.md", s))) {
        Ok(text) => Ok(TEMPLATE.replace("{body}", &text)),
        Err(err) => Err(err),
    }
}

/**
 * Writes the file to disk
 */
fn write_file(filename: &str, contents: &str) {
    match fs::File::create(format!("output/{}.html", filename)) {
        Ok(mut f) => {
            match f.write_all(contents.as_bytes()) {
                Ok(_) => println!("Successfully wrote {}", filename),
                Err(err) => println!("Error when writing {}: {}", filename, err),
            };
        },
        Err(err) => println!("Error when writing {}: {}", filename, err),
    };
}
