use std::iter::Peekable;
use std::vec::IntoIter;

use self::States::*;
use crate::token::Token::*;
use crate::token::*;

pub(super) fn parse_i16(tokens: &mut IntoIter<Token>) -> i16 {
    let mut state = Start;
    let mut tokens = tokens.peekable();

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
    fn next_state(self, tokens: &mut Peekable<&mut IntoIter<Token>>) -> Self {
        let token = tokens.peek().expect("The source code ended prematurely");
        match self {
            Start => match token {
                PositiveValue => {
                    tokens.next();
                    Sign(true)
                }
                NegativValue => {
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
                _ => panic!("Expected an i16 beginning with either an sign or a digit."),
            },
            Sign(positive) => match token {
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
            Digit0(number, positive) => match token {
                Zero => {
                    tokens.next();
                    Digit1(number << 1, positive)
                }
                One => {
                    tokens.next();
                    Digit1((number << 1) + 1, positive)
                }
                _ => End(number, positive),
            },
            Digit1(number, positive) => match token {
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
            Digit2(number, positive) => match token {
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
            Digit3(number, positive) => match token {
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
            Digit4(number, positive) => match token {
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
            Digit5(number, positive) => match token {
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
            Digit6(number, positive) => match token {
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
            Digit7(number, positive) => match token {
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
            Digit8(number, positive) => match token {
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
            Digit9(number, positive) => match token {
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
            Digit10(number, positive) => match token {
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
            Digit11(number, positive) => match token {
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
            Digit12(number, positive) => match token {
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
            Digit13(number, positive) => match token {
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
    fn test_parse_i16_0_unsigned_full() {
        let tokens = vec![
            Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero,
            StructEnd,
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0);
    }

    #[test]
    fn test_parse_i16_0_positive_full() {
        let tokens = vec![
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
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0);
    }

    #[test]
    fn test_parse_i16_0_negative_full() {
        let tokens = vec![
            NegativValue,
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
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0);
    }

    #[test]
    fn test_parse_i16_max_unsigned() {
        let tokens = vec![
            One, One, One, One, One, One, One, One, One, One, One, One, One, One, One, StructEnd,
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0b111111111111111);
    }

    #[test]
    fn test_parse_i16_max_positive() {
        let tokens = vec![
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
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0b111111111111111);
    }

    #[test]
    fn test_parse_i16_max_negative() {
        let tokens = vec![
            NegativValue,
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
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), -0b111111111111111);
    }

    #[test]
    fn test_parse_i16_random_partial() {
        let tokens = vec![
            NegativValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
        ];
        assert_eq!(parse_i16(&mut tokens.into_iter()), -0b11111111);
    }

    #[test]
    fn test_parse_i16_random_partial_1() {
        let tokens = vec![PositiveValue, Zero, One, One, Zero, One, Zero, StructEnd];
        assert_eq!(parse_i16(&mut tokens.into_iter()), 0b011010);
    }
}
