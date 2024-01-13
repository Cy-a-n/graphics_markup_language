use std::fmt::Display;

use strum_macros::Display;

use self::Error::Parser;
use self::ParserError::UnexpectedEnd;
use crate::token::Token;
use crate::token::Value;
use colored::Colorize;

#[derive(Display, Debug, PartialEq)]
pub enum Error<'a> {
    Parser(ParserError<'a>),
    LexerUnexpectedChar {
        error_char: char,
        expected_chars: Vec<char>,
        error_line_content: String,
        error_line_number: usize,
        error_offset: usize,
    },
}

impl Error<'_> {
    pub fn raise(self) {
        eprintln!("{}", self.error_message());
    }

    pub fn error_message(&self) -> String {
        format!(
            "{} '{}': {}",
            "ERROR:".red(),
            self.error_type(),
            self.reason()
        )
    }

    fn error_type(&self) -> String {
        match self {
            Parser(error) => format!("{self}.{}", error.error_type()),
            Error::LexerUnexpectedChar {
                error_char: _,
                expected_chars: _,
                error_line_content: _,
                error_line_number: _,
                error_offset: _,
            } => format!("{self}"),
        }
    }

    fn reason(&self) -> String {
        match self {
            Parser(error) => error.reason(),
            Error::LexerUnexpectedChar {
                error_char,
                expected_chars,
                error_line_number,
                error_line_content,
                error_offset,
            } => {
                let error_line_number = line_number_with_padding(*error_line_number);
                format!("The lexer encountered the unexpected character '{error_char}'. Expected chars are {}.\n\n{error_line_number}{error_line_content}\n{error_line_number}{}", format_slice(expected_chars), error_in_source_code("Did not expect this char.", *error_offset, 1))
            }
        }
    }
}

#[derive(Display, Debug, PartialEq)]
pub enum ParserError<'a> {
    UnexpectedEnd {
        parsed_type: ParsedType,
        current_value_slice: &'a [Token],
        expected_tokens: Vec<Value>,
    },
    UnexpectedToken {
        parsed_type: ParsedType,
        current_value_slice: &'a [Token],
        expected_tokens: Vec<Value>,
    },
}

impl ParserError<'_> {
    fn error_type(&self) -> String {
        format!("{self}")
    }

    fn reason(&self) -> String {
        match self {
            UnexpectedEnd {
                parsed_type,
                current_value_slice,
                expected_tokens,
            } => format!(
                "The source code ended while parsing a value of type '{parsed_type}'. Expected tokens are {}.\n\n{}.",
                format_slice(expected_tokens)
                , source_code_with_error(current_value_slice, current_value_slice.last().unwrap_or_else(|| panic!("BUG: Expected the passed current_value_slice vec to have at least one token. '{self}', {self:?}")),"Unexpected end of source code here.")
            ),
            ParserError::UnexpectedToken { parsed_type, current_value_slice, expected_tokens } =>
            {
                let error_token = current_value_slice.last().unwrap_or_else(|| panic!("BUG: Expected the passed current_value_slice vec to have at least one token. '{self}', {self:?}")); 
                format!(
                    "Encountered an unexpected Token '{}' while parsing a value of type '{parsed_type}'. Expected tokens are {}.\n\n{}.",
                    error_token.value(),
                    format_slice(expected_tokens)
                    , source_code_with_error(current_value_slice, error_token, "Unexpected token here.")
                )
            },
        }
    }
}

#[allow(unused)]
#[derive(Display, Debug, PartialEq)]
pub enum ParsedType {
    #[strum(serialize = "u8")]
    U8,
    #[strum(serialize = "i16")]
    I16,
    Color,
    Point,
    Polygon,
    Group,
    Main,
}

fn format_slice<T: Display>(slice: &[T]) -> String {
    let mut output = String::new();
    let mut slice = slice.iter();

    match slice.next() {
        Some(value) => output += &format!("'{value}'"),
        None => return output,
    };

    for value in slice {
        output.push_str(&format!(", '{value}'"));
    }

    output
}

