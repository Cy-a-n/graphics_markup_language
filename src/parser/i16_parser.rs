use self::State::*;
use super::macros::{transition, transition_return_on_unexpected};
use crate::error_handling::Error;
use crate::error_handling::Error::Parser;
use crate::error_handling::ParsedType::U8;
use crate::error_handling::ParserError::UnexpectedEnd;
use crate::error_handling::ParserError::UnexpectedToken;
use crate::token::Token;
use crate::token::Value;
use crate::token::Value::{Equals, NegativeSign, One, PositiveSign, Zero};
use std::iter::Enumerate;
use std::str::FromStr;
use std::{iter::Peekable, slice::Iter};

#[allow(unused)]
pub(super) fn parse_i16<'a>(
    tokens_iter: &mut Peekable<Enumerate<Iter<Token>>>,
    tokens: &'a [Token],
) -> Result<i16, Error<'a>> {
    let slice_from_value_start = &tokens[tokens_iter
        .peek()
        .expect("BUG: 'tokens_iter' should have at least one token.")
        .0..];
    let mut state = Start;

    loop {
        state = state.next_state(tokens_iter);
        match state {
            Return((value, negative)) => match negative {
                true => return Ok(-value),
                false => return Ok(value),
            },
            UnexpectedEnd(expected_tokens) => {
                return Err(Parser(UnexpectedEnd {
                    parsed_type: U8,
                    current_value_slice: slice_from_value_start,
                    expected_tokens,
                }))
            }
            UnexpectedToken(expected_tokens, i) => {
                return Err(Parser(UnexpectedToken {
                    parsed_type: U8,
                    current_value_slice: &slice_from_value_start[..i + 1],
                    expected_tokens,
                }))
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
enum State {
    Start,
    EqualsSign,
    Sign(bool),
    Digit0((i16, bool)),
    Digit1((i16, bool)),
    Digit2((i16, bool)),
    Digit3((i16, bool)),
    Digit4((i16, bool)),
    Digit5((i16, bool)),
    Digit6((i16, bool)),
    Digit7((i16, bool)),
    Digit8((i16, bool)),
    Digit9((i16, bool)),
    Digit10((i16, bool)),
    Digit11((i16, bool)),
    Digit12((i16, bool)),
    Digit13((i16, bool)),
    Return((i16, bool)),
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
                PositiveSign => Sign(false),
                NegativeSign => Sign(true),
                Zero => Digit0((0, false)),
                One => Digit0((1, false)),
            ),
            Sign(negative) => transition!(tokens_iter,
                Zero => Digit0((0, negative)),
                One => Digit0((1, negative)),
            ),
            Digit0((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit1((value << 1, negative)),
                One => Digit1(((value << 1) + 1, negative)),
            ),
            Digit1((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit2((value << 1, negative)),
                One => Digit2(((value << 1) + 1, negative)),
            ),
            Digit2((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit3((value << 1, negative)),
                One => Digit3(((value << 1) + 1, negative)),
            ),
            Digit3((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit4((value << 1, negative)),
                One => Digit4(((value << 1) + 1, negative)),
            ),
            Digit4((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit5((value << 1, negative)),
                One => Digit5(((value << 1) + 1, negative)),
            ),
            Digit5((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit6((value << 1, negative)),
                One => Digit6(((value << 1) + 1, negative)),
            ),
            Digit6((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit7((value << 1, negative)),
                One => Digit7(((value << 1) + 1, negative)),
            ),
            Digit7((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit8((value << 1, negative)),
                One => Digit8(((value << 1) + 1, negative)),
            ),
            Digit8((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit9((value << 1, negative)),
                One => Digit9(((value << 1) + 1, negative)),
            ),
            Digit9((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit10((value << 1, negative)),
                One => Digit10(((value << 1) + 1, negative)),
            ),
            Digit10((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit11((value << 1, negative)),
                One => Digit11(((value << 1) + 1, negative)),
            ),
            Digit11((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit12((value << 1, negative)),
                One => Digit12(((value << 1) + 1, negative)),
            ),
            Digit12((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Digit13((value << 1, negative)),
                One => Digit13(((value << 1) + 1, negative)),
            ),
            Digit13((value, negative)) => transition_return_on_unexpected!(tokens_iter, (value, negative),
                Zero => Return((value << 1, negative)),
                One => Return(((value << 1) + 1, negative)),
            ),
            Return((_, _)) => panic!("BUG: The `next_state` method should never be called on the `End` state. 'state': '{self:?}'."),
            UnexpectedEnd(_) => panic!("BUG: The `next_state` method should never be called on the `TokensUnexpectedEnd` state. 'state': '{self:?}'."),
            UnexpectedToken(_, _) => panic!("BUG: The `next_state` method should never be called on the `UnexpectedToken` state. 'state': '{self:?}'."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error_handling::ParserError::{UnexpectedEnd, UnexpectedToken},
        token::Value::StructEnd,
    };

    #[test]
    fn unsigned_0_full() {
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
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = 0;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn positive_0_full() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(PositiveSign),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
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
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_0_full() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(NegativeSign),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
            Token::default(Zero),
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
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);

        assert_eq!(expected, actual);
    }

    #[test]
    fn unsigned_max() {
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
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(StructEnd),
        ];
        let expected = i16::MAX;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn positive_max() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(PositiveSign),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
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
        let expected = i16::MAX;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn negative_max() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(NegativeSign),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
            Token::default(One),
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
        let expected = i16::MIN + 1;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn random_partial() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(NegativeSign),
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
        let expected = -0b11111111;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn random_partial_1() {
        let tokens = vec![
            Token::default(Equals),
            Token::default(PositiveSign),
            Token::default(Zero),
            Token::default(One),
            Token::default(One),
            Token::default(Zero),
            Token::default(One),
            Token::default(Zero),
            Token::default(StructEnd),
        ];
        let expected = 0b011010;
        let actual = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens)
            .expect("The parser failed.");

        assert_eq!(expected, actual);
    }

    #[test]
    fn unexpected_token() {
        let tokens = vec![Token::default(Equals), Token::new(0, 1, StructEnd)];
        let expected = Parser(UnexpectedToken {
            parsed_type: U8,
            current_value_slice: &tokens,
            expected_tokens: vec![PositiveSign, NegativeSign, Zero, One],
        });
        if let Err(actual) = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }

    #[test]
    fn tokens_unexpected_end() {
        let tokens = vec![Token::default(Equals)];
        let expected = Error::Parser(UnexpectedEnd {
            parsed_type: U8,
            current_value_slice: &tokens,
            expected_tokens: vec![PositiveSign, NegativeSign, Zero, One],
        });
        if let Err(actual) = parse_i16(&mut tokens.iter().enumerate().peekable(), &tokens) {
            assert_eq!(expected, actual);
        } else {
            panic!("The parser succeeded when it shouldn't have.")
        }
    }
}
