use crate::{error_handling::Error, token::Token};

mod color;
mod i16;
mod macros;
mod point;
mod polygon;
mod u8;

pub use {color::Color, point::Point, polygon::Polygon};

pub fn parse(tokens: &mut [Token]) -> Result<Polygon, Error> {
    polygon::Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), tokens)
}