fn error_in_source_code(error_message: &str, offset: usize, error_len: usize) -> String {
    format!(
        "{}{} {error_message}",
        " ".repeat(offset),
        "^".repeat(error_len)
    )
    .red()
    .to_string()
}

fn line_number_with_padding(line_number: usize) -> String {
    let line_number = line_number.to_string();
    let padding = " ".repeat(6_usize.saturating_sub(line_number.len()) + 2);
    format!("{}{padding}", line_number.yellow())
}

fn source_code_with_error(tokens: &[Token], error_token: &Token, error_message: &str) -> String {
    // Error handling
    {
        assert!(
            !tokens.is_empty(),
            "BUG: 'tokens.len()' is zero. 'tokens': {tokens:?}, 'error_token': {error_token:?}",
        );

        assert!(
            tokens.contains(error_token),
            "BUG: 'tokens' does not contain 'error_token'. 'tokens': {tokens:?}, 'error_token': {error_token:?}",
        );
    }

    let line_offset = tokens.first().unwrap().line();
    let tokens_in_lines = {
        let last_line = tokens.last().unwrap().line();
        let mut tokens_in_lines: Vec<Vec<&Token>> = vec![vec![]; last_line - line_offset + 1];

        for token in tokens {
            tokens_in_lines[token.line() - line_offset].push(token);
        }
        tokens_in_lines
    };

    {
        // Push the line number at the start of each string of the output source code
        let mut output: Vec<String> = vec![String::new(); tokens_in_lines.len()];
        for (index, source_code) in output.iter_mut().enumerate() {
            *source_code = line_number_with_padding(index + line_offset);
        }

        // Parse the tokens to their original source code
        let mut source_code = vec![String::new(); tokens_in_lines.len()];
        for (tokens, source_code) in tokens_in_lines.into_iter().zip(source_code.iter_mut()) {
            for token in tokens {
                // Consider white spaces between the tokens
                source_code.push_str(&format!(
                    "{}{}",
                    " ".repeat(token.offset_start_inclusive() - source_code.len()),
                    token.value(),
                ));
            }
        }

        // Create the error message
        let error_message_index = error_token.line() - line_offset;
        let error_message = format!(
            "{}{}",
            output[error_message_index],
            error_in_source_code(
                error_message,
                *error_token.offset_start_inclusive(),
                error_token.value().len(),
            )
        );

        for (output, source_code) in output.iter_mut().zip(source_code.iter()) {
            output.push_str(source_code);
        }
        output.insert(error_message_index + 1, error_message);

        output.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Value::*;

    #[test]
    fn test_source_code_with_error() {
        let tokens = vec![
            Token::new(0, 0, LeftBrace),
            Token::new(1, 4, VisibleExtent),
            Token::new(2, 4, BackgroundColor),
            Token::new(3, 4, Children),
            Token::new(4, 0, RightBrace),
        ];
        let error_token = tokens[0].clone();
        let expected = "\u{1b}[33m0\u{1b}[0m       {\n\u{1b}[33m0\u{1b}[0m       \u{1b}[31m^ There is no error\u{1b}[0m\n\u{1b}[33m1\u{1b}[0m           visible_extent\n\u{1b}[33m2\u{1b}[0m           background_color\n\u{1b}[33m3\u{1b}[0m           children\n\u{1b}[33m4\u{1b}[0m       }";
        let actual = source_code_with_error(&tokens, &error_token, "There is no error");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_source_code_with_error_1() {
        let tokens = vec![
            Token::new(4, 3, LeftBrace),
            Token::new(4, 5, Red),
            Token::new(4, 9, Green),
            Token::new(4, 15, Blue),
            Token::new(4, 20, RightBrace),
        ];
        let error_token = tokens[2].clone();
        let expected = "\u{1b}[33m4\u{1b}[0m          { red green blue }\n\u{1b}[33m4\u{1b}[0m       \u{1b}[31m         ^^^^^ There is no error\u{1b}[0m"        ;
        let actual = source_code_with_error(&tokens, &error_token, "There is no error");

        assert_eq!(actual, expected);
    }
}
