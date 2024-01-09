mod custom_error;
mod draw_elements;
mod error_handling;
mod lexer;
mod parser;
mod token;

fn main() {
    let tokens = lexer::to_tokens("=+->");
    if let Err(error) = tokens {
        error.raise();
    }
}
