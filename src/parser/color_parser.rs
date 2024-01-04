use crate::token::Token;
use std::iter::Peekable;
use std::{iter::Enumerate, slice::IterMut};

#[allow(unused)]
enum State {
    Start,
    ColorStart,
    Red,
    RedEnd,
    Green,
    GreenEnd,
    Blue,
    BlueEnd,
    ColorEnd,
}

impl State {
    #[allow(unused)]
    fn next_state(&self, tokens: &mut Peekable<Enumerate<IterMut<Token>>>) {
        match tokens.peek() {
            None => todo!(),
            Some(_) => todo!(),
        }
    }
}
