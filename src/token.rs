use std::fmt::{Formatter, Result};

use strum_macros::{Display, EnumString};

#[derive(PartialEq, Clone)]
pub struct Token {
    value: TokenValue,
    line: usize,
    offset_start_inclusive: usize,
    offset_end_exclusive: usize,
}

impl Token {
    #[cfg(test)]
    pub fn default(value: TokenValue) -> Self {
        Token {
            line: 0,
            offset_start_inclusive: 0,
            offset_end_exclusive: 0,
            value,
        }
    }

    #[cfg(test)]
    pub fn new(line: usize, offset_start_inclusive: usize, value: TokenValue) -> Self {
        Token {
            line,
            offset_start_inclusive,
            offset_end_exclusive: offset_start_inclusive + TokenValue::len(&value),
            value,
        }
    }

    pub fn new_from_end(line: usize, offset_end_exclusive: usize, value: TokenValue) -> Self {
        Token {
            line,
            offset_start_inclusive: offset_end_exclusive - value.len(),
            offset_end_exclusive,
            value,
        }
    }

    pub fn value(&self) -> &TokenValue {
        &self.value
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn offset_start_inclusive(&self) -> &usize {
        &self.offset_start_inclusive
    }

    pub fn _offset_end_exclusive(&self) -> &usize {
        &self.offset_end_exclusive
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Token")
            .field("value", &self.value)
            .field("line", &self.line)
            .field("offset_start_inclusive", &self.offset_start_inclusive)
            .field("offset_end_exclusive", &self.offset_end_exclusive)
            .finish()
    }
}

#[derive(Debug, PartialEq, Clone, EnumString, Display)]
pub enum TokenValue {
    #[strum(to_string = "=", serialize = "Equals")]
    Equals,
    #[strum(to_string = "-", serialize = "Minus")]
    Minus,
    #[strum(to_string = "+", serialize = "Plus")]
    Plus,
    #[strum(to_string = "0", serialize = "Zero")]
    Zero,
    #[strum(to_string = "1", serialize = "One")]
    One,
    #[strum(to_string = "{", serialize = "LeftBrace")]
    LeftBrace,
    #[strum(to_string = "}", serialize = "RightBrace")]
    RightBrace,
    #[strum(to_string = "[", serialize = "LeftBracket")]
    LeftBracket,
    #[strum(to_string = "]", serialize = "RightBracket")]
    RightBracket,
    #[strum(to_string = "red", serialize = "Red")]
    Red,
    #[strum(to_string = "green", serialize = "Green")]
    Green,
    #[strum(to_string = "blue", serialize = "Blue")]
    Blue,
    #[strum(to_string = "x", serialize = "X")]
    X,
    #[strum(to_string = "y", serialize = "Y")]
    Y,
    #[strum(to_string = "position", serialize = "Position")]
    Position,
    #[strum(to_string = "rotation", serialize = "Rotation")]
    Rotation,
    #[strum(to_string = "width", serialize = "Width")]
    Width,
    #[strum(to_string = "border_color", serialize = "BorderColor")]
    BorderColor,
    #[strum(to_string = "fill_color", serialize = "FillColor")]
    FillColor,
    #[strum(to_string = "vertices", serialize = "Vertices")]
    Vertices,
    #[strum(to_string = "visible_extent", serialize = "VisibleExtent")]
    VisibleExtent,
    #[strum(to_string = "background_color", serialize = "BackgroundColor")]
    BackgroundColor,
    #[strum(to_string = "children", serialize = "Children")]
    Children,
}

impl TokenValue {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}
