use std::iter::Peekable;
use std::vec::IntoIter;

use self::States::*;
use crate::token::Token::*;
use crate::token::*;

pub(super) fn parse_u8(tokens: &mut Peekable<IntoIter<Token>>) -> u8 {
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
    fn next_state(self, tokens: &mut Peekable<IntoIter<Token>>) -> Self {
        let token = tokens.peek().expect("The source code ended prematurely");
        match self {
            Start => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit0(0)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit0(1)
                }
                _ => panic!("Expected an u8 beginning with either an  or a digit."),
            },
            Digit0(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit1(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit1((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit1(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit1(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit2((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit2(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit3(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit3((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit3(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit4(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit4((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit4(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit5(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit5((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit5(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    Digit6(number << 1)
                }
                One(line, offset) => {
                    tokens.next();
                    Digit6((number << 1) + 1)
                }
                _ => End(number),
            },
            Digit6(number) => match token {
                Zero(line, offset) => {
                    tokens.next();
                    End(number << 1)
                }
                One(line, offset) => {
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
        let mut tokens = vec![
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter()
        .peekable();
        assert_eq!(parse_u8(&mut tokens), 0);
    }

    #[test]
    fn test_parse_u8_max() {
        let mut tokens = vec![
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter()
        .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b11111111);
    }

    #[test]
    fn test_parse_u8_random_partial() {
        let mut tokens = vec![
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter()
        .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b01010);
    }

    #[test]
    fn test_parse_u8_random_partial_1() {
        let mut tokens = vec![
            Zero(0, 0),
            One(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
        ]
        .into_iter()
        .peekable();
        assert_eq!(parse_u8(&mut tokens), 0b011010);
    }
}
