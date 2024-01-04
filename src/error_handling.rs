use std::cmp::min;
use std::usize;

use strum_macros::Display;

use self::Error::*;
use self::ParserError::*;
use crate::token::Token;
use crate::token::TokenValue;

#[derive(Display)]
pub enum Error {
    #[allow(unused)]
    Parser(ParserError),
}

impl Error {
    #[allow(unused)]
    pub fn raise_error(self) -> ! {
        panic!("Error of type: `{}`: {}", self.error_type(), self.reason())
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
        expected_tokens: Vec<TokenValue>,
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
                "The source code ended while parsing a value of type `{parsed_type}`. Expected {}\n\n{}",
                expected_tokens_to_string(expected_tokens)
                , source_code_with_error(current_value_slice, current_value_slice.last().expect(&format!("BUG: Expected the passed current_value_slice vec to have at least one token. `{self}`, {:?}", self)), &format!("Source code ends here. Expect {}", expected_tokens_to_string(expected_tokens)))
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

fn expected_tokens_to_string(expected_tokens: &Vec<TokenValue>) -> String {
    let mut expected_tokens = expected_tokens.into_iter();
    let mut output = "".to_string();

    match expected_tokens.next() {
        Some(token) => output += &format!("`{}`", token.to_str()),
        None => return output,
    };

    for token in expected_tokens {
        output += &format!("{output}, `{}`", token.to_str());
    }
    output
}

fn source_code_with_error(tokens: &Vec<Token>, error_token: &Token, error_message: &str) -> String {
    // Error handling
    {
        if tokens.is_empty() {
            panic!(
                "BUG: `tokens.len()` is zero. `tokens`: {:?}, `error_token`: {:?}",
                tokens, error_token
            )
        }

        if !tokens.contains(&error_token) {
            panic!(
                "BUG: `tokens` does not contain `error_token`. `tokens`: {:?}, `error_token`: {:?}",
                tokens, error_token
            )
        }
    }

    let line_offset = tokens.first().unwrap().line();
    let tokens_in_lines = {
        let last_line = tokens.last().unwrap().line();
        let mut tokens_in_lines: Vec<Vec<&Token>> = Vec::with_capacity(last_line - line_offset + 2);

        for token in tokens {
            tokens_in_lines[token.line() - line_offset].push(token);
        }
        tokens_in_lines
    };

    let char_offset = {
        let mut min_char = usize::MAX;

        for line in tokens_in_lines.iter() {
            for token in line {
                min_char = min(min_char, *token.offset_start_inclusive());
            }
        }

        min_char
    };

    {
        let mut source_code: Vec<String> = Vec::with_capacity(tokens_in_lines.len());

        let mut error_token_index = 0;
        let mut tokens_in_lines = tokens_in_lines
            .iter()
            .zip(source_code.iter_mut())
            .enumerate();

        // Parse the tokens to source code until the line where the error token is found
        for (line_index, (tokens, line)) in &mut tokens_in_lines {
            line.push_str(&format!("{}", line_index + line_offset));
            line.push_str(&" ".repeat(14_usize.checked_sub(line.len()).unwrap_or(0) + 2));

            let mut string_index = 0;
            let mut error_token_found: Option<usize> = None;
            for token in tokens {
                let padding_amount = token.offset_start_inclusive() - char_offset - string_index;
                line.push_str(&" ".repeat(padding_amount));

                line.push_str(token.value().to_str());
                string_index = padding_amount + token.value().length(); // Potentially I have to add one here due to exclusive end.

                if *token == error_token {
                    error_token_found = Some(line.len());
                }
            }

            if let Some(index) = error_token_found {
                error_token_index = index;
                break;
            }
        }

        // Add the error message to the source code

        let (_, (tokens, line)) = tokens_in_lines
            .next()
            .expect("BUG: Error token found. The `tokens_in_lines` iterator shouldn't end here.");
        if !tokens.is_empty() {
            panic!("BUG: Tokens should be empty because this is the line where the error message should be inserted. `Tokens`: `{:?}`", tokens);
        }

        line.push_str(&" ".repeat(error_token_index - error_token.value().length()));
        line.push_str(&"^".repeat(error_token.value().length()));
        line.push(' ');
        line.push_str(error_message);

        // Parse the rest of the tokens to source code. This is a simplified version of the for loop above
        for (line_index, (tokens, line)) in tokens_in_lines {
            line.push_str(&format!("{}", line_index + line_offset - 1));
            line.push_str(&" ".repeat(14_usize.checked_sub(line.len()).unwrap_or(0) + 2));

            let mut string_index = 0;
            for token in tokens {
                let padding_amount = token.offset_start_inclusive() - char_offset - string_index;
                line.push_str(&" ".repeat(padding_amount));

                line.push_str(token.value().to_str());
                string_index = padding_amount + token.value().length(); // Potentially I have to add one here due to exclusive end.
            }
        }

        source_code.join("\n")
    }
}
