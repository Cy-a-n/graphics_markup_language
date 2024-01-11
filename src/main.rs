use std::process::exit;

use parser::parse;

mod custom_error;
mod error_handling;
mod lexer;
mod parser;
mod token;

fn main() {
    let mut tokens = lexer::to_tokens(
        r#"{
            position
             rotation
              width
               border_color
                fill_color 
            vertices"#,
    )
    .unwrap_or_else(|error| {
        error.raise();
        exit(1)
    });
    let _to_draw = parse(&mut tokens).unwrap_or_else(|error| {
        error.raise();
        exit(1)
    });
}
