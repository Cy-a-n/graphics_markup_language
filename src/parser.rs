use self::ParserState::*;
use crate::draw_elements::*;
use crate::token::Token;
use crate::token::Token::*;

pub fn to_polygons(tokens: Vec<Token>) -> Vec<Vec<(i16, i16)>> {
    let mut tokens = tokens.into_iter();
    let state = Start;

    vec![vec![(0, 0)]]
}

#[derive(Debug)]
enum ParserState {
    Start,
    MainStart,
    MainVisibleExtent,
    MainVisibleExtentStart,
    MainVisibleExtentX,
    MainVisibleExtentXEnd,
    MainVisibleExtentY,
    MainVisibleExtentYEnd,
    MainVisibleExtentEnd,
    MainBackgroundColor,
    MainBackgroundColorStart,
    MainBackgroundColorRed,
    MainBackgroundColorRedEnd,
    MainBackgroundColorGreen,
    MainBackgroundColorGreenEnd,
    MainBackgroundColorBlue,
    MainBackgroundColorBlueEnd,
    MainBackgroundColorEnd,
    MainShapes,
    MainShapesStart,
    MainShapesElementStart,
    MainShapesElementPosition,
    MainShapesElementPositionStart,
    MainShapesElementPositionX,
    MainShapesElementPositionXEnd,
    MainShapesElementPositionY,
    MainShapesElementPositionYEnd,
    MainShapesElementPositionEnd,
    MainShapesElementRotation,
    MainShapesElementRotationEnd,
    MainShapesPolygonWidth,
    MainShapesPolygonWidthEnd,
    MainShapesPolygonBorderColor,
    MainShapesPolygonBorderColorStart,
    MainShapesPolygonBorderColorRed,
    MainShapesPolygonBorderColorRedEnd,
    MainShapesPolygonBorderColorGreen,
    MainShapesPolygonBorderColorGreenEnd,
    MainShapesPolygonBorderColorBlue,
    MainShapesPolygonBorderColorBlueEnd,
    MainShapesPolygonBorderColorEnd,
    MainShapesPolygonFillColor,
    MainShapesPolygonFillColorStart,
    MainShapesPolygonFillColorRed,
    MainShapesPolygonFillColorRedEnd,
    MainShapesPolygonFillColorGreen,
    MainShapesPolygonFillColorGreenEnd,
    MainShapesPolygonFillColorBlue,
    MainShapesPolygonFillColorBlueEnd,
    MainShapesPolygonFillColorEnd,
    MainShapesPolygonVertices,
    MainShapesPolygonVerticesStart,
    MainShapesPolygonVerticesVertexStart,
    MainShapesPolygonVerticesVertexX,
    MainShapesPolygonVerticesVertexXEnd,
    MainShapesPolygonVerticesVertexY,
    MainShapesPolygonVerticesVertexYEnd,
    MainShapesPolygonVerticesEnd,
    MainShapesGroupShapes,
    MainShapesGroupShapesStart,
    MainShapesGroupShapesPolygonStart,
    MainShapesGroupShapesPolygonPosition,
    MainShapesGroupShapesPolygonPositionStart,
    MainShapesGroupShapesPolygonPositionX,
    MainShapesGroupShapesPolygonPositionXEnd,
    MainShapesGroupShapesPolygonPositionY,
    MainShapesGroupShapesPolygonPositionYEnd,
    MainShapesGroupShapesPolygonPositionEnd,
    MainShapesGroupShapesPolygonRotation,
    MainShapesGroupShapesPolygonRotationEnd,
    MainShapesGroupShapesPolygonWidth,
    MainShapesGroupShapesPolygonWidthEnd,
    MainShapesGroupShapesPolygonBorderColor,
    MainShapesGroupShapesPolygonBorderColorStart,
    MainShapesGroupShapesPolygonBorderColorRed,
    MainShapesGroupShapesPolygonBorderColorRedEnd,
    MainShapesGroupShapesPolygonBorderColorGreen,
    MainShapesGroupShapesPolygonBorderColorGreenEnd,
    MainShapesGroupShapesPolygonBorderColorBlue,
    MainShapesGroupShapesPolygonBorderColorBlueEnd,
    MainShapesGroupShapesPolygonBorderColorEnd,
    MainShapesGroupShapesPolygonFillColor,
    MainShapesGroupShapesPolygonFillColorStart,
    MainShapesGroupShapesPolygonFillColorRed,
    MainShapesGroupShapesPolygonFillColorRedEnd,
    MainShapesGroupShapesPolygonFillColorGreen,
    MainShapesGroupShapesPolygonFillColorGreenEnd,
    MainShapesGroupShapesPolygonFillColorBlue,
    MainShapesGroupShapesPolygonFillColorBlueEnd,
    MainShapesGroupShapesPolygonFillColorEnd,
    MainShapesGroupShapesPolygonVertices,
    MainShapesGroupShapesPolygonVerticesStart,
    MainShapesGroupShapesPolygonVerticesVertexStart,
    MainShapesGroupShapesPolygonVerticesVertexX,
    MainShapesGroupShapesPolygonVerticesVertexXEnd,
    MainShapesGroupShapesPolygonVerticesVertexY,
    MainShapesGroupShapesPolygonVerticesVertexYEnd,
    MainShapesGroupShapesPolygonVerticesVertexEnd,
    MainShapesGroupShapesPolygonVerticesEnd,
    MainShapesGroupShapesPolygonEnd,
    MainShapesGroupShapesEnd,
    MainShapesEnd,
    MainEnd,
    Err,
}

