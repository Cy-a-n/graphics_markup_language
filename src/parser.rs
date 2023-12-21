mod primitive_parser;

use std::vec::IntoIter;

use self::ParserState::*;
use crate::draw_elements::*;
use crate::token::Token;
use crate::token::Token::*;

pub fn to_polygons(tokens: Vec<Token>) -> Vec<Vec<(i16, i16)>> {
    let mut tokens = tokens.into_iter();
    let state = Start;

    vec![vec![(0, 0)]]
}

enum ParserState {
    Start,
    MainStart(Main),
    MainVisibleExtent(Main),
    MainVisibleExtentStart(Main),
    MainVisibleExtentX(Main),
    MainVisibleExtentXEnd(Main),
    MainVisibleExtentY(Main),
    MainVisibleExtentYEnd(Main),
    MainVisibleExtentEnd(Main),
    MainBackgroundColor(Main),
    MainBackgroundColorStart(Main),
    MainBackgroundColorRed(Main),
    MainBackgroundColorRedEnd(Main),
    MainBackgroundColorGreen(Main),
    MainBackgroundColorGreenEnd(Main),
    MainBackgroundColorBlue(Main),
    MainBackgroundColorBlueEnd(Main),
    MainBackgroundColorEnd(Main),
    MainShapes(Main),
    MainShapesStart(Main),
    MainShapesElementStart(Main),
    MainShapesElementPosition(Main),
    MainShapesElementPositionStart(Main),
    MainShapesElementPositionX(Main),
    MainShapesElementPositionXEnd(Main),
    MainShapesElementPositionY(Main),
    MainShapesElementPositionYEnd(Main),
    MainShapesElementPositionEnd(Main),
    MainShapesElementRotation(Main),
    MainShapesElementRotationEnd(Main),
    MainShapesPolygonWidth(Main),
    MainShapesPolygonWidthEnd(Main),
    MainShapesPolygonBorderColor(Main),
    MainShapesPolygonBorderColorStart(Main),
    MainShapesPolygonBorderColorRed(Main),
    MainShapesPolygonBorderColorRedEnd(Main),
    MainShapesPolygonBorderColorGreen(Main),
    MainShapesPolygonBorderColorGreenEnd(Main),
    MainShapesPolygonBorderColorBlue(Main),
    MainShapesPolygonBorderColorBlueEnd(Main),
    MainShapesPolygonBorderColorEnd(Main),
    MainShapesPolygonFillColor(Main),
    MainShapesPolygonFillColorStart(Main),
    MainShapesPolygonFillColorRed(Main),
    MainShapesPolygonFillColorRedEnd(Main),
    MainShapesPolygonFillColorGreen(Main),
    MainShapesPolygonFillColorGreenEnd(Main),
    MainShapesPolygonFillColorBlue(Main),
    MainShapesPolygonFillColorBlueEnd(Main),
    MainShapesPolygonFillColorEnd(Main),
    MainShapesPolygonVertices(Main),
    MainShapesPolygonVerticesStart(Main),
    MainShapesPolygonVerticesVertexStart(Main),
    MainShapesPolygonVerticesVertexX(Main),
    MainShapesPolygonVerticesVertexXEnd(Main),
    MainShapesPolygonVerticesVertexY(Main),
    MainShapesPolygonVerticesVertexYEnd(Main),
    MainShapesPolygonVerticesEnd(Main),
    MainShapesGroupShapes(Main),
    MainShapesGroupShapesStart(Main),
    MainShapesGroupShapesPolygonStart(Main),
    MainShapesGroupShapesPolygonPosition(Main),
    MainShapesGroupShapesPolygonPositionStart(Main),
    MainShapesGroupShapesPolygonPositionX(Main),
    MainShapesGroupShapesPolygonPositionXEnd(Main),
    MainShapesGroupShapesPolygonPositionY(Main),
    MainShapesGroupShapesPolygonPositionYEnd(Main),
    MainShapesGroupShapesPolygonPositionEnd(Main),
    MainShapesGroupShapesPolygonRotation(Main),
    MainShapesGroupShapesPolygonRotationEnd(Main),
    MainShapesGroupShapesPolygonWidth(Main),
    MainShapesGroupShapesPolygonWidthEnd(Main),
    MainShapesGroupShapesPolygonBorderColor(Main),
    MainShapesGroupShapesPolygonBorderColorStart(Main),
    MainShapesGroupShapesPolygonBorderColorRed(Main),
    MainShapesGroupShapesPolygonBorderColorRedEnd(Main),
    MainShapesGroupShapesPolygonBorderColorGreen(Main),
    MainShapesGroupShapesPolygonBorderColorGreenEnd(Main),
    MainShapesGroupShapesPolygonBorderColorBlue(Main),
    MainShapesGroupShapesPolygonBorderColorBlueEnd(Main),
    MainShapesGroupShapesPolygonBorderColorEnd(Main),
    MainShapesGroupShapesPolygonFillColor(Main),
    MainShapesGroupShapesPolygonFillColorStart(Main),
    MainShapesGroupShapesPolygonFillColorRed(Main),
    MainShapesGroupShapesPolygonFillColorRedEnd(Main),
    MainShapesGroupShapesPolygonFillColorGreen(Main),
    MainShapesGroupShapesPolygonFillColorGreenEnd(Main),
    MainShapesGroupShapesPolygonFillColorBlue(Main),
    MainShapesGroupShapesPolygonFillColorBlueEnd(Main),
    MainShapesGroupShapesPolygonFillColorEnd(Main),
    MainShapesGroupShapesPolygonVertices(Main),
    MainShapesGroupShapesPolygonVerticesStart(Main),
    MainShapesGroupShapesPolygonVerticesVertexStart(Main),
    MainShapesGroupShapesPolygonVerticesVertexX(Main),
    MainShapesGroupShapesPolygonVerticesVertexXEnd(Main),
    MainShapesGroupShapesPolygonVerticesVertexY(Main),
    MainShapesGroupShapesPolygonVerticesVertexYEnd(Main),
    MainShapesGroupShapesPolygonVerticesVertexEnd(Main),
    MainShapesGroupShapesPolygonVerticesEnd(Main),
    MainShapesGroupShapesPolygonEnd(Main),
    MainShapesGroupShapesEnd(Main),
    MainShapesEnd(Main),
    MainEnd(Main),
    Err,
}

