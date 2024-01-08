use std::fmt::{Formatter, Result};

#[derive(PartialEq, Clone)]
pub struct Token {
    value: Value,
    line: usize,
    offset_start_inclusive: usize,
    offset_end_exclusive: usize,
}

impl Token {
    #[allow(unused)]
    pub fn default(value: Value) -> Self {
        Token {
            line: 0,
            offset_start_inclusive: 0,
            offset_end_exclusive: 0,
            value,
        }
    }
    #[allow(unused)]
    pub fn new(line: usize, offset_start_inclusive: usize, value: Value) -> Self {
        Token {
            line,
            offset_start_inclusive,
            offset_end_exclusive: offset_start_inclusive + Value::len(&value),
            value,
        }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn line(&self) -> &usize {
        &self.line
    }

    pub fn offset_start_inclusive(&self) -> &usize {
        &self.offset_start_inclusive
    }

    #[allow(unused)]
    pub fn offset_end_exclusive(&self) -> &usize {
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

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
pub enum Value {
    EqualsChar,
    NegativeSign,
    PositiveSign,
    Zero,
    One,
    StructStart,
    StructEnd,
    ArrayStart,
    ArrayEnd,
    AttributRed,
    AttributGreen,
    AttributBlue,
    AttributX,
    AttributY,
    AttributPosition,
    AttributRotation,
    AttributWidth,
    AttributBorderColor,
    AttributFillColor,
    AttributVertices,
    AttributVisibleExtent,
    AttributBackgroundColor,
    AttributShapes,
}

impl Value {
    const LENGTH_PRIMITIVE_VALUE: usize = 1;
    const LENGTH_NEGATIVE_VALUE: usize = 1;
    const LENGTH_POSITIVE_VALUE: usize = 1;
    const LENGTH_ZERO: usize = 1;
    const LENGTH_ONE: usize = 1;
    const LENGTH_STRUCT_START: usize = 1;
    const LENGTH_STRUCT_END: usize = 1;
    const LENGTH_ARRAY_START: usize = 1;
    const LENGTH_ARRAY_END: usize = 1;
    const LENGTH_ATTRIBUT_RED: usize = 3;
    const LENGTH_ATTRIBUT_GREEN: usize = 5;
    const LENGTH_ATTRIBUT_BLUE: usize = 4;
    const LENGTH_ATTRIBUTE_X: usize = 1;
    const LENGTH_ATTRIBUTE_Y: usize = 1;
    const LENGTH_ATTRIBUTE_POSITION: usize = 8;
    const LENGTH_ATTRIBUTE_ROTATION: usize = 8;
    const LENGTH_ATTRIBUTE_WIDTH: usize = 5;
    const LENGTH_ATTRIBUTE_BORDER_COLOR: usize = 12;
    const LENGTH_ATTRIBUTE_FILL_COLOR: usize = 10;
    const LENGTH_ATTRIBUTE_VERTICES: usize = 8;
    const LENGTH_ATTRIBUTE_VISIBLE_EXTENT: usize = 14;
    const LENGTH_ATTRIBUTE_BACKGROUND_COLOR: usize = 16;
    const LENGTH_ATTRIBUTE_SHAPES: usize = 6;

    const STR_PRIMITIVE_VALUE: &str = "=";
    const STR_NEGATIVE_VALUE: &str = "-";
    const STR_POSITIVE_VALUE: &str = "+";
    const STR_ZERO: &str = "0";
    const STR_ONE: &str = "1";
    const STR_STRUCT_START: &str = "{";
    const STR_STRUCT_END: &str = "}";
    const STR_ARRAY_START: &str = "[";
    const STR_ARRAY_END: &str = "]";
    const STR_ATTRIBUT_RED: &str = "red";
    const STR_ATTRIBUT_GREEN: &str = "green";
    const STR_ATTRIBUT_BLUE: &str = "blue";
    const STR_ATTRIBUTE_X: &str = "x";
    const STR_ATTRIBUTE_Y: &str = "y";
    const STR_ATTRIBUTE_POSITION: &str = "position";
    const STR_ATTRIBUTE_ROTATION: &str = "rotation";
    const STR_ATTRIBUTE_WIDTH: &str = "width";
    const STR_ATTRIBUTE_BORDER_COLOR: &str = "border_color";
    const STR_ATTRIBUTE_FILL_COLOR: &str = "fill_color";
    const STR_ATTRIBUTE_VERTICES: &str = "vertices";
    const STR_ATTRIBUTE_VISIBLE_EXTENT: &str = "visible_extent";
    const STR_ATTRIBUTE_BACKGROUND_COLOR: &str = "background_color";
    const STR_ATTRIBUTE_SHAPES: &str = "shapes";

    #[allow(unused)]
    pub fn len(&self) -> usize {
        match self {
            Value::EqualsChar => Self::LENGTH_PRIMITIVE_VALUE,
            Value::NegativeSign => Self::LENGTH_NEGATIVE_VALUE,
            Value::PositiveSign => Self::LENGTH_POSITIVE_VALUE,
            Value::Zero => Self::LENGTH_ZERO,
            Value::One => Self::LENGTH_ONE,
            Value::StructStart => Self::LENGTH_STRUCT_START,
            Value::StructEnd => Self::LENGTH_STRUCT_END,
            Value::ArrayStart => Self::LENGTH_ARRAY_START,
            Value::ArrayEnd => Self::LENGTH_ARRAY_END,
            Value::AttributRed => Self::LENGTH_ATTRIBUT_RED,
            Value::AttributGreen => Self::LENGTH_ATTRIBUT_GREEN,
            Value::AttributBlue => Self::LENGTH_ATTRIBUT_BLUE,
            Value::AttributX => Self::LENGTH_ATTRIBUTE_X,
            Value::AttributY => Self::LENGTH_ATTRIBUTE_Y,
            Value::AttributPosition => Self::LENGTH_ATTRIBUTE_POSITION,
            Value::AttributRotation => Self::LENGTH_ATTRIBUTE_ROTATION,
            Value::AttributWidth => Self::LENGTH_ATTRIBUTE_WIDTH,
            Value::AttributBorderColor => Self::LENGTH_ATTRIBUTE_BORDER_COLOR,
            Value::AttributFillColor => Self::LENGTH_ATTRIBUTE_FILL_COLOR,
            Value::AttributVertices => Self::LENGTH_ATTRIBUTE_VERTICES,
            Value::AttributVisibleExtent => Self::LENGTH_ATTRIBUTE_VISIBLE_EXTENT,
            Value::AttributBackgroundColor => Self::LENGTH_ATTRIBUTE_BACKGROUND_COLOR,
            Value::AttributShapes => Self::LENGTH_ATTRIBUTE_SHAPES,
        }
    }

    #[allow(unused)]
    pub fn to_str(&self) -> &str {
        match self {
            Value::EqualsChar => Self::STR_PRIMITIVE_VALUE,
            Value::NegativeSign => Self::STR_NEGATIVE_VALUE,
            Value::PositiveSign => Self::STR_POSITIVE_VALUE,
            Value::Zero => Self::STR_ZERO,
            Value::One => Self::STR_ONE,
            Value::StructStart => Self::STR_STRUCT_START,
            Value::StructEnd => Self::STR_STRUCT_END,
            Value::ArrayStart => Self::STR_ARRAY_START,
            Value::ArrayEnd => Self::STR_ARRAY_END,
            Value::AttributRed => Self::STR_ATTRIBUT_RED,
            Value::AttributGreen => Self::STR_ATTRIBUT_GREEN,
            Value::AttributBlue => Self::STR_ATTRIBUT_BLUE,
            Value::AttributX => Self::STR_ATTRIBUTE_X,
            Value::AttributY => Self::STR_ATTRIBUTE_Y,
            Value::AttributPosition => Self::STR_ATTRIBUTE_POSITION,
            Value::AttributRotation => Self::STR_ATTRIBUTE_ROTATION,
            Value::AttributWidth => Self::STR_ATTRIBUTE_WIDTH,
            Value::AttributBorderColor => Self::STR_ATTRIBUTE_BORDER_COLOR,
            Value::AttributFillColor => Self::STR_ATTRIBUTE_FILL_COLOR,
            Value::AttributVertices => Self::STR_ATTRIBUTE_VERTICES,
            Value::AttributVisibleExtent => Self::STR_ATTRIBUTE_VISIBLE_EXTENT,
            Value::AttributBackgroundColor => Self::STR_ATTRIBUTE_BACKGROUND_COLOR,
            Value::AttributShapes => Self::STR_ATTRIBUTE_SHAPES,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_str())
    }
}