impl ParserState {
    fn next_state(&self, input: Token) -> (Self,) {
        match self {
            Err => panic!("The `next_state`-method should never be called on ParserState::Err"),
            Start => match input {
                StructStart => (MainStart, None),
                _ => (Err, None),
            },
            MainStart => match input {
                AttributVisibleExtent => (MainVisibleExtent, None),
                _ => (Err, None),
            },
            MainVisibleExtent => match input {
                AttributBackgroundColor => (MainBackgroundColor, Some(MainVisibleExtentDefault)),
                StructStart => (MainVisibleExtentStart, None),
                _ => (Err, None),
            },
            MainVisibleExtentStart => match input {
                AttributX => (MainVisibleExtentX, None),
                _ => (Err, None),
            },
            MainVisibleExtentX => match input {
                AttributY => (MainVisibleExtentY, Some(MainVisibleExtentXDefault)),
                PrimitiveValue => (MainVisibleExtentXEnd, Some(ParseMainVisibleExtentX)),
                _ => (Err, None),
            },
            MainVisibleExtentXEnd => match input {
                AttributY => (MainVisibleExtentY, None),
                _ => (Err, None),
            },
            MainVisibleExtentY => match input {
                StructEnd => (MainVisibleExtentEnd, Some(MainVisibleExtentYDefault)),
                PrimitiveValue => (MainVisibleExtentYEnd, Some(ParseMainVisibleExtentY)),
                _ => (Err, None),
            },
            MainVisibleExtentYEnd => match input {
                StructEnd => (MainVisibleExtentEnd, None),
                _ => (Err, None),
            },
            MainVisibleExtentEnd => match input {
                AttributBackgroundColor => (MainBackgroundColor, None),
                _ => (Err, None),
            },
            MainBackgroundColor => match input {
                AttributShapes => (MainShapes, Some(MainBackgroundColorDefault)),
                StructStart => (MainBackgroundColorStart, None),
                _ => (Err, None),
            },
            MainBackgroundColorStart => match input {
                AttributRed => (MainBackgroundColorRed, None),
                _ => (Err, None),
            },
            MainBackgroundColorRed => match input {
                AttributGreen => (
                    MainBackgroundColorGreen,
                    Some(MainBackgroundColorRedDefault),
                ),
                PrimitiveValue => (MainBackgroundColorRedEnd, Some(ParseMainBackgroundColorRed)),
                _ => (Err, None),
            },
            MainBackgroundColorRedEnd => match input {
                AttributGreen => (MainBackgroundColorGreen, None),
                _ => (Err, None),
            },
            MainBackgroundColorGreen => match input {
                AttributBlue => (
                    MainBackgroundColorBlue,
                    Some(MainBackgroundColorGreenDefault),
                ),
                PrimitiveValue => (
                    MainBackgroundColorGreenEnd,
                    Some(ParseMainBackgroundColorGreen),
                ),
                _ => (Err, None),
            },
            MainBackgroundColorGreenEnd => match input {
                AttributBlue => (MainBackgroundColorBlue, None),
                _ => (Err, None),
            },
            MainBackgroundColorBlue => match input {
                StructEnd => (MainBackgroundColorEnd, Some(MainBackgroundColorBlueDefault)),
                PrimitiveValue => (
                    MainBackgroundColorBlueEnd,
                    Some(ParseMainBackgroundColorBlue),
                ),
                _ => (Err, None),
            },
            MainBackgroundColorBlueEnd => match input {
                StructEnd => (MainBackgroundColorEnd, None),
                _ => (Err, None),
            },
            MainBackgroundColorEnd => match input {
                AttributShapes => (MainShapes, None),
                _ => (Err, None),
            },
            MainShapes => match input {
                StructEnd => (MainEnd, Some(MainShapesDefault)),
                ArrayStart => (MainShapesStart, None),
                _ => (Err, None),
            },
            MainShapesStart => match input {
                ArrayEnd => (MainShapesEnd, None),
                StructStart => (MainShapesElementStart, None),
                _ => (Err, None),
            },
            MainShapesElementStart => match input {
                AttributPosition => (MainShapesElementPosition, None),
                _ => (Err, None),
            },
            MainShapesElementPosition => match input {
                AttributRotation => (
                    MainShapesElementRotation,
                    Some(MainShapesElementPositionDefault),
                ),
                StructStart => (MainShapesElementPositionStart, None),
                _ => (Err, None),
            },
            MainShapesElementPositionStart => match input {
                AttributX => (MainShapesElementPositionX, None),
                _ => (Err, None),
            },
            MainShapesElementPositionX => match input {
                AttributY => (
                    MainShapesElementPositionY,
                    Some(MainShapesElementPositionXDefault),
                ),
                PrimitiveValue => (
                    MainShapesElementPositionXEnd,
                    Some(ParseMainShapesElementPositionX),
                ),
                _ => (Err, None),
            },
            MainShapesElementPositionXEnd => match input {
                AttributY => (MainShapesElementPositionY, None),
                _ => (Err, None),
            },
            MainShapesElementPositionY => match input {
                StructEnd => (
                    MainShapesElementPositionEnd,
                    Some(MainShapesElementPositionYDefault),
                ),
                PrimitiveValue => (
                    MainShapesElementPositionYEnd,
                    Some(ParseMainShapesElementPositionY),
                ),
                _ => (Err, None),
            },
            MainShapesElementPositionYEnd => match input {
                StructEnd => (MainShapesElementPositionEnd, None),
                _ => (Err, None),
            },
            MainShapesElementPositionEnd => match input {
                AttributRotation => (MainShapesElementRotation, None),
                _ => (Err, None),
            },
            MainShapesElementRotation => match input {
                AttributWidth => (
                    MainShapesPolygonWidth,
                    Some(MainShapesElementRotationDefault),
                ),
                AttributShapes => (
                    MainShapesGroupShapes,
                    Some(MainShapesElementRotationDefault),
                ),
                PrimitiveValue => (
                    MainShapesElementRotationEnd,
                    Some(ParseMainShapesElementRotation),
                ),
                _ => (Err, None),
            },
            MainShapesElementRotationEnd => match input {
                AttributWidth => (MainShapesPolygonWidth, None),
                AttributShapes => (MainShapesGroupShapes, None),
                _ => (Err, None),
            },
            MainShapesPolygonWidth => match input {
                AttributBorderColor => (
                    MainShapesPolygonBorderColor,
                    Some(MainShapesPolygonWidthDefault),
                ),
                PrimitiveValue => (MainShapesPolygonWidthEnd, Some(ParseMainShapesPolygonWidth)),
                _ => (Err, None),
            },
            MainShapesPolygonWidthEnd => match input {
                AttributBorderColor => (MainShapesPolygonBorderColor, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColor => match input {
                AttributFillColor => (
                    MainShapesPolygonFillColor,
                    Some(MainShapesPolygonBorderColorDefault),
                ),
                StructStart => (MainShapesPolygonBorderColorStart, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorStart => match input {
                AttributRed => (MainShapesPolygonBorderColorRed, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorRed => match input {
                AttributGreen => (
                    MainShapesPolygonBorderColorGreen,
                    Some(MainShapesPolygonBorderColorRedDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonBorderColorRedEnd,
                    Some(ParseMainShapesPolygonBorderColorRed),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorRedEnd => match input {
                AttributGreen => (MainShapesPolygonBorderColorGreen, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorGreen => match input {
                AttributBlue => (
                    MainShapesPolygonBorderColorBlue,
                    Some(MainShapesPolygonBorderColorGreenDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonBorderColorGreenEnd,
                    Some(ParseMainShapesPolygonBorderColorGreen),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorGreenEnd => match input {
                AttributBlue => (MainShapesPolygonBorderColorBlue, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorBlue => match input {
                StructEnd => (
                    MainShapesPolygonBorderColorEnd,
                    Some(MainShapesPolygonBorderColorBlueDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonBorderColorBlueEnd,
                    Some(ParseMainShapesPolygonBorderColorBlue),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorBlueEnd => match input {
                StructEnd => (MainShapesPolygonBorderColorEnd, None),
                _ => (Err, None),
            },
            MainShapesPolygonBorderColorEnd => match input {
                AttributFillColor => (MainShapesPolygonFillColor, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColor => match input {
                AttributVertices => (
                    MainShapesPolygonVertices,
                    Some(MainShapesPolygonFillColorDefault),
                ),
                StructStart => (MainShapesPolygonFillColorStart, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorStart => match input {
                AttributRed => (MainShapesPolygonFillColorRed, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorRed => match input {
                AttributGreen => (
                    MainShapesPolygonFillColorGreen,
                    Some(MainShapesPolygonFillColorRedDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonFillColorRedEnd,
                    Some(ParseMainShapesPolygonFillColorRed),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorRedEnd => match input {
                AttributGreen => (MainShapesPolygonFillColorGreen, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorGreen => match input {
                AttributBlue => (
                    MainShapesPolygonFillColorBlue,
                    Some(MainShapesPolygonFillColorGreenDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonFillColorGreenEnd,
                    Some(ParseMainShapesPolygonFillColorGreen),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorGreenEnd => match input {
                AttributBlue => (MainShapesPolygonFillColorBlue, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorBlue => match input {
                StructEnd => (
                    MainShapesPolygonFillColorEnd,
                    Some(MainShapesPolygonFillColorBlueDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonFillColorBlueEnd,
                    Some(ParseMainShapesPolygonFillColorBlue),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorBlueEnd => match input {
                StructEnd => (MainShapesPolygonFillColorEnd, None),
                _ => (Err, None),
            },
            MainShapesPolygonFillColorEnd => match input {
                AttributVertices => (MainShapesPolygonVertices, None),
                _ => (Err, None),
            },
            MainShapesPolygonVertices => match input {
                StructEnd => (MainShapesStart, Some(MainShapesPolygonVerticesDefault)),
                ArrayStart => (MainShapesPolygonVerticesStart, None),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesStart => match input {
                ArrayEnd => (MainShapesPolygonVerticesEnd, None),
                StructStart => (MainShapesPolygonVerticesVertexStart, None),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesVertexStart => match input {
                AttributX => (MainShapesPolygonVerticesVertexX, None),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesVertexX => match input {
                AttributY => (
                    MainShapesPolygonVerticesVertexY,
                    Some(MainShapesPolygonVerticesVertexXDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonVerticesVertexXEnd,
                    Some(ParseMainShapesPolygonVerticesVertexX),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesVertexXEnd => match input {
                AttributY => (MainShapesPolygonVerticesVertexY, None),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesVertexY => match input {
                StructEnd => (
                    MainShapesPolygonVerticesStart,
                    Some(MainShapesPolygonVerticesVertexYDefault),
                ),
                PrimitiveValue => (
                    MainShapesPolygonVerticesVertexYEnd,
                    Some(ParseMainShapesPolygonVerticesVertexY),
                ),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesVertexYEnd => match input {
                StructEnd => (MainShapesPolygonVerticesStart, None),
                _ => (Err, None),
            },
            MainShapesPolygonVerticesEnd => match input {
                StructEnd => (MainShapesStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapes => match input {
                StructEnd => (MainShapesStart, Some(MainShapesGroupShapesDefault)),
                ArrayStart => (MainShapesGroupShapesStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesStart => match input {
                ArrayEnd => (MainShapesGroupShapesEnd, None),
                StructStart => (MainShapesGroupShapesPolygonStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonStart => match input {
                AttributPosition => (MainShapesGroupShapesPolygonPosition, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPosition => match input {
                AttributRotation => (
                    MainShapesGroupShapesPolygonRotation,
                    Some(MainShapesGroupShapesPolygonPositionDefault),
                ),
                StructStart => (MainShapesGroupShapesPolygonPositionStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionStart => match input {
                AttributX => (MainShapesGroupShapesPolygonPositionX, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionX => match input {
                AttributY => (
                    MainShapesGroupShapesPolygonPositionY,
                    Some(MainShapesGroupShapesPolygonPositionXDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonPositionXEnd,
                    Some(ParseMainShapesGroupShapesPolygonPositionX),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionXEnd => match input {
                AttributY => (MainShapesGroupShapesPolygonPositionY, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionY => match input {
                StructEnd => (
                    MainShapesGroupShapesPolygonPositionEnd,
                    Some(MainShapesGroupShapesPolygonPositionYDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonPositionYEnd,
                    Some(ParseMainShapesGroupShapesPolygonPositionY),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionYEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonPositionEnd, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonPositionEnd => match input {
                AttributRotation => (MainShapesGroupShapesPolygonRotation, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonRotation => match input {
                AttributWidth => (
                    MainShapesGroupShapesPolygonWidth,
                    Some(MainShapesGroupShapesPolygonRotationDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonRotationEnd,
                    Some(ParseMainShapesGroupShapesPolygonRotation),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonRotationEnd => match input {
                AttributWidth => (MainShapesGroupShapesPolygonWidth, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonWidth => match input {
                AttributBorderColor => (
                    MainShapesGroupShapesPolygonBorderColor,
                    Some(MainShapesGroupShapesPolygonWidthDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonWidthEnd,
                    Some(ParseMainShapesGroupShapesPolygonWidth),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonWidthEnd => match input {
                AttributBorderColor => (MainShapesGroupShapesPolygonBorderColor, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColor => match input {
                AttributFillColor => (
                    MainShapesGroupShapesPolygonFillColor,
                    Some(MainShapesGroupShapesPolygonBorderColorDefault),
                ),
                StructStart => (MainShapesGroupShapesPolygonBorderColorStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorStart => match input {
                AttributRed => (MainShapesGroupShapesPolygonBorderColorRed, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorRed => match input {
                AttributGreen => (
                    MainShapesGroupShapesPolygonBorderColorGreen,
                    Some(MainShapesGroupShapesPolygonBorderColorRedDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonBorderColorRedEnd,
                    Some(ParseMainShapesGroupShapesPolygonBorderColorRed),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorRedEnd => match input {
                AttributGreen => (MainShapesGroupShapesPolygonBorderColorGreen, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorGreen => match input {
                AttributBlue => (
                    MainShapesGroupShapesPolygonBorderColorBlue,
                    Some(MainShapesGroupShapesPolygonBorderColorGreenDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonBorderColorGreenEnd,
                    Some(ParseMainShapesGroupShapesPolygonBorderColorGreen),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorGreenEnd => match input {
                AttributBlue => (MainShapesGroupShapesPolygonBorderColorBlue, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorBlue => match input {
                StructEnd => (
                    MainShapesGroupShapesPolygonBorderColorEnd,
                    Some(MainShapesGroupShapesPolygonBorderColorBlueDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonBorderColorBlueEnd,
                    Some(ParseMainShapesGroupShapesPolygonBorderColorBlue),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorBlueEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonBorderColorEnd, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonBorderColorEnd => match input {
                AttributFillColor => (MainShapesGroupShapesPolygonFillColor, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColor => match input {
                AttributVertices => (
                    MainShapesGroupShapesPolygonVertices,
                    Some(MainShapesGroupShapesPolygonFillColorDefault),
                ),
                StructStart => (MainShapesGroupShapesPolygonFillColorStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorStart => match input {
                AttributRed => (MainShapesGroupShapesPolygonFillColorRed, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorRed => match input {
                AttributGreen => (
                    MainShapesGroupShapesPolygonFillColorGreen,
                    Some(MainShapesGroupShapesPolygonFillColorRedDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonFillColorRedEnd,
                    Some(ParseMainShapesGroupShapesPolygonFillColorRed),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorRedEnd => match input {
                AttributGreen => (MainShapesGroupShapesPolygonFillColorGreen, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorGreen => match input {
                AttributBlue => (
                    MainShapesGroupShapesPolygonFillColorBlue,
                    Some(MainShapesGroupShapesPolygonFillColorGreenDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonFillColorGreenEnd,
                    Some(ParseMainShapesGroupShapesPolygonFillColorGreen),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorGreenEnd => match input {
                AttributBlue => (MainShapesGroupShapesPolygonFillColorBlue, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorBlue => match input {
                StructEnd => (
                    MainShapesGroupShapesPolygonFillColorEnd,
                    Some(MainShapesGroupShapesPolygonFillColorBlueDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonFillColorBlueEnd,
                    Some(ParseMainShapesGroupShapesPolygonFillColorBlue),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorBlueEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonFillColorEnd, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonFillColorEnd => match input {
                AttributVertices => (MainShapesGroupShapesPolygonVertices, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVertices => match input {
                StructEnd => (
                    MainShapesStart,
                    Some(MainShapesGroupShapesPolygonVerticesDefault),
                ),
                ArrayStart => (MainShapesGroupShapesPolygonVerticesStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesStart => match input {
                ArrayEnd => (MainShapesGroupShapesPolygonVerticesEnd, None),
                StructStart => (MainShapesGroupShapesPolygonVerticesVertexStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexStart => match input {
                AttributX => (MainShapesGroupShapesPolygonVerticesVertexX, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexX => match input {
                AttributY => (
                    MainShapesGroupShapesPolygonVerticesVertexY,
                    Some(MainShapesGroupShapesPolygonVerticesVertexXDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonVerticesVertexXEnd,
                    Some(ParseMainShapesGroupShapesPolygonVerticesVertexX),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexXEnd => match input {
                AttributY => (MainShapesGroupShapesPolygonVerticesVertexY, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexY => match input {
                StructEnd => (
                    MainShapesGroupShapesPolygonVerticesStart,
                    Some(MainShapesGroupShapesPolygonVerticesVertexYDefault),
                ),
                PrimitiveValue => (
                    MainShapesGroupShapesPolygonVerticesVertexYEnd,
                    Some(ParseMainShapesGroupShapesPolygonVerticesVertexY),
                ),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexYEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonVerticesVertexEnd, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesVertexEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonVerticesStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonVerticesEnd => match input {
                StructEnd => (MainShapesGroupShapesStart, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesPolygonEnd => match input {
                StructEnd => (MainShapesGroupShapesPolygonEnd, None),
                _ => (Err, None),
            },
            MainShapesGroupShapesEnd => match input {
                StructEnd => (MainShapesStart, None),
                _ => (Err, None),
            },
            MainShapesEnd => match input {
                StructEnd => (MainEnd, None),
                _ => (Err, None),
            },
            MainEnd => match input {
                _ => (Err, None),
            },
        }
    }
}
