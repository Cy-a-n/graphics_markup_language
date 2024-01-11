use self::State::*;
use super::macros::{transition, transition_return_on_unexpected};
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType::U8;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{Equals,One, Zero};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_u8<'a>(tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>, tokens: &'a [Token]) -> Result<u8, Error<'a>> {
    let slice_from_value_start = &tokens[tokens_iter.peek().expect("BUG: 'tokens_iter' should have at least one token.").0..];
    let mut state = Start;

    loop {
        state = state.next_state(tokens_iter);
        match state {
            Return(value) => return Ok(value),
            UnexpectedEnd(expected_tokens) => return Err(Parser(UnexpectedEnd { parsed_type: U8, current_value_slice: slice_from_value_start, expected_tokens })),
            UnexpectedToken(expected_tokens, i) => return Err(Parser(UnexpectedToken { parsed_type: U8, current_value_slice: &slice_from_value_start[..i + 1], expected_tokens })),
            _ => {},
        }
    }
}

#[derive(Debug)]
enum State {
    Start,
    EqualsSign,
    Digit0(u8),
    Digit1(u8),
    Digit2(u8),
    Digit3(u8),
    Digit4(u8),
    Digit5(u8),
    Digit6(u8),
    Return(u8),
    UnexpectedEnd(Vec<Value>),
    UnexpectedToken(Vec<Value>, usize),
}

impl State {
    fn next_state(self, tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>) -> Self {
        match self {
            Start => transition!(tokens_iter,
                Equals => EqualsSign
            ),
            EqualsSign => transition!(tokens_iter,
                Zero => Digit0(0),
                One => Digit0(1),
            ),
            Digit0(value) => transition_return_on_unexpected!(tokens_iter, value, 
                Zero => Digit1(value << 1),
                One => Digit1((value << 1) + 1),
            ),
            Digit1(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Digit2(value << 1),
                One => Digit2((value << 1) + 1),
            ),
            Digit2(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Digit3(value << 1),
                One => Digit3((value << 1) + 1),
            ),
            Digit3(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Digit4(value << 1),
                One => Digit4((value << 1) + 1),
            ),
            Digit4(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Digit5(value << 1),
                One => Digit5((value << 1) + 1),
            ),
            Digit5(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Digit6(value << 1),
                One => Digit6((value << 1) + 1),
            ),
            Digit6(value) => transition_return_on_unexpected!(tokens_iter, value,
                Zero => Return(value << 1),
                One => Return((value << 1) + 1),
            ),
            Return(_) => panic!("BUG: The 'next_state' method should never be called on the 'End' state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The 'next_state' method should never be called on the 'TokensUnexpectedEnd' state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The 'next_state' method should never be called on the 'UnexpectedToken' state. 'state': '{self:?}'."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{token::Value::{One, StructEnd, Zero}, error_handling::ParserError::{UnexpectedToken, UnexpectedEnd}};

    #[test]
    fn parse_u8_0_full() {
        let tokens = vec![
            Token::default(Equals),
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
    fn parse_u8_max() {
        let tokens = vec![
            Token::default(Equals),
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
    fn parse_u8_random_partial() {
        let tokens = vec![
            Token::default(Equals),
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
    fn parse_u8_random_partial_1() {
        let tokens = vec![
            Token::default(Equals),
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
    fn unexpected_token() {
        let tokens = vec![
            Token::default(Equals),
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
    fn tokens_unexpected_end() {
        let tokens = vec![
            Token::default(Equals),
        ];
        let expected = Error::Parser(UnexpectedEnd { parsed_type: U8, current_value_slice: &tokens, expected_tokens: vec![Zero, One]});
        if let Err(actual) = parse_u8(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }
}
