use std::iter::{Enumerate, Peekable};
use std::vec::IntoIter;

use self::States::*;
use crate::token::TokenValue::*;
use crate::token::*;

use super::parser_panic;

pub(super) fn parse_i16(mut tokens: &mut Peekable<Enumerate<IntoIter<Token>>>) -> i16 {
    let mut state = Start;

    loop {
        state = state.next_state(&mut tokens);

        if let End(number, positive) = state {
            match positive {
                true => return number,
                false => return -number,
            }
        }
    }
}

enum States {
    Start,
    Sign(bool),
    Digit0(i16, bool),
    Digit1(i16, bool),
    Digit2(i16, bool),
    Digit3(i16, bool),
    Digit4(i16, bool),
    Digit5(i16, bool),
    Digit6(i16, bool),
    Digit7(i16, bool),
    Digit8(i16, bool),
    Digit9(i16, bool),
    Digit10(i16, bool),
    Digit11(i16, bool),
    Digit12(i16, bool),
    Digit13(i16, bool),
    End(i16, bool),
}

impl States {
    fn next_state(self, tokens: &mut Peekable<Enumerate<IntoIter<Token>>>) -> Self {
        let (index, token) = tokens.peek().expect("The source code ended prematurely");
        let token_value = token.value();

        match self {
            Start => match token_value {
                PositiveValue => {
                    tokens.next();
                    Sign(true)
                }
                NegativeValue => {
                    tokens.next();
                    Sign(false)
                }
                Zero => {
                    tokens.next();
                    Digit0(0, true)
                }
                One => {
                    tokens.next();
                    Digit0(1, true)
                }
                _ => parser_panic(token, index),
            },
            Sign(positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit0(0, positive)
                }
                One => {
                    tokens.next();
                    Digit0(1, positive)
                }
                _ => panic!("Expected at least one digit after the sign."),
            },
            Digit0(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit1(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit1((number << 1) + 1, positive)
                }
                _ => {
                    tokens.next();
                    End(number, positive)
                }
            },
            Digit1(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit1(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit2((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit2(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit3(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit3((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit3(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit4(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit4((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit4(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit5(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit5((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit5(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit6(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit6((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit6(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit7(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit7((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit7(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit8(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit8((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit8(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit9(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit9((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit9(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit10(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit10((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit10(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit11(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit11((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit11(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit12(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit12((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit12(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    Digit13(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit13((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit13(number, positive) => match token_value {
                Zero => {
                    tokens.next();
                    End(number << 1, positive)
                }
                One => {
                    tokens.next();
                    End((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            End(_, _) => panic!("The next_state method shouldn't be called on End!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_0_full() {
        let mut tokens = vec![
            Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();

        assert_eq!(parse_i16(&mut tokens), 0);
    }

    #[test]
    fn positive_0_full() {
        let mut tokens = vec![
            PositiveValue,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), 0);
    }

    #[test]
    fn negative_0_full() {
        let mut tokens = vec![
            NegativeValue,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), 0);
    }

    #[test]
    fn unsigned_max() {
        let mut tokens = vec![
            One, One, One, One, One, One, One, One, One, One, One, One, One, One, One, StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), 0b111111111111111);
    }

    #[test]
    fn positive_max() {
        let mut tokens = vec![
            PositiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), 0b111111111111111);
    }

    #[test]
    fn negative_max() {
        let mut tokens = vec![
            NegativeValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), -0b111111111111111);
    }

    #[test]
    fn random_partial() {
        let mut tokens = vec![
            NegativeValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>()
        .into_iter()
        .enumerate()
        .peekable();
        assert_eq!(parse_i16(&mut tokens), -0b11111111);
    }

    #[test]
    fn random_partial_1() {
        let mut tokens = vec![PositiveValue, Zero, One, One, Zero, One, Zero, StructEnd]
            .into_iter()
            .map(|token_value| Token::default(token_value))
            .collect::<Vec<Token>>()
            .into_iter()
            .enumerate()
            .peekable();
        assert_eq!(parse_i16(&mut tokens), 0b011010);
    }
}
