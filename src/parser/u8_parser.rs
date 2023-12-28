use std::iter::{Enumerate, Peekable};
use std::vec::IntoIter;

use self::States::*;
use crate::token::TokenValue::*;
use crate::token::*;

use super::parser_panic;

pub(super) fn parse_u8(tokens: &mut Peekable<Enumerate<IntoIter<Token>>>) -> u8 {
    let mut state = Start;

    loop {
        state = state.next_state(tokens);

        if let End(number) = state {
            return number;
        }
    }
}

enum States {
    Start,
    Digit0(u8),
    Digit1(u8),
    Digit2(u8),
    Digit3(u8),
    Digit4(u8),
    Digit5(u8),
    Digit6(u8),
    End(u8),
}

impl States {
    fn next_state(self, tokens: &mut Peekable<Enumerate<IntoIter<Token>>>) -> Self {
        let (index, token) = tokens.peek().expect("The source code ended prematurely");
        let token_value = token.value();

        match self {
            Start => match token_value {
                Zero => {
                    tokens.next();
                    Digit0(0)
                }
                One => {
                    tokens.next();
                    Digit0(1)
                }
                _ => parser_panic(token, index),
            },
            Digit0(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit1(number << 1)
                }
                One => {
                    tokens.next();
                    Digit1((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit1(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit1(number << 1)
                }
                One => {
                    tokens.next();
                    Digit2((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit2(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit3(number << 1)
                }
                One => {
                    tokens.next();
                    Digit3((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit3(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit4(number << 1)
                }
                One => {
                    tokens.next();
                    Digit4((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit4(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit5(number << 1)
                }
                One => {
                    tokens.next();
                    Digit5((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit5(number) => match token_value {
                Zero => {
                    tokens.next();
                    Digit6(number << 1)
                }
                One => {
                    tokens.next();
                    Digit6((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit6(number) => match token_value {
                Zero => {
                    tokens.next();
                    End(number << 1)
                }
                One => {
                    tokens.next();
                    End((number << 1) + 1)
                }
                _ => End(number),
            },
            End(_) => panic!("The next_state method shouldn't be called on End!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u8_0_full() {
        let mut tokens = vec![Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, StructEnd]
            .into_iter()
            .map(|token_value| Token::default(token_value))
            .collect::<Vec<Token>>()
            .into_iter()
            .enumerate()
            .peekable();

        assert_eq!(parse_u8(&mut tokens), 0);
    }

    #[test]
    fn test_parse_u8_max() {
        let mut tokens = vec![One, One, One, One, One, One, One, One, StructEnd]
            .into_iter()
            .map(|token_value| Token::default(token_value))
            .collect::<Vec<Token>>()
            .into_iter()
            .enumerate()
            .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b11111111);
    }

    #[test]
    fn test_parse_u8_random_partial() {
        let mut tokens = vec![Zero, One, Zero, One, Zero, StructEnd]
            .into_iter()
            .map(|token_value| Token::default(token_value))
            .collect::<Vec<Token>>()
            .into_iter()
            .enumerate()
            .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b01010);
    }

    #[test]
    fn test_parse_u8_random_partial_1() {
        let mut tokens = vec![Zero, One, One, Zero, One, Zero, StructEnd]
            .into_iter()
            .map(|token_value| Token::default(token_value))
            .collect::<Vec<Token>>()
            .into_iter()
            .enumerate()
            .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b011010);
    }
}
