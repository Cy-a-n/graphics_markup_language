mod i16_parser;
mod u8_parser;

use std::iter::Enumerate;
use std::iter::Peekable;
use std::vec::IntoIter;

use self::ParserState::*;
use crate::draw_elements::*;
use crate::token::Token;
use crate::token::TokenValue::*;
use i16_parser::parse_i16;
use u8_parser::parse_u8;

pub fn to_main_draw_element(tokens: Vec<Token>) -> Main {
    let mut tokens = tokens.into_iter().enumerate().peekable();
    let mut state = Start;

    loop {
        state = state.next_state(&mut tokens);

        if let MainEnd(main) = state {
            return main;
        }
    }
}

fn parser_panic(token: &Token, index: &usize) -> ! {
    panic!(
        "Encountered invalid token `{:?}` at index {}.",
        token, index
    )
}

#[derive(Debug)]
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
    MainShapesElementStart(Main, AmbiguousElement),
    MainShapesElementPosition(Main, AmbiguousElement),
    MainShapesElementPositionStart(Main, AmbiguousElement),
    MainShapesElementPositionX(Main, AmbiguousElement),
    MainShapesElementPositionXEnd(Main, AmbiguousElement),
    MainShapesElementPositionY(Main, AmbiguousElement),
    MainShapesElementPositionYEnd(Main, AmbiguousElement),
    MainShapesElementPositionEnd(Main, AmbiguousElement),
    MainShapesElementRotation(Main, AmbiguousElement),
    MainShapesElementRotationEnd(Main, AmbiguousElement),
    MainShapesPolygonWidth(Main, Polygon),
    MainShapesPolygonWidthEnd(Main, Polygon),
    MainShapesPolygonBorderColor(Main, Polygon),
    MainShapesPolygonBorderColorStart(Main, Polygon),
    MainShapesPolygonBorderColorRed(Main, Polygon),
    MainShapesPolygonBorderColorRedEnd(Main, Polygon),
    MainShapesPolygonBorderColorGreen(Main, Polygon),
    MainShapesPolygonBorderColorGreenEnd(Main, Polygon),
    MainShapesPolygonBorderColorBlue(Main, Polygon),
    MainShapesPolygonBorderColorBlueEnd(Main, Polygon),
    MainShapesPolygonBorderColorEnd(Main, Polygon),
    MainShapesPolygonFillColor(Main, Polygon),
    MainShapesPolygonFillColorStart(Main, Polygon),
    MainShapesPolygonFillColorRed(Main, Polygon),
    MainShapesPolygonFillColorRedEnd(Main, Polygon),
    MainShapesPolygonFillColorGreen(Main, Polygon),
    MainShapesPolygonFillColorGreenEnd(Main, Polygon),
    MainShapesPolygonFillColorBlue(Main, Polygon),
    MainShapesPolygonFillColorBlueEnd(Main, Polygon),
    MainShapesPolygonFillColorEnd(Main, Polygon),
    MainShapesPolygonVertices(Main, Polygon),
    MainShapesPolygonVerticesStart(Main, Polygon),
    MainShapesPolygonVerticesVertexStart(Main, Polygon),
    MainShapesPolygonVerticesVertexX(Main, Polygon, Point),
    MainShapesPolygonVerticesVertexXEnd(Main, Polygon, Point),
    MainShapesPolygonVerticesVertexY(Main, Polygon, Point),
    MainShapesPolygonVerticesVertexYEnd(Main, Polygon, Point),
    MainShapesPolygonVerticesEnd(Main, Polygon),
    MainShapesGroupShapes(Main, Group),
    MainShapesGroupShapesStart(Main, Group),
    MainShapesGroupShapesPolygonStart(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPosition(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionStart(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionX(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionXEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionY(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionYEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonPositionEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonRotation(Main, Group, Polygon),
    MainShapesGroupShapesPolygonRotationEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonWidth(Main, Group, Polygon),
    MainShapesGroupShapesPolygonWidthEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColor(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorStart(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorRed(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorRedEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorGreen(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorGreenEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorBlue(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorBlueEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonBorderColorEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColor(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorStart(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorRed(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorRedEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorGreen(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorGreenEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorBlue(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorBlueEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonFillColorEnd(Main, Group, Polygon),
    MainShapesGroupShapesPolygonVertices(Main, Group, Polygon),
    MainShapesGroupShapesPolygonVerticesStart(Main, Group, Polygon),
    MainShapesGroupShapesPolygonVerticesVertexStart(Main, Group, Polygon, Point),
    MainShapesGroupShapesPolygonVerticesVertexX(Main, Group, Polygon, Point),
    MainShapesGroupShapesPolygonVerticesVertexXEnd(Main, Group, Polygon, Point),
    MainShapesGroupShapesPolygonVerticesVertexY(Main, Group, Polygon, Point),
    MainShapesGroupShapesPolygonVerticesVertexYEnd(Main, Group, Polygon, Point),
    MainShapesGroupShapesPolygonVerticesEnd(Main, Group, Polygon),
    MainShapesGroupShapesEnd(Main, Group),
    MainShapesEnd(Main),
    MainEnd(Main),
}

impl ParserState {
    fn next_state(self, tokens: &mut Peekable<Enumerate<IntoIter<Token>>>) -> Self {
        let (index, token) = tokens.next().expect("The source code ended prematurely");
        let token_value = token.value();

        match self {
            Start => match token_value {
                StructStart => MainStart(Main::default()),
                _ => parser_panic(&token, &index),
            },
            MainStart(main) => match token_value {
                AttributVisibleExtent => MainVisibleExtent(main),
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtent(main) => match token_value {
                AttributBackgroundColor => MainBackgroundColor(main),
                StructStart => MainVisibleExtentStart(main),
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentStart(main) => match token_value {
                AttributX => MainVisibleExtentX(main),
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentX(mut main) => match token_value {
                AttributY => MainVisibleExtentY(main),
                PrimitiveValue => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainVisibleExtentXEnd(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentXEnd(main) => match token_value {
                AttributY => MainVisibleExtentY(main),
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentY(mut main) => match token_value {
                StructEnd => MainVisibleExtentEnd(main),
                PrimitiveValue => {
                    main.visible_extent.y = parse_i16(tokens);
                    MainVisibleExtentYEnd(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentYEnd(main) => match token_value {
                StructEnd => MainVisibleExtentEnd(main),
                _ => parser_panic(&token, &index),
            },
            MainVisibleExtentEnd(main) => match token_value {
                AttributBackgroundColor => MainBackgroundColor(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColor(main) => match token_value {
                AttributShapes => MainShapes(main),
                StructStart => MainBackgroundColorStart(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorStart(main) => match token_value {
                AttributRed => MainBackgroundColorRed(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorRed(mut main) => match token_value {
                AttributGreen => MainBackgroundColorGreen(main),
                PrimitiveValue => {
                    main.background_color.red = parse_u8(tokens);
                    MainBackgroundColorRedEnd(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorRedEnd(main) => match token_value {
                AttributGreen => MainBackgroundColorGreen(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorGreen(mut main) => match token_value {
                AttributBlue => MainBackgroundColorBlue(main),
                PrimitiveValue => {
                    main.background_color.green = parse_u8(tokens);
                    MainBackgroundColorGreenEnd(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorGreenEnd(main) => match token_value {
                AttributBlue => MainBackgroundColorBlue(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorBlue(mut main) => match token_value {
                StructEnd => MainBackgroundColorEnd(main),
                PrimitiveValue => {
                    main.background_color.blue = parse_u8(tokens);
                    MainBackgroundColorBlueEnd(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorBlueEnd(main) => match token_value {
                StructEnd => MainBackgroundColorEnd(main),
                _ => parser_panic(&token, &index),
            },
            MainBackgroundColorEnd(main) => match token_value {
                AttributShapes => MainShapes(main),
                _ => parser_panic(&token, &index),
            },
            MainShapes(main) => match token_value {
                StructEnd => MainEnd(main),
                ArrayStart => MainShapesStart(main),
                _ => parser_panic(&token, &index),
            },
            MainShapesStart(main) => match token_value {
                ArrayEnd => MainShapesEnd(main),
                StructStart => MainShapesElementStart(main, AmbiguousElement::default()),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementStart(main, ambiguous_element) => match token_value {
                AttributPosition => MainShapesElementPosition(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPosition(main, ambiguous_element) => match token_value {
                AttributRotation => MainShapesElementRotation(main, ambiguous_element),
                StructStart => MainShapesElementPositionStart(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionStart(main, ambiguous_element) => match token_value {
                AttributX => MainShapesElementPositionX(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionX(main, mut ambiguous_element) => match token_value {
                AttributY => MainShapesElementPositionY(main, ambiguous_element),
                PrimitiveValue => {
                    ambiguous_element.position.x = parse_i16(tokens);
                    MainShapesElementPositionXEnd(main, ambiguous_element)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionXEnd(main, ambiguous_element) => match token_value {
                AttributY => MainShapesElementPositionY(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionY(mut main, ambiguous_element) => match token_value {
                StructEnd => MainShapesElementPositionEnd(main, ambiguous_element),
                PrimitiveValue => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainShapesElementPositionYEnd(main, ambiguous_element)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionYEnd(main, ambiguous_element) => match token_value {
                StructEnd => MainShapesElementPositionEnd(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementPositionEnd(main, ambiguous_element) => match token_value {
                AttributRotation => MainShapesElementRotation(main, ambiguous_element),
                _ => parser_panic(&token, &index),
            },
            MainShapesElementRotation(main, mut ambiguous_element) => match token_value {
                AttributWidth => MainShapesPolygonWidth(main, ambiguous_element.into_polygon()),
                AttributShapes => MainShapesGroupShapes(main, ambiguous_element.into_group()),
                PrimitiveValue => {
                    ambiguous_element.rotation = parse_u8(tokens);
                    MainShapesElementRotationEnd(main, ambiguous_element)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesElementRotationEnd(main, ambiguous_element) => match token_value {
                AttributWidth => MainShapesPolygonWidth(main, ambiguous_element.into_polygon()),
                AttributShapes => MainShapesGroupShapes(main, ambiguous_element.into_group()),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonWidth(main, mut polygon) => match token_value {
                AttributBorderColor => MainShapesPolygonBorderColor(main, polygon),
                PrimitiveValue => {
                    polygon.width = parse_i16(tokens);
                    MainShapesPolygonWidthEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonWidthEnd(main, polygon) => match token_value {
                AttributBorderColor => MainShapesPolygonBorderColor(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColor(main, polygon) => match token_value {
                AttributFillColor => MainShapesPolygonFillColor(main, polygon),
                StructStart => MainShapesPolygonBorderColorStart(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorStart(main, polygon) => match token_value {
                AttributRed => MainShapesPolygonBorderColorRed(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorRed(main, mut polygon) => match token_value {
                AttributGreen => MainShapesPolygonBorderColorGreen(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.red = parse_u8(tokens);
                    MainShapesPolygonBorderColorRedEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorRedEnd(main, polygon) => match token_value {
                AttributGreen => MainShapesPolygonBorderColorGreen(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorGreen(main, mut polygon) => match token_value {
                AttributBlue => MainShapesPolygonBorderColorBlue(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.green = parse_u8(tokens);
                    MainShapesPolygonBorderColorGreenEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorGreenEnd(main, polygon) => match token_value {
                AttributBlue => MainShapesPolygonBorderColorBlue(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorBlue(main, mut polygon) => match token_value {
                StructEnd => MainShapesPolygonBorderColorEnd(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.blue = parse_u8(tokens);
                    MainShapesPolygonBorderColorBlueEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorBlueEnd(main, polygon) => match token_value {
                StructEnd => MainShapesPolygonBorderColorEnd(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonBorderColorEnd(main, polygon) => match token_value {
                AttributFillColor => MainShapesPolygonFillColor(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColor(main, polygon) => match token_value {
                AttributVertices => MainShapesPolygonVertices(main, polygon),
                StructStart => MainShapesPolygonFillColorStart(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorStart(main, polygon) => match token_value {
                AttributRed => MainShapesPolygonFillColorRed(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorRed(main, mut polygon) => match token_value {
                AttributGreen => MainShapesPolygonFillColorGreen(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesPolygonFillColorRedEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorRedEnd(main, polygon) => match token_value {
                AttributGreen => MainShapesPolygonFillColorGreen(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorGreen(main, mut polygon) => match token_value {
                AttributBlue => MainShapesPolygonFillColorBlue(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.green = parse_u8(tokens);
                    MainShapesPolygonFillColorGreenEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorGreenEnd(main, polygon) => match token_value {
                AttributBlue => MainShapesPolygonFillColorBlue(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorBlue(main, mut polygon) => match token_value {
                StructEnd => MainShapesPolygonFillColorEnd(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.blue = parse_u8(tokens);
                    MainShapesPolygonFillColorBlueEnd(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorBlueEnd(main, polygon) => match token_value {
                StructEnd => MainShapesPolygonFillColorEnd(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonFillColorEnd(main, polygon) => match token_value {
                AttributVertices => MainShapesPolygonVertices(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVertices(mut main, polygon) => match token_value {
                StructEnd => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                ArrayStart => MainShapesPolygonVerticesStart(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesStart(main, polygon) => match token_value {
                ArrayEnd => MainShapesPolygonVerticesEnd(main, polygon),
                StructStart => MainShapesPolygonVerticesVertexStart(main, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesVertexStart(main, polygon) => match token_value {
                AttributX => MainShapesPolygonVerticesVertexX(main, polygon, Point::default()),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesVertexX(main, polygon, mut vertex) => match token_value {
                AttributY => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                PrimitiveValue => {
                    vertex.x = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex) => match token_value {
                AttributY => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesVertexY(main, mut polygon, mut vertex) => match token_value {
                StructEnd => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                PrimitiveValue => {
                    vertex.y = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexYEnd(main, polygon, vertex)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesVertexYEnd(main, mut polygon, vertex) => match token_value {
                StructEnd => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesPolygonVerticesEnd(mut main, polygon) => match token_value {
                StructEnd => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapes(mut main, group) => match token_value {
                StructEnd => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                } // TODO: Add default group
                ArrayStart => MainShapesGroupShapesStart(main, group),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesStart(main, group) => match token_value {
                ArrayEnd => MainShapesGroupShapesEnd(main, group),
                StructStart => MainShapesGroupShapesPolygonStart(main, group, Polygon::default()),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonStart(main, group, polygon) => match token_value {
                AttributPosition => MainShapesGroupShapesPolygonPosition(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPosition(main, group, polygon) => match token_value {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonPositionStart(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionStart(main, group, polygon) => match token_value {
                AttributX => MainShapesGroupShapesPolygonPositionX(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionX(main, group, mut polygon) => match token_value {
                AttributY => MainShapesGroupShapesPolygonPositionY(main, group, polygon),
                PrimitiveValue => {
                    polygon.position.x = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon) => match token_value {
                AttributY => MainShapesGroupShapesPolygonPositionY(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionY(main, group, mut polygon) => match token_value {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main, group, polygon),
                PrimitiveValue => {
                    polygon.position.y = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon) => match token_value {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonPositionEnd(main, group, polygon) => match token_value {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonRotation(main, group, mut polygon) => match token_value {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main, group, polygon),
                PrimitiveValue => {
                    polygon.rotation = parse_u8(tokens);
                    MainShapesGroupShapesPolygonRotationEnd(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonRotationEnd(main, group, polygon) => match token_value {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonWidth(main, group, mut polygon) => match token_value {
                AttributBorderColor => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                PrimitiveValue => {
                    polygon.width = parse_i16(tokens);
                    MainShapesGroupShapesPolygonWidthEnd(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonWidthEnd(main, group, polygon) => match token_value {
                AttributBorderColor => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonBorderColor(main, group, polygon) => match token_value {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon) => match token_value
            {
                AttributRed => MainShapesGroupShapesPolygonBorderColorRed(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonBorderColorRed(main, group, mut polygon) => {
                match token_value {
                    AttributGreen => {
                        MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon)
                    }
                    PrimitiveValue => {
                        polygon.border_color.red = parse_u8(tokens);
                        MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon) => {
                match token_value {
                    AttributGreen => {
                        MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorGreen(main, group, mut polygon) => {
                match token_value {
                    AttributBlue => {
                        MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon)
                    }
                    PrimitiveValue => {
                        polygon.border_color.green = parse_u8(tokens);
                        MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon) => {
                match token_value {
                    AttributBlue => {
                        MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorBlue(main, group, mut polygon) => {
                match token_value {
                    StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon),
                    PrimitiveValue => {
                        polygon.border_color.blue = parse_u8(tokens);
                        MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon) => {
                match token_value {
                    StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon),
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon) => match token_value {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonFillColor(main, group, polygon) => match token_value {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonFillColorStart(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonFillColorStart(main, group, polygon) => match token_value {
                AttributRed => MainShapesGroupShapesPolygonFillColorRed(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonFillColorRed(main, group, mut polygon) => match token_value
            {
                AttributGreen => MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon),
                PrimitiveValue => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon) => {
                match token_value {
                    AttributGreen => {
                        MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonFillColorGreen(main, group, mut polygon) => {
                match token_value {
                    AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon),
                    PrimitiveValue => {
                        polygon.fill_color.green = parse_u8(tokens);
                        MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon) => {
                match token_value {
                    AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon),
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonFillColorBlue(main, group, mut polygon) => {
                match token_value {
                    StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon),
                    PrimitiveValue => {
                        polygon.fill_color.blue = parse_u8(tokens);
                        MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon) => match token_value
            {
                StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon) => match token_value {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonVertices(main, mut group, polygon) => match token_value {
                StructEnd => {
                    group.shapes.push(polygon);
                    MainShapesGroupShapesStart(main, group)
                }
                ArrayStart => MainShapesGroupShapesPolygonVerticesStart(main, group, polygon),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonVerticesStart(main, group, polygon) => match token_value {
                ArrayEnd => MainShapesGroupShapesPolygonVerticesEnd(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonVerticesVertexStart(
                    main,
                    group,
                    polygon,
                    Point::default(),
                ),
                _ => parser_panic(&token, &index),
            },
            MainShapesGroupShapesPolygonVerticesVertexStart(main, group, polygon, vertex) => {
                match token_value {
                    AttributX => {
                        MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, vertex)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, mut vertex) => {
                match token_value {
                    AttributY => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    PrimitiveValue => {
                        vertex.x = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex) => {
                match token_value {
                    AttributY => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexY(main, group, mut polygon, mut vertex) => {
                match token_value {
                    StructEnd => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    PrimitiveValue => {
                        vertex.y = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, polygon, vertex)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, mut polygon, vertex) => {
                match token_value {
                    StructEnd => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesPolygonVerticesEnd(main, mut group, polygon) => {
                match token_value {
                    StructEnd => {
                        group.shapes.push(polygon);
                        MainShapesGroupShapesStart(main, group)
                    }
                    _ => panic!(
                        "Encountered invalid token  `{:?}` at index {}.",
                        token, index
                    ),
                }
            }
            MainShapesGroupShapesEnd(mut main, group) => match token_value {
                StructEnd => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                }
                _ => parser_panic(&token, &index),
            },
            MainShapesEnd(main) => match token_value {
                StructEnd => MainEnd(main),
                _ => parser_panic(&token, &index),
            },
            MainEnd(_) => match token_value {
                _ => parser_panic(&token, &index),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_tokens() {
        let tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>();

        let expected = Main::default();

        assert_eq!(to_main_draw_element(tokens), expected);
    }

    #[test]
    fn custom_visible_extent() {
        let tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            One,
            Zero,
            Zero,
            Zero,
            One,
            One,
            One,
            One,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            Zero,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>();
        let expected = Main {
            visible_extent: Point {
                x: 0b10001111,
                y: -0b001010101111111,
            },
            background_color: Color::default(),
            shapes: vec![],
        };

        assert_eq!(to_main_draw_element(tokens), expected);
    }

    #[test]
    fn custom_background_color() {
        let tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            AttributBlue,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributShapes,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>();
        let mut expected = Main::default();
        expected.background_color = Color {
            red: 0b11111111,
            green: 0b00000000,
            blue: 0,
        };

        assert_eq!(to_main_draw_element(tokens), expected);
    }

    #[test]
    fn custom_full() {
        let tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            Zero,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            AttributY,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativeValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativeValue,
            One,
            AttributY,
            PrimitiveValue,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativeValue,
            One,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativeValue,
            One,
            Zero,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            Zero,
            AttributGreen,
            PrimitiveValue,
            One,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributShapes,
            StructEnd,
            ArrayEnd,
            StructEnd,
        ]
        .into_iter()
        .map(|token_value| Token::default(token_value))
        .collect::<Vec<Token>>();
        let expected = Main {
            visible_extent: Point {
                x: -0b10,
                y: 0b11111,
            },
            background_color: Color {
                red: 1,
                green: 0,
                blue: 0b11,
            },
            shapes: vec![
                Shape::Polygon(AmbiguousElement::default().into_polygon()),
                Shape::Group(Group {
                    position: Point { x: 0, y: 0 },
                    rotation: 0,
                    shapes: vec![
                        Polygon {
                            position: Point { x: 0, y: 0 },
                            rotation: 0,
                            width: -0b111111111111111,
                            border_color: Color {
                                red: 1,
                                green: 0,
                                blue: 0b11,
                            },
                            fill_color: Color {
                                red: 0b11111111,
                                green: 0,
                                blue: 10,
                            },
                            vertices: vec![Point { x: -0b10, y: 0b10 }, Point { x: -1, y: 1 }],
                        },
                        AmbiguousElement::default().into_polygon(),
                    ],
                }),
                Shape::Polygon(Polygon {
                    position: Point { x: 0, y: -1 },
                    rotation: 1,
                    width: -0b10,
                    border_color: Color {
                        red: 1,
                        green: 0,
                        blue: 0b101,
                    },
                    fill_color: Color {
                        red: 0,
                        green: 1,
                        blue: 0b10,
                    },
                    vertices: vec![
                        Point {
                            x: 0b1010,
                            y: 0b10101,
                        },
                        Point {
                            x: 0b101010,
                            y: 0b1010101,
                        },
                    ],
                }),
                Shape::Group(AmbiguousElement::default().into_group()),
            ],
        };
        assert_eq!(to_main_draw_element(tokens), expected);
    }
}
