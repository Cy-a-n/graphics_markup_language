#[derive(PartialEq)]
pub struct Token {
    value: TokenValue,
    line: usize,
    offset_start_inclusive: usize,
    offset_end_exclusive: usize,
}

impl Token {
    #[allow(unused)]
    pub fn default(value: TokenValue) -> Self {
        Token {
            line: 0,
            offset_start_inclusive: 0,
            offset_end_exclusive: 0,
            value,
        }
    }
    pub fn new(line: usize, offset_start_inclusive: usize, value: TokenValue) -> Self {
        Token {
            line,
            offset_start_inclusive,
            offset_end_exclusive: offset_start_inclusive + TokenValue::length(&value),
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

    #[allow(unused)]
    pub fn offset_end_exclusive(&self) -> &usize {
        &self.offset_end_exclusive
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("value", &self.value)
            .field("line", &self.line)
            .field("offset_start_inclusive", &self.offset_start_inclusive)
            .field("offset_end_exclusive", &self.offset_end_exclusive)
            .finish()
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    PrimitiveValue,
    NegativeValue,
    PositiveValue,
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

impl TokenValue {
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
    pub fn length(&self) -> usize {
        match self {
            TokenValue::PrimitiveValue => Self::LENGTH_PRIMITIVE_VALUE,
            TokenValue::NegativeValue => Self::LENGTH_NEGATIVE_VALUE,
            TokenValue::PositiveValue => Self::LENGTH_POSITIVE_VALUE,
            TokenValue::Zero => Self::LENGTH_ZERO,
            TokenValue::One => Self::LENGTH_ONE,
            TokenValue::StructStart => Self::LENGTH_STRUCT_START,
            TokenValue::StructEnd => Self::LENGTH_STRUCT_END,
            TokenValue::ArrayStart => Self::LENGTH_ARRAY_START,
            TokenValue::ArrayEnd => Self::LENGTH_ARRAY_END,
            TokenValue::AttributRed => Self::LENGTH_ATTRIBUT_RED,
            TokenValue::AttributGreen => Self::LENGTH_ATTRIBUT_GREEN,
            TokenValue::AttributBlue => Self::LENGTH_ATTRIBUT_BLUE,
            TokenValue::AttributX => Self::LENGTH_ATTRIBUTE_X,
            TokenValue::AttributY => Self::LENGTH_ATTRIBUTE_Y,
            TokenValue::AttributPosition => Self::LENGTH_ATTRIBUTE_POSITION,
            TokenValue::AttributRotation => Self::LENGTH_ATTRIBUTE_ROTATION,
            TokenValue::AttributWidth => Self::LENGTH_ATTRIBUTE_WIDTH,
            TokenValue::AttributBorderColor => Self::LENGTH_ATTRIBUTE_BORDER_COLOR,
            TokenValue::AttributFillColor => Self::LENGTH_ATTRIBUTE_FILL_COLOR,
            TokenValue::AttributVertices => Self::LENGTH_ATTRIBUTE_VERTICES,
            TokenValue::AttributVisibleExtent => Self::LENGTH_ATTRIBUTE_VISIBLE_EXTENT,
            TokenValue::AttributBackgroundColor => Self::LENGTH_ATTRIBUTE_BACKGROUND_COLOR,
            TokenValue::AttributShapes => Self::LENGTH_ATTRIBUTE_SHAPES,
        }
    }

    #[allow(unused)]
    pub fn to_str(&self) -> &str {
        match self {
            TokenValue::PrimitiveValue => Self::STR_PRIMITIVE_VALUE,
            TokenValue::NegativeValue => Self::STR_NEGATIVE_VALUE,
            TokenValue::PositiveValue => Self::STR_POSITIVE_VALUE,
            TokenValue::Zero => Self::STR_ZERO,
            TokenValue::One => Self::STR_ONE,
            TokenValue::StructStart => Self::STR_STRUCT_START,
            TokenValue::StructEnd => Self::STR_STRUCT_END,
            TokenValue::ArrayStart => Self::STR_ARRAY_START,
            TokenValue::ArrayEnd => Self::STR_ARRAY_END,
            TokenValue::AttributRed => Self::STR_ATTRIBUT_RED,
            TokenValue::AttributGreen => Self::STR_ATTRIBUT_GREEN,
            TokenValue::AttributBlue => Self::STR_ATTRIBUT_BLUE,
            TokenValue::AttributX => Self::STR_ATTRIBUTE_X,
            TokenValue::AttributY => Self::STR_ATTRIBUTE_Y,
            TokenValue::AttributPosition => Self::STR_ATTRIBUTE_POSITION,
            TokenValue::AttributRotation => Self::STR_ATTRIBUTE_ROTATION,
            TokenValue::AttributWidth => Self::STR_ATTRIBUTE_WIDTH,
            TokenValue::AttributBorderColor => Self::STR_ATTRIBUTE_BORDER_COLOR,
            TokenValue::AttributFillColor => Self::STR_ATTRIBUTE_FILL_COLOR,
            TokenValue::AttributVertices => Self::STR_ATTRIBUTE_VERTICES,
            TokenValue::AttributVisibleExtent => Self::STR_ATTRIBUTE_VISIBLE_EXTENT,
            TokenValue::AttributBackgroundColor => Self::STR_ATTRIBUTE_BACKGROUND_COLOR,
            TokenValue::AttributShapes => Self::STR_ATTRIBUTE_SHAPES,
        }
    }
}
