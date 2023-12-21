use std::vec::IntoIter;

use crate::token::*;
use crate::token::Token::*;
use self::States::*;

fn parse_i16(tokens: &mut IntoIter<Token>) -> i16 {
    fn next_token(tokens: &mut IntoIter<Token>) -> Token {
        
    }

    match next_token(tokens) {
        PositiveValue => 
    }
}

enum States {
    Start,
    Sign,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6   ,
    Digit7  ,
    Digit8   ,
    Digit9   ,
    Digit10   ,
    Digit11   ,
    Digit12   ,
    Digit13   ,
    Digit14   ,
}

impl States {
    fn next_state(self, tokens : &mut IntoIter<Token> ) -> Self {
        let token = tokens.next().expect("The source code ended prematurely");
        match self {
            Start => 
        }
    }
    
}
