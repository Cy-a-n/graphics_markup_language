use strum_macros::Display;

use self::Error::Parser;
use self::ParserError::TokensEndPrematurely;
use crate::token::Token;
use crate::token::Value;
use colored::Colorize;

#[derive(Display)]
pub enum Error {
    #[allow(unused)]
    Parser(ParserError),
}

impl Error {
    #[allow(unused)]
    pub fn raise(self) {
        eprintln!(
            "{} `{}`: {}",
            "ERROR:".red(),
            self.error_type(),
            self.reason()
        );
    }

    fn error_type(&self) -> String {
        match self {
            Parser(error) => format!("{self}.{}", error.error_type()),
        }
    }

    fn reason(&self) -> String {
        match self {
            Parser(error) => error.reason(),
        }
    }
}

#[derive(Display, Debug)]
pub enum ParserError {
    #[allow(unused)]
    TokensEndPrematurely {
        parsed_type: ParsedType,
        current_value_slice: Vec<Token>,
        expected_tokens: Vec<Value>,
    },
}

impl ParserError {
    fn error_type(&self) -> String {
        match self {
            TokensEndPrematurely {
                parsed_type: _,
                current_value_slice: _,
                expected_tokens: _,
            } => format!("{self}"),
        }
    }

    fn reason(&self) -> String {
        match self {
            TokensEndPrematurely {
                parsed_type,
                current_value_slice,
                expected_tokens,
            } => format!(
                "The source code ended while parsing a value of type `{parsed_type}`. Expected the tokens {}\n\n{}.",
                expected_tokens_to_string(expected_tokens)
                , source_code_with_error(current_value_slice, current_value_slice.last().unwrap_or_else(|| panic!("BUG: Expected the passed current_value_slice vec to have at least one token. `{self}`, {self:?}")),"Source code ends here.")
            ),
        }
    }
}

#[derive(Display, Debug)]
pub enum ParsedType {
    #[allow(unused, non_camel_case_types)]
    i16,
    #[allow(unused, non_camel_case_types)]
    u8,
    #[allow(unused)]
    Color,
    #[allow(unused)]
    Point,
    #[allow(unused)]
    Polygon,
    #[allow(unused)]
    Group,
    #[allow(unused)]
    Main,
}

fn expected_tokens_to_string(expected_tokens: &[Value]) -> String {
    let mut output = String::new();
    let mut expected_tokens = expected_tokens.iter();

    match expected_tokens.next() {
        Some(token) => output += &format!("`{}`", token.to_str()),
        None => return output,
    };

    for token in expected_tokens {
        output.push_str(&format!(", `{token}`"));
    }

    output
}

fn source_code_with_error(tokens: &Vec<Token>, error_token: &Token, error_message: &str) -> String {
    // Error handling
    {
        assert!(
            !tokens.is_empty(),
            "BUG: `tokens.len()` is zero. `tokens`: {tokens:?}, `error_token`: {error_token:?}",
        );

        assert!(
            tokens.contains(error_token),
            "BUG: `tokens` does not contain `error_token`. `tokens`: {tokens:?}, `error_token`: {error_token:?}",
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
            let line_number = (index + line_offset).to_string();
            let padding = " ".repeat(6_usize.saturating_sub(line_number.len()) + 2);
            *source_code = format!("{}{padding}", line_number.yellow());
        }

        // Parse the tokens to their original source code
        let mut source_code = vec![String::new(); tokens_in_lines.len()];
        for (tokens, source_code) in tokens_in_lines.into_iter().zip(source_code.iter_mut()) {
            for token in tokens {
                // Consider white spaces between the tokens
                let token_start = token.offset_start_inclusive();
                let source_code_len = source_code.len();
                source_code.push_str(&format!(
                    "{}{}",
                    " ".repeat(token_start - source_code_len),
                    token.value(),
                ));
            }
        }

        // Create the error message
        let error_message_index = error_token.line() - line_offset;
        let error_message = format!(
            "{}{}{} {error_message}",
            output[error_message_index],
            " ".repeat(*error_token.offset_start_inclusive()),
            "^".repeat(error_token.value().len())
        )
        .red()
        .to_string();

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
            Token::new(0, 0, StructStart),
            Token::new(1, 4, AttributVisibleExtent),
            Token::new(2, 4, AttributBackgroundColor),
            Token::new(3, 4, AttributShapes),
            Token::new(4, 0, StructEnd),
        ];
        let error_token = tokens[0].clone();
        let expected = "\u{1b}[33m0\u{1b}[0m       {\n\u{1b}[31m\u{1b}[33m0\u{1b}[0m\u{1b}[31m       ^ There is no error\u{1b}[0m\n\u{1b}[33m1\u{1b}[0m           visible_extent\n\u{1b}[33m2\u{1b}[0m           background_color\n\u{1b}[33m3\u{1b}[0m           shapes\n\u{1b}[33m4\u{1b}[0m       }";
        let actual = source_code_with_error(&tokens, &error_token, "There is no error");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_source_code_with_error_1() {
        let tokens = vec![
            Token::new(4, 3, StructStart),
            Token::new(4, 5, AttributRed),
            Token::new(4, 9, AttributGreen),
            Token::new(4, 15, AttributBlue),
            Token::new(4, 20, StructEnd),
        ];
        let error_token = tokens[2].clone();
        let expected = "\u{1b}[33m4\u{1b}[0m          { red green blue }\n\u{1b}[31m\u{1b}[33m4\u{1b}[0m\u{1b}[31m                ^^^^^ There is no error\u{1b}[0m";
        let actual = source_code_with_error(&tokens, &error_token, "There is no error");

        assert_eq!(actual, expected);
    }
}
