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

/// Parses the tokenized source code to simple polygons.
pub fn parse(tokens: &mut [Token]) -> Result<Vec<SimplePolygon>, Error> {
    // First we build a tree of polygons that corresponds one to one to the polygon structure of the source code.
    let polygon_tree = Polygon::from_tokens(&mut tokens.iter().enumerate().peekable(), tokens);
    match polygon_tree {
        // Then we simplify it.
        Ok(root_polygon) => Ok(simple_polygon::simplify(root_polygon)),
        Err(error) => Err(error),
    }
}
