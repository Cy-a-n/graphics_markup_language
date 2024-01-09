use self::States::*;
use super::macros::{sub_parser_branch, sub_parser_return_branch};
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType::U8;
use crate::error_handling::ParserError::TokensUnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{EqualsChar,One, Zero};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_u8<'a>(tokens_iter: &'a mut Peekable<Enumerate<Iter<'a, Token>>>, tokens: &'a [Token]) -> Result<u8, Error<'a>> {
    let slice_from_value_start = &tokens[tokens_iter.peek().expect("BUG: 'tokens_iter' should have at least one token.").0..];
    let mut state = Start;

    loop {
        state = state.next_state(tokens_iter);
        match state {
            End(value) => return Ok(value),
            TokensUnexpectedEnd(expected_tokens) => return Err(Parser(TokensUnexpectedEnd { parsed_type: U8, current_value_slice: slice_from_value_start, expected_tokens })),
            UnexpectedToken(expected_tokens) => return Err(Parser(UnexpectedToken { parsed_type: U8, current_value_slice: &slice_from_value_start[..tokens_iter.next().expect("BUG: 'tokens_iter' should have at least one token.").0 + 1], expected_tokens })),
            _ => {},
        }
    }
}

#[derive(Debug)]
enum States {
    Start,
    EqualsSign,
    Digit0(u8),
    Digit1(u8),
    Digit2(u8),
    Digit3(u8),
    Digit4(u8),
    Digit5(u8),
    Digit6(u8),
    End(u8),
    TokensUnexpectedEnd(Vec<Value>),
    UnexpectedToken(Vec<Value>),
}

impl States {
    fn next_state(self, tokens: &mut Peekable<Enumerate<Iter<Token>>>) -> Self {
        match self {
            Start => sub_parser_branch!(tokens,
                EqualsChar => EqualsSign
            ),
            EqualsSign => sub_parser_branch!(tokens,
                Zero => Digit0(0),
                One => Digit0(1),
            ),
            Digit0(value) => sub_parser_return_branch!(tokens, value, 
                Zero => Digit1(value << 1),
                One => Digit1((value << 1) + 1),
            ),
            Digit1(value) => sub_parser_return_branch!(tokens, value,
                Zero => Digit2(value << 1),
                One => Digit2((value << 1) + 1),
            ),
            Digit2(value) => sub_parser_return_branch!(tokens, value,
                Zero => Digit3(value << 1),
                One => Digit3((value << 1) + 1),
            ),
            Digit3(value) => sub_parser_return_branch!(tokens, value,
                Zero => Digit4(value << 1),
                One => Digit4((value << 1) + 1),
            ),
            Digit4(value) => sub_parser_return_branch!(tokens, value,
                Zero => Digit5(value << 1),
                One => Digit5((value << 1) + 1),
            ),
            Digit5(value) => sub_parser_return_branch!(tokens, value,
                Zero => Digit6(value << 1),
                One => Digit6((value << 1) + 1),
            ),
            Digit6(value) => sub_parser_return_branch!(tokens, value,
                Zero => End(value << 1),
                One => End((value << 1) + 1),
            ),
            End(_) => panic!("BUG: The `next_state` method should never be called on the `End` state. 'state': '{self:?}'."),
            TokensUnexpectedEnd(_) => panic!("BUG: The `next_state` method should never be called on the `TokensUnexpectedEnd` state. 'state': '{self:?}'."),
            UnexpectedToken(_) => panic!("BUG: The `next_state` method should never be called on the `UnexpectedToken` state. 'state': '{self:?}'."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{token::Value::{One, StructEnd, Zero}, error_handling::ParserError::{UnexpectedToken, TokensUnexpectedEnd}};

    #[test]
    fn test_parse_u8_0_full() {
        let tokens = vec![
            Token::default(EqualsChar),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = 0;
        let actual = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens).expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_u8_max() {
        let tokens = vec![
            Token::default(EqualsChar),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(StructEnd),
        ];
        let expected = 0b11111111;
        let actual = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens).expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_u8_random_partial() {
        let tokens = vec![
            Token::default(EqualsChar),
            Token::default(Zero),
            Token::default(One),
            Token::default(Zero),
            Token::default(One),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = 0b01010;
        let actual = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens).expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_u8_random_partial_1() {
        let tokens = vec![
            Token::default(EqualsChar),
            Token::default(Zero),
            Token::default(One),
            Token::default(One),
            Token::default(Zero),
            Token::default(One),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = 0b011010;
        let actual = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens).expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unexpected_token() {
        let tokens = vec![
            Token::default(EqualsChar),
            Token::new(0, 1, StructEnd),
        ];
        let expected = Parser(UnexpectedToken { parsed_type: U8, current_value_slice: &tokens, expected_tokens: vec![Zero, One]});
        if let Err(actual) = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn test_tokens_unexpected_end() {
        let tokens = vec![
            Token::default(EqualsChar),
        ];
        let expected = Error::Parser(TokensUnexpectedEnd { parsed_type: U8, current_value_slice: &tokens, expected_tokens: vec![Zero, One]});
        if let Err(actual) = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }
}
