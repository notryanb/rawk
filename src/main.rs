extern crate clap;

use clap::{Arg, App, SubCommand};
mod parse;

/*
    Need to implement args
        -f parses AWK program from other file
        -F lets you pass the file separator

    pattern { action }
*/

fn main() {
    let matches = App::new("rawk")
                          .version("0.1")
                          .author("Ryan Blecher <notryanb@gmail.com>")
                          .about("AWK interpreter")
                          .get_matches();

    let other_file = parse::some_function();
    println!("{}", other_file);
}

