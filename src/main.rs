use std::{process::exit, str::FromStr};

use parser::parse;

mod custom_error;
mod error_handling;
mod lexer;
mod parser;
mod token;

fn main() {
    let mut tokens = lexer::to_tokens(
        r#"
{
    position
    rotation = 1111111
    width
    border_color
    fill_color 
    vertices
    children [
        {
            position {
                x = 1
                y
            }
            rotation
            width
            border_color
            fill_color 
            vertices
            children
        }
    ]
}
"#,
    )
    .unwrap_or_else(|error| {
        error.raise();
        exit(1)
    });
    let _to_draw = parse(&mut tokens).unwrap_or_else(|error| {
        error.raise();
        exit(1)
    });

    println!("{:?}", token::Value::from_str("Equals"));
}
