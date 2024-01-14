use crate::{error_handling::Error, token::Token};

use polygon::Polygon;

pub use color::Color;
pub use point::Point;
pub use simple_polygon::SimplePolygon;

mod color;
mod i16;
mod macros;
mod point;
mod polygon;
mod simple_polygon;
mod u8;

pub fn parse(tokens: &mut [Token]) -> Result<Vec<SimplePolygon>, Error> {
    match Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), tokens) {
        Ok(root_polygon) => Ok(simple_polygon::simplify(root_polygon)),
        Err(error) => Err(error),
    }
}
