use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    print_usage();
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another",
        "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}