#[derive(Debug, PartialEq)]
pub enum Token {
    PrimitiveValue(usize, usize),
    NegativeValue(usize, usize),
    PositiveValue(usize, usize),
    Zero(usize, usize),
    One(usize, usize),
    StructStart(usize, usize),
    StructEnd(usize, usize),
    ArrayStart(usize, usize),
    ArrayEnd(usize, usize),
    AttributRed(usize, usize),
    AttributGreen(usize, usize),
    AttributBlue(usize, usize),
    AttributX(usize, usize),
    AttributY(usize, usize),
    AttributPosition(usize, usize),
    AttributRotation(usize, usize),
    AttributWidth(usize, usize),
    AttributBorderColor(usize, usize),
    AttributFillColor(usize, usize),
    AttributVertices(usize, usize),
    AttributVisibleExtent(usize, usize),
    AttributBackgroundColor(usize, usize),
    AttributShapes(usize, usize),
}

impl Token {
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
            Token::PrimitiveValue(_, _) => Self::LENGTH_PRIMITIVE_VALUE,
            Token::NegativeValue(_, _) => Self::LENGTH_NEGATIVE_VALUE,
            Token::PositiveValue(_, _) => Self::LENGTH_POSITIVE_VALUE,
            Token::Zero(_, _) => Self::LENGTH_ZERO,
            Token::One(_, _) => Self::LENGTH_ONE,
            Token::StructStart(_, _) => Self::LENGTH_STRUCT_START,
            Token::StructEnd(_, _) => Self::LENGTH_STRUCT_END,
            Token::ArrayStart(_, _) => Self::LENGTH_ARRAY_START,
            Token::ArrayEnd(_, _) => Self::LENGTH_ARRAY_END,
            Token::AttributRed(_, _) => Self::LENGTH_ATTRIBUT_RED,
            Token::AttributGreen(_, _) => Self::LENGTH_ATTRIBUT_GREEN,
            Token::AttributBlue(_, _) => Self::LENGTH_ATTRIBUT_BLUE,
            Token::AttributX(_, _) => Self::LENGTH_ATTRIBUTE_X,
            Token::AttributY(_, _) => Self::LENGTH_ATTRIBUTE_Y,
            Token::AttributPosition(_, _) => Self::LENGTH_ATTRIBUTE_POSITION,
            Token::AttributRotation(_, _) => Self::LENGTH_ATTRIBUTE_ROTATION,
            Token::AttributWidth(_, _) => Self::LENGTH_ATTRIBUTE_WIDTH,
            Token::AttributBorderColor(_, _) => Self::LENGTH_ATTRIBUTE_BORDER_COLOR,
            Token::AttributFillColor(_, _) => Self::LENGTH_ATTRIBUTE_FILL_COLOR,
            Token::AttributVertices(_, _) => Self::LENGTH_ATTRIBUTE_VERTICES,
            Token::AttributVisibleExtent(_, _) => Self::LENGTH_ATTRIBUTE_VISIBLE_EXTENT,
            Token::AttributBackgroundColor(_, _) => Self::LENGTH_ATTRIBUTE_BACKGROUND_COLOR,
            Token::AttributShapes(_, _) => Self::LENGTH_ATTRIBUTE_SHAPES,
        }
    }

    #[allow(unused)]
    pub fn to_str(&self) -> &str {
        match self {
            Token::PrimitiveValue(_, _) => Self::STR_PRIMITIVE_VALUE,
            Token::NegativeValue(_, _) => Self::STR_NEGATIVE_VALUE,
            Token::PositiveValue(_, _) => Self::STR_POSITIVE_VALUE,
            Token::Zero(_, _) => Self::STR_ZERO,
            Token::One(_, _) => Self::STR_ONE,
            Token::StructStart(_, _) => Self::STR_STRUCT_START,
            Token::StructEnd(_, _) => Self::STR_STRUCT_END,
            Token::ArrayStart(_, _) => Self::STR_ARRAY_START,
            Token::ArrayEnd(_, _) => Self::STR_ARRAY_END,
            Token::AttributRed(_, _) => Self::STR_ATTRIBUT_RED,
            Token::AttributGreen(_, _) => Self::STR_ATTRIBUT_GREEN,
            Token::AttributBlue(_, _) => Self::STR_ATTRIBUT_BLUE,
            Token::AttributX(_, _) => Self::STR_ATTRIBUTE_X,
            Token::AttributY(_, _) => Self::STR_ATTRIBUTE_Y,
            Token::AttributPosition(_, _) => Self::STR_ATTRIBUTE_POSITION,
            Token::AttributRotation(_, _) => Self::STR_ATTRIBUTE_ROTATION,
            Token::AttributWidth(_, _) => Self::STR_ATTRIBUTE_WIDTH,
            Token::AttributBorderColor(_, _) => Self::STR_ATTRIBUTE_BORDER_COLOR,
            Token::AttributFillColor(_, _) => Self::STR_ATTRIBUTE_FILL_COLOR,
            Token::AttributVertices(_, _) => Self::STR_ATTRIBUTE_VERTICES,
            Token::AttributVisibleExtent(_, _) => Self::STR_ATTRIBUTE_VISIBLE_EXTENT,
            Token::AttributBackgroundColor(_, _) => Self::STR_ATTRIBUTE_BACKGROUND_COLOR,
            Token::AttributShapes(_, _) => Self::STR_ATTRIBUTE_SHAPES,
        }
    }
}
