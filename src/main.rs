use std::{env, fs::File, io::prelude::*, process::exit};

mod custom_error;
mod error_handling;
mod lexer;
mod parser;
mod svg_transpiler;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No input file provided");
        exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() > 2 {
        args[2].clone()
    } else {
        input_path.replace(".gcml", ".svg")
    };

    let mut input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening input file: {}", error);
            exit(1);
        }
    };

    let mut source_code = String::new();
    if input_file.read_to_string(&mut source_code).is_err() {
        eprintln!("Error reading input file");
        exit(1);
    }

    // The main part of the program takes place here.
    // We first tokenize the input.
    let mut tokens = match lexer::to_tokens(&source_code) {
        Ok(t) => t,
        Err(error) => {
            error.raise();
            exit(1);
        }
    };

    // Then we parse the output to simple polygons.
    let to_draw = match parser::parse(&mut tokens) {
        Ok(td) => td,
        Err(error) => {
            error.raise();
            exit(1);
        }
    };

    // Lastly we transpile the polygons to svg.
    let svg = svg_transpiler::to_svg(to_draw);

    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error creating output file: {}", error);
            exit(1);
        }
    };

    if output_file.write_all(svg.as_bytes()).is_err() {
        eprintln!("Error writing to output file");
        exit(1);
    }
}
