mod custom_error;
mod draw_elements;
mod error_handling;
mod finite_automata;
mod lexer;
mod token;

fn main() {
    let _tokens = lexer::to_tokens("=+->");
}