impl ParserState {
    fn next_state(self, tokens: &mut IntoIter<Token>) -> Self {
        let token = tokens.next().expect("The source code ended prematurely");
        match self {
            Err => panic!("The `next_state`-method should never be called on ParserState::Err"),
            Start => match token {
                StructStart => MainStart(Main::default()),
                _ => panic!("Encountered invalid token"),
            },
            MainStart(main) => match token {
                AttributVisibleExtent => MainVisibleExtent(main),
                _ => Err,
            },
            MainVisibleExtent(main) => match token {
                AttributBackgroundColor => MainBackgroundColor(main),
                StructStart => MainVisibleExtentStart(main),
                _ => Err,
            },
            MainVisibleExtentStart(main) => match token {
                AttributX => MainVisibleExtentX(main),
                _ => Err,
            },
            MainVisibleExtentX(main) => match token {
                AttributY => MainVisibleExtentY(main),
                PrimitiveValue => main.x = parse_i16(token),
                _ => Err,
            },
            MainVisibleExtentXEnd(main) => match token {
                AttributY => MainVisibleExtentY(main),
                _ => Err,
            },
            MainVisibleExtentY(main) => match token {
                StructEnd => MainVisibleExtentEnd(main),
                PrimitiveValue => MainVisibleExtentYEnd(main),
                _ => Err,
            },
            MainVisibleExtentYEnd(main) => match token {
                StructEnd => MainVisibleExtentEnd(main),
                _ => Err,
            },
            MainVisibleExtentEnd(main) => match token {
                AttributBackgroundColor => MainBackgroundColor(main),
                _ => Err,
            },
            MainBackgroundColor(main) => match token {
                AttributShapes => MainShapes(main),
                StructStart => MainBackgroundColorStart(main),
                _ => Err,
            },
            MainBackgroundColorStart(main) => match token {
                AttributRed => MainBackgroundColorRed(main),
                _ => Err,
            },
            MainBackgroundColorRed(main) => match token {
                AttributGreen => MainBackgroundColorGreen(main),
                PrimitiveValue => MainBackgroundColorRedEnd(main),
                _ => Err,
            },
            MainBackgroundColorRedEnd(main) => match token {
                AttributGreen => MainBackgroundColorGreen(main),
                _ => Err,
            },
            MainBackgroundColorGreen(main) => match token {
                AttributBlue => MainBackgroundColorBlue(main),
                PrimitiveValue => MainBackgroundColorGreenEnd(main),
                _ => Err,
            },
            MainBackgroundColorGreenEnd(main) => match token {
                AttributBlue => MainBackgroundColorBlue(main),
                _ => Err,
            },
            MainBackgroundColorBlue(main) => match token {
                StructEnd => MainBackgroundColorEnd(main),
                PrimitiveValue => MainBackgroundColorBlueEnd(main),
                _ => Err,
            },
            MainBackgroundColorBlueEnd(main) => match token {
                StructEnd => MainBackgroundColorEnd(main),
                _ => Err,
            },
            MainBackgroundColorEnd(main) => match token {
                AttributShapes => MainShapes(main),
                _ => Err,
            },
            MainShapes(main) => match token {
                StructEnd => MainEnd(main),
                ArrayStart => MainShapesStart(main),
                _ => Err,
            },
            MainShapesStart(main) => match token {
                ArrayEnd => MainShapesEnd(main),
                StructStart => MainShapesElementStart(main),
                _ => Err,
            },
            MainShapesElementStart(main) => match token {
                AttributPosition => MainShapesElementPosition(main),
                _ => Err,
            },
            MainShapesElementPosition(main) => match token {
                AttributRotation => MainShapesElementRotation(main),
                StructStart => MainShapesElementPositionStart(main),
                _ => Err,
            },
            MainShapesElementPositionStart(main) => match token {
                AttributX => MainShapesElementPositionX(main),
                _ => Err,
            },
            MainShapesElementPositionX(main) => match token {
                AttributY => MainShapesElementPositionY(main),
                PrimitiveValue => MainShapesElementPositionXEnd(main),
                _ => Err,
            },
            MainShapesElementPositionXEnd(main) => match token {
                AttributY => MainShapesElementPositionY(main),
                _ => Err,
            },
            MainShapesElementPositionY(main) => match token {
                StructEnd => MainShapesElementPositionEnd(main),
                PrimitiveValue => MainShapesElementPositionYEnd(main),
                _ => Err,
            },
            MainShapesElementPositionYEnd(main) => match token {
                StructEnd => MainShapesElementPositionEnd(main),
                _ => Err,
            },
            MainShapesElementPositionEnd(main) => match token {
                AttributRotation => MainShapesElementRotation(main),
                _ => Err,
            },
            MainShapesElementRotation(main) => match token {
                AttributWidth => MainShapesPolygonWidth(main),
                AttributShapes => MainShapesGroupShapes(main),
                PrimitiveValue => MainShapesElementRotationEnd(main),
                _ => Err,
            },
            MainShapesElementRotationEnd(main) => match token {
                AttributWidth => MainShapesPolygonWidth(main),
                AttributShapes => MainShapesGroupShapes(main),
                _ => Err,
            },
            MainShapesPolygonWidth(main) => match token {
                AttributBorderColor => MainShapesPolygonBorderColor(main),
                PrimitiveValue => MainShapesPolygonWidthEnd(main),
                _ => Err,
            },
            MainShapesPolygonWidthEnd(main) => match token {
                AttributBorderColor => MainShapesPolygonBorderColor(main),
                _ => Err,
            },
            MainShapesPolygonBorderColor(main) => match token {
                AttributFillColor => MainShapesPolygonFillColor(main),
                StructStart => MainShapesPolygonBorderColorStart(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorStart(main) => match token {
                AttributRed => MainShapesPolygonBorderColorRed(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorRed(main) => match token {
                AttributGreen => MainShapesPolygonBorderColorGreen(main),
                PrimitiveValue => MainShapesPolygonBorderColorRedEnd(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorRedEnd(main) => match token {
                AttributGreen => MainShapesPolygonBorderColorGreen(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorGreen(main) => match token {
                AttributBlue => MainShapesPolygonBorderColorBlue(main),
                PrimitiveValue => MainShapesPolygonBorderColorGreenEnd(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorGreenEnd(main) => match token {
                AttributBlue => MainShapesPolygonBorderColorBlue(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorBlue(main) => match token {
                StructEnd => MainShapesPolygonBorderColorEnd(main),
                PrimitiveValue => MainShapesPolygonBorderColorBlueEnd(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorBlueEnd(main) => match token {
                StructEnd => MainShapesPolygonBorderColorEnd(main),
                _ => Err,
            },
            MainShapesPolygonBorderColorEnd(main) => match token {
                AttributFillColor => MainShapesPolygonFillColor(main),
                _ => Err,
            },
            MainShapesPolygonFillColor(main) => match token {
                AttributVertices => MainShapesPolygonVertices(main),
                StructStart => MainShapesPolygonFillColorStart(main),
                _ => Err,
            },
            MainShapesPolygonFillColorStart(main) => match token {
                AttributRed => MainShapesPolygonFillColorRed(main),
                _ => Err,
            },
            MainShapesPolygonFillColorRed(main) => match token {
                AttributGreen => MainShapesPolygonFillColorGreen(main),
                PrimitiveValue => MainShapesPolygonFillColorRedEnd(main),
                _ => Err,
            },
            MainShapesPolygonFillColorRedEnd(main) => match token {
                AttributGreen => MainShapesPolygonFillColorGreen(main),
                _ => Err,
            },
            MainShapesPolygonFillColorGreen(main) => match token {
                AttributBlue => MainShapesPolygonFillColorBlue(main),
                PrimitiveValue => MainShapesPolygonFillColorGreenEnd(main),
                _ => Err,
            },
            MainShapesPolygonFillColorGreenEnd(main) => match token {
                AttributBlue => MainShapesPolygonFillColorBlue(main),
                _ => Err,
            },
            MainShapesPolygonFillColorBlue(main) => match token {
                StructEnd => MainShapesPolygonFillColorEnd(main),
                PrimitiveValue => MainShapesPolygonFillColorBlueEnd(main),
                _ => Err,
            },
            MainShapesPolygonFillColorBlueEnd(main) => match token {
                StructEnd => MainShapesPolygonFillColorEnd(main),
                _ => Err,
            },
            MainShapesPolygonFillColorEnd(main) => match token {
                AttributVertices => MainShapesPolygonVertices(main),
                _ => Err,
            },
            MainShapesPolygonVertices(main) => match token {
                StructEnd => MainShapesStart(main),
                ArrayStart => MainShapesPolygonVerticesStart(main),
                _ => Err,
            },
            MainShapesPolygonVerticesStart(main) => match token {
                ArrayEnd => MainShapesPolygonVerticesEnd(main),
                StructStart => MainShapesPolygonVerticesVertexStart(main),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexStart(main) => match token {
                AttributX => MainShapesPolygonVerticesVertexX(main),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexX(main) => match token {
                AttributY => MainShapesPolygonVerticesVertexY(main),
                PrimitiveValue => MainShapesPolygonVerticesVertexXEnd(main),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexXEnd(main) => match token {
                AttributY => MainShapesPolygonVerticesVertexY(main),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexY(main) => match token {
                StructEnd => MainShapesPolygonVerticesStart(main),
                PrimitiveValue => MainShapesPolygonVerticesVertexYEnd(main),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexYEnd(main) => match token {
                StructEnd => MainShapesPolygonVerticesStart(main),
                _ => Err,
            },
            MainShapesPolygonVerticesEnd(main) => match token {
                StructEnd => MainShapesStart(main),
                _ => Err,
            },
            MainShapesGroupShapes(main) => match token {
                StructEnd => MainShapesStart(main),
                ArrayStart => MainShapesGroupShapesStart(main),
                _ => Err,
            },
            MainShapesGroupShapesStart(main) => match token {
                ArrayEnd => MainShapesGroupShapesEnd(main),
                StructStart => MainShapesGroupShapesPolygonStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonStart(main) => match token {
                AttributPosition => MainShapesGroupShapesPolygonPosition(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPosition(main) => match token {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main),
                StructStart => MainShapesGroupShapesPolygonPositionStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionStart(main) => match token {
                AttributX => MainShapesGroupShapesPolygonPositionX(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionX(main) => match token {
                AttributY => MainShapesGroupShapesPolygonPositionY(main),
                PrimitiveValue => MainShapesGroupShapesPolygonPositionXEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionXEnd(main) => match token {
                AttributY => MainShapesGroupShapesPolygonPositionY(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionY(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main),
                PrimitiveValue => MainShapesGroupShapesPolygonPositionYEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionYEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionEnd(main) => match token {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonRotation(main) => match token {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main),
                PrimitiveValue => MainShapesGroupShapesPolygonRotationEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonRotationEnd(main) => match token {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonWidth(main) => match token {
                AttributBorderColor => MainShapesGroupShapesPolygonBorderColor(main),
                PrimitiveValue => MainShapesGroupShapesPolygonWidthEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonWidthEnd(main) => match token {
                AttributBorderColor => MainShapesGroupShapesPolygonBorderColor(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColor(main) => match token {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main),
                StructStart => MainShapesGroupShapesPolygonBorderColorStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorStart(main) => match token {
                AttributRed => MainShapesGroupShapesPolygonBorderColorRed(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorRed(main) => match token {
                AttributGreen => MainShapesGroupShapesPolygonBorderColorGreen(main),
                PrimitiveValue => MainShapesGroupShapesPolygonBorderColorRedEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorRedEnd(main) => match token {
                AttributGreen => MainShapesGroupShapesPolygonBorderColorGreen(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorGreen(main) => match token {
                AttributBlue => MainShapesGroupShapesPolygonBorderColorBlue(main),
                PrimitiveValue => MainShapesGroupShapesPolygonBorderColorGreenEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorGreenEnd(main) => match token {
                AttributBlue => MainShapesGroupShapesPolygonBorderColorBlue(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorBlue(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main),
                PrimitiveValue => MainShapesGroupShapesPolygonBorderColorBlueEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorBlueEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorEnd(main) => match token {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColor(main) => match token {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main),
                StructStart => MainShapesGroupShapesPolygonFillColorStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorStart(main) => match token {
                AttributRed => MainShapesGroupShapesPolygonFillColorRed(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorRed(main) => match token {
                AttributGreen => MainShapesGroupShapesPolygonFillColorGreen(main),
                PrimitiveValue => MainShapesGroupShapesPolygonFillColorRedEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorRedEnd(main) => match token {
                AttributGreen => MainShapesGroupShapesPolygonFillColorGreen(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorGreen(main) => match token {
                AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main),
                PrimitiveValue => MainShapesGroupShapesPolygonFillColorGreenEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorGreenEnd(main) => match token {
                AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorBlue(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main),
                PrimitiveValue => MainShapesGroupShapesPolygonFillColorBlueEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorBlueEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorEnd(main) => match token {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVertices(main) => match token {
                StructEnd => MainShapesStart(main),
                ArrayStart => MainShapesGroupShapesPolygonVerticesStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesStart(main) => match token {
                ArrayEnd => MainShapesGroupShapesPolygonVerticesEnd(main),
                StructStart => MainShapesGroupShapesPolygonVerticesVertexStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexStart(main) => match token {
                AttributX => MainShapesGroupShapesPolygonVerticesVertexX(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexX(main) => match token {
                AttributY => MainShapesGroupShapesPolygonVerticesVertexY(main),
                PrimitiveValue => MainShapesGroupShapesPolygonVerticesVertexXEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexXEnd(main) => match token {
                AttributY => MainShapesGroupShapesPolygonVerticesVertexY(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexY(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonVerticesStart(main),
                PrimitiveValue => MainShapesGroupShapesPolygonVerticesVertexYEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexYEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonVerticesVertexEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonVerticesStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesEnd(main) => match token {
                StructEnd => MainShapesGroupShapesStart(main),
                _ => Err,
            },
            MainShapesGroupShapesPolygonEnd(main) => match token {
                StructEnd => MainShapesGroupShapesPolygonEnd(main),
                _ => Err,
            },
            MainShapesGroupShapesEnd(main) => match token {
                StructEnd => MainShapesStart(main),
                _ => Err,
            },
            MainShapesEnd(main) => match token {
                StructEnd => MainEnd(main),
                _ => Err,
            },
            MainEnd(main) => match token {
                _ => Err,
            },
        }
    }
}
