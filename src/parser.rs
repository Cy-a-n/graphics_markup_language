mod i16_parser;
mod u8_parser;

use std::iter::Peekable;
use std::vec::IntoIter;

use self::ParserState::*;
use crate::draw_elements::*;
use crate::token::Token;
use crate::token::Token::*;
use i16_parser::parse_i16;
use u8_parser::parse_u8;

pub fn to_main_draw_element(tokens: Vec<Token>) -> Main {
    let mut tokens: Peekable<IntoIter<Token>> = tokens.into_iter().peekable();
    let mut state = Start;

    loop {
        state = state.next_state(&mut tokens);

        if let MainEnd(main) = state {
            return main;
        }
    }
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
    Err(Token),
}

impl ParserState {
    fn next_state(self, tokens: &mut Peekable<IntoIter<Token>>) -> Self {
        let token = tokens.next().expect("The source code ended prematurely");
        match self {
            Err(previous_token) => {
                panic!(
                    "Encountered invalid token `{:?}`. Next token is {:?}",
                    previous_token, token
                )
            }
            Start => match token {
                StructStart(line, offset) => MainStart(Main::default()),
                _ => panic!(),
            },
            MainStart(main) => match token {
                AttributVisibleExtent(line, offset) => MainVisibleExtent(main),
                _ => Err(token),
            },
            MainVisibleExtent(main) => match token {
                AttributBackgroundColor(line, offset) => MainBackgroundColor(main),
                StructStart(line, offset) => MainVisibleExtentStart(main),
                _ => Err(token),
            },
            MainVisibleExtentStart(main) => match token {
                AttributX(line, offset) => MainVisibleExtentX(main),
                _ => Err(token),
            },
            MainVisibleExtentX(mut main) => match token {
                AttributY(line, offset) => MainVisibleExtentY(main),
                PrimitiveValue(line, offset) => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainVisibleExtentXEnd(main)
                }
                _ => Err(token),
            },
            MainVisibleExtentXEnd(main) => match token {
                AttributY(line, offset) => MainVisibleExtentY(main),
                _ => Err(token),
            },
            MainVisibleExtentY(mut main) => match token {
                StructEnd(line, offset) => MainVisibleExtentEnd(main),
                PrimitiveValue(line, offset) => {
                    main.visible_extent.y = parse_i16(tokens);
                    MainVisibleExtentYEnd(main)
                }
                _ => Err(token),
            },
            MainVisibleExtentYEnd(main) => match token {
                StructEnd(line, offset) => MainVisibleExtentEnd(main),
                _ => Err(token),
            },
            MainVisibleExtentEnd(main) => match token {
                AttributBackgroundColor(line, offset) => MainBackgroundColor(main),
                _ => Err(token),
            },
            MainBackgroundColor(main) => match token {
                AttributShapes(line, offset) => MainShapes(main),
                StructStart(line, offset) => MainBackgroundColorStart(main),
                _ => Err(token),
            },
            MainBackgroundColorStart(main) => match token {
                AttributRed(line, offset) => MainBackgroundColorRed(main),
                _ => Err(token),
            },
            MainBackgroundColorRed(mut main) => match token {
                AttributGreen(line, offset) => MainBackgroundColorGreen(main),
                PrimitiveValue(line, offset) => {
                    main.background_color.red = parse_u8(tokens);
                    MainBackgroundColorRedEnd(main)
                }
                _ => Err(token),
            },
            MainBackgroundColorRedEnd(main) => match token {
                AttributGreen(line, offset) => MainBackgroundColorGreen(main),
                _ => Err(token),
            },
            MainBackgroundColorGreen(mut main) => match token {
                AttributBlue(line, offset) => MainBackgroundColorBlue(main),
                PrimitiveValue(line, offset) => {
                    main.background_color.green = parse_u8(tokens);
                    MainBackgroundColorGreenEnd(main)
                }
                _ => Err(token),
            },
            MainBackgroundColorGreenEnd(main) => match token {
                AttributBlue(line, offset) => MainBackgroundColorBlue(main),
                _ => Err(token),
            },
            MainBackgroundColorBlue(mut main) => match token {
                StructEnd(line, offset) => MainBackgroundColorEnd(main),
                PrimitiveValue(line, offset) => {
                    main.background_color.blue = parse_u8(tokens);
                    MainBackgroundColorBlueEnd(main)
                }
                _ => Err(token),
            },
            MainBackgroundColorBlueEnd(main) => match token {
                StructEnd(line, offset) => MainBackgroundColorEnd(main),
                _ => Err(token),
            },
            MainBackgroundColorEnd(main) => match token {
                AttributShapes(line, offset) => MainShapes(main),
                _ => Err(token),
            },
            MainShapes(main) => match token {
                StructEnd(line, offset) => MainEnd(main),
                ArrayStart(line, offset) => MainShapesStart(main),
                _ => Err(token),
            },
            MainShapesStart(main) => match token {
                ArrayEnd(line, offset) => MainShapesEnd(main),
                StructStart(line, offset) => {
                    MainShapesElementStart(main, AmbiguousElement::default())
                }
                _ => Err(token),
            },
            MainShapesElementStart(main, ambiguous_element) => match token {
                AttributPosition(line, offset) => {
                    MainShapesElementPosition(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementPosition(main, ambiguous_element) => match token {
                AttributRotation(line, offset) => {
                    MainShapesElementRotation(main, ambiguous_element)
                }
                StructStart(line, offset) => {
                    MainShapesElementPositionStart(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementPositionStart(main, ambiguous_element) => match token {
                AttributX(line, offset) => MainShapesElementPositionX(main, ambiguous_element),
                _ => Err(token),
            },
            MainShapesElementPositionX(main, mut ambiguous_element) => match token {
                AttributY(line, offset) => MainShapesElementPositionY(main, ambiguous_element),
                PrimitiveValue(line, offset) => {
                    ambiguous_element.position.x = parse_i16(tokens);
                    MainShapesElementPositionXEnd(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementPositionXEnd(main, ambiguous_element) => match token {
                AttributY(line, offset) => MainShapesElementPositionY(main, ambiguous_element),
                _ => Err(token),
            },
            MainShapesElementPositionY(mut main, ambiguous_element) => match token {
                StructEnd(line, offset) => MainShapesElementPositionEnd(main, ambiguous_element),
                PrimitiveValue(line, offset) => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainShapesElementPositionYEnd(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementPositionYEnd(main, ambiguous_element) => match token {
                StructEnd(line, offset) => MainShapesElementPositionEnd(main, ambiguous_element),
                _ => Err(token),
            },
            MainShapesElementPositionEnd(main, ambiguous_element) => match token {
                AttributRotation(line, offset) => {
                    MainShapesElementRotation(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementRotation(main, mut ambiguous_element) => match token {
                AttributWidth(line, offset) => {
                    MainShapesPolygonWidth(main, ambiguous_element.into_polygon())
                }
                AttributShapes(line, offset) => {
                    MainShapesGroupShapes(main, ambiguous_element.into_group())
                }
                PrimitiveValue(line, offset) => {
                    ambiguous_element.rotation = parse_u8(tokens);
                    MainShapesElementRotationEnd(main, ambiguous_element)
                }
                _ => Err(token),
            },
            MainShapesElementRotationEnd(main, ambiguous_element) => match token {
                AttributWidth(line, offset) => {
                    MainShapesPolygonWidth(main, ambiguous_element.into_polygon())
                }
                AttributShapes(line, offset) => {
                    MainShapesGroupShapes(main, ambiguous_element.into_group())
                }
                _ => Err(token),
            },
            MainShapesPolygonWidth(main, mut polygon) => match token {
                AttributBorderColor(line, offset) => MainShapesPolygonBorderColor(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.width = parse_i16(tokens);
                    MainShapesPolygonWidthEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonWidthEnd(main, polygon) => match token {
                AttributBorderColor(line, offset) => MainShapesPolygonBorderColor(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColor(main, polygon) => match token {
                AttributFillColor(line, offset) => MainShapesPolygonFillColor(main, polygon),
                StructStart(line, offset) => MainShapesPolygonBorderColorStart(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColorStart(main, polygon) => match token {
                AttributRed(line, offset) => MainShapesPolygonBorderColorRed(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColorRed(main, mut polygon) => match token {
                AttributGreen(line, offset) => MainShapesPolygonBorderColorGreen(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.border_color.red = parse_u8(tokens);
                    MainShapesPolygonBorderColorRedEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonBorderColorRedEnd(main, polygon) => match token {
                AttributGreen(line, offset) => MainShapesPolygonBorderColorGreen(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColorGreen(main, mut polygon) => match token {
                AttributBlue(line, offset) => MainShapesPolygonBorderColorBlue(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.border_color.green = parse_u8(tokens);
                    MainShapesPolygonBorderColorGreenEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonBorderColorGreenEnd(main, polygon) => match token {
                AttributBlue(line, offset) => MainShapesPolygonBorderColorBlue(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColorBlue(main, mut polygon) => match token {
                StructEnd(line, offset) => MainShapesPolygonBorderColorEnd(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.border_color.blue = parse_u8(tokens);
                    MainShapesPolygonBorderColorBlueEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonBorderColorBlueEnd(main, polygon) => match token {
                StructEnd(line, offset) => MainShapesPolygonBorderColorEnd(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonBorderColorEnd(main, polygon) => match token {
                AttributFillColor(line, offset) => MainShapesPolygonFillColor(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColor(main, polygon) => match token {
                AttributVertices(line, offset) => MainShapesPolygonVertices(main, polygon),
                StructStart(line, offset) => MainShapesPolygonFillColorStart(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColorStart(main, polygon) => match token {
                AttributRed(line, offset) => MainShapesPolygonFillColorRed(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColorRed(main, mut polygon) => match token {
                AttributGreen(line, offset) => MainShapesPolygonFillColorGreen(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesPolygonFillColorRedEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonFillColorRedEnd(main, polygon) => match token {
                AttributGreen(line, offset) => MainShapesPolygonFillColorGreen(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColorGreen(main, mut polygon) => match token {
                AttributBlue(line, offset) => MainShapesPolygonFillColorBlue(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.green = parse_u8(tokens);
                    MainShapesPolygonFillColorGreenEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonFillColorGreenEnd(main, polygon) => match token {
                AttributBlue(line, offset) => MainShapesPolygonFillColorBlue(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColorBlue(main, mut polygon) => match token {
                StructEnd(line, offset) => MainShapesPolygonFillColorEnd(main, polygon),
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.blue = parse_u8(tokens);
                    MainShapesPolygonFillColorBlueEnd(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonFillColorBlueEnd(main, polygon) => match token {
                StructEnd(line, offset) => MainShapesPolygonFillColorEnd(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonFillColorEnd(main, polygon) => match token {
                AttributVertices(line, offset) => MainShapesPolygonVertices(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonVertices(mut main, polygon) => match token {
                StructEnd(line, offset) => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                ArrayStart(line, offset) => MainShapesPolygonVerticesStart(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonVerticesStart(main, polygon) => match token {
                ArrayEnd(line, offset) => MainShapesPolygonVerticesEnd(main, polygon),
                StructStart(line, offset) => MainShapesPolygonVerticesVertexStart(main, polygon),
                _ => Err(token),
            },
            MainShapesPolygonVerticesVertexStart(main, polygon) => match token {
                AttributX(line, offset) => {
                    MainShapesPolygonVerticesVertexX(main, polygon, Point::default())
                }
                _ => Err(token),
            },
            MainShapesPolygonVerticesVertexX(main, polygon, mut vertex) => match token {
                AttributY(line, offset) => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                PrimitiveValue(line, offset) => {
                    vertex.x = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex)
                }
                _ => Err(token),
            },
            MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex) => match token {
                AttributY(line, offset) => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                _ => Err(token),
            },
            MainShapesPolygonVerticesVertexY(main, mut polygon, mut vertex) => match token {
                StructEnd(line, offset) => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                PrimitiveValue(line, offset) => {
                    vertex.y = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexYEnd(main, polygon, vertex)
                }
                _ => Err(token),
            },
            MainShapesPolygonVerticesVertexYEnd(main, mut polygon, vertex) => match token {
                StructEnd(line, offset) => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                _ => Err(token),
            },
            MainShapesPolygonVerticesEnd(mut main, polygon) => match token {
                StructEnd(line, offset) => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                _ => Err(token),
            },
            MainShapesGroupShapes(mut main, group) => match token {
                StructEnd(line, offset) => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                } // TODO: Add default group
                ArrayStart(line, offset) => MainShapesGroupShapesStart(main, group),
                _ => Err(token),
            },
            MainShapesGroupShapesStart(main, group) => match token {
                ArrayEnd(line, offset) => MainShapesGroupShapesEnd(main, group),
                StructStart(line, offset) => {
                    MainShapesGroupShapesPolygonStart(main, group, Polygon::default())
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonStart(main, group, polygon) => match token {
                AttributPosition(line, offset) => {
                    MainShapesGroupShapesPolygonPosition(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPosition(main, group, polygon) => match token {
                AttributRotation(line, offset) => {
                    MainShapesGroupShapesPolygonRotation(main, group, polygon)
                }
                StructStart(line, offset) => {
                    MainShapesGroupShapesPolygonPositionStart(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionStart(main, group, polygon) => match token {
                AttributX(line, offset) => {
                    MainShapesGroupShapesPolygonPositionX(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionX(main, group, mut polygon) => match token {
                AttributY(line, offset) => {
                    MainShapesGroupShapesPolygonPositionY(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.position.x = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon) => match token {
                AttributY(line, offset) => {
                    MainShapesGroupShapesPolygonPositionY(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionY(main, group, mut polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonPositionEnd(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.position.y = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonPositionEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonPositionEnd(main, group, polygon) => match token {
                AttributRotation(line, offset) => {
                    MainShapesGroupShapesPolygonRotation(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonRotation(main, group, mut polygon) => match token {
                AttributWidth(line, offset) => {
                    MainShapesGroupShapesPolygonWidth(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.rotation = parse_u8(tokens);
                    MainShapesGroupShapesPolygonRotationEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonRotationEnd(main, group, polygon) => match token {
                AttributWidth(line, offset) => {
                    MainShapesGroupShapesPolygonWidth(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonWidth(main, group, mut polygon) => match token {
                AttributBorderColor(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.width = parse_i16(tokens);
                    MainShapesGroupShapesPolygonWidthEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonWidthEnd(main, group, polygon) => match token {
                AttributBorderColor(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColor(main, group, polygon) => match token {
                AttributFillColor(line, offset) => {
                    MainShapesGroupShapesPolygonFillColor(main, group, polygon)
                }
                StructStart(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon) => match token {
                AttributRed(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorRed(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorRed(main, group, mut polygon) => match token {
                AttributGreen(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.border_color.red = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon) => match token {
                AttributGreen(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorGreen(main, group, mut polygon) => match token {
                AttributBlue(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.border_color.green = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon) => match token {
                AttributBlue(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorBlue(main, group, mut polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.border_color.blue = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon) => match token {
                AttributFillColor(line, offset) => {
                    MainShapesGroupShapesPolygonFillColor(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColor(main, group, polygon) => match token {
                AttributVertices(line, offset) => {
                    MainShapesGroupShapesPolygonVertices(main, group, polygon)
                }
                StructStart(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorStart(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorStart(main, group, polygon) => match token {
                AttributRed(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorRed(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorRed(main, group, mut polygon) => match token {
                AttributGreen(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon) => match token {
                AttributGreen(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorGreen(main, group, mut polygon) => match token {
                AttributBlue(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.green = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon) => match token {
                AttributBlue(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorBlue(main, group, mut polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon)
                }
                PrimitiveValue(line, offset) => {
                    polygon.fill_color.blue = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon) => match token {
                StructEnd(line, offset) => {
                    MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon) => match token {
                AttributVertices(line, offset) => {
                    MainShapesGroupShapesPolygonVertices(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonVertices(main, mut group, polygon) => match token {
                StructEnd(line, offset) => {
                    group.shapes.push(polygon);
                    MainShapesGroupShapesStart(main, group)
                }
                ArrayStart(line, offset) => {
                    MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonVerticesStart(main, group, polygon) => match token {
                ArrayEnd(line, offset) => {
                    MainShapesGroupShapesPolygonVerticesEnd(main, group, polygon)
                }
                StructStart(line, offset) => MainShapesGroupShapesPolygonVerticesVertexStart(
                    main,
                    group,
                    polygon,
                    Point::default(),
                ),
                _ => Err(token),
            },
            MainShapesGroupShapesPolygonVerticesVertexStart(main, group, polygon, vertex) => {
                match token {
                    AttributX(line, offset) => {
                        MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, vertex)
                    }
                    _ => Err(token),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, mut vertex) => {
                match token {
                    AttributY(line, offset) => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    PrimitiveValue(line, offset) => {
                        vertex.x = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex)
                    }
                    _ => Err(token),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex) => {
                match token {
                    AttributY(line, offset) => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    _ => Err(token),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexY(main, group, mut polygon, mut vertex) => {
                match token {
                    StructEnd(line, offset) => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    PrimitiveValue(line, offset) => {
                        vertex.y = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, polygon, vertex)
                    }
                    _ => Err(token),
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, mut polygon, vertex) => {
                match token {
                    StructEnd(line, offset) => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    _ => Err(token),
                }
            }
            MainShapesGroupShapesPolygonVerticesEnd(main, mut group, polygon) => match token {
                StructEnd(line, offset) => {
                    group.shapes.push(polygon);
                    MainShapesGroupShapesStart(main, group)
                }
                _ => Err(token),
            },
            MainShapesGroupShapesEnd(mut main, group) => match token {
                StructEnd(line, offset) => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                }
                _ => Err(token),
            },
            MainShapesEnd(main) => match token {
                StructEnd(line, offset) => MainEnd(main),
                _ => Err(token),
            },
            MainEnd(_) => match token {
                _ => Err(token),
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
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            AttributBackgroundColor(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ];
        let expected = Main::default();

        assert_eq!(to_main_draw_element(tokens), expected);
    }

    #[test]
    fn custom_visible_extent() {
        let tokens = vec![
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributBackgroundColor(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ];
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
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            AttributBackgroundColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
        ];
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
            StructStart(0, 0),
            AttributVisibleExtent(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributBackgroundColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributShapes(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributWidth(0, 0),
            AttributBorderColor(0, 0),
            AttributFillColor(0, 0),
            AttributVertices(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributShapes(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            AttributWidth(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributBorderColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributFillColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributVertices(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributWidth(0, 0),
            AttributBorderColor(0, 0),
            AttributFillColor(0, 0),
            AttributVertices(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            PositiveValue(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributRotation(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributWidth(0, 0),
            PrimitiveValue(0, 0),
            NegativeValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributBorderColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            AttributFillColor(0, 0),
            StructStart(0, 0),
            AttributRed(0, 0),
            PrimitiveValue(0, 0),
            Zero(0, 0),
            AttributGreen(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            AttributBlue(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            StructEnd(0, 0),
            AttributVertices(0, 0),
            ArrayStart(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributX(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            AttributY(0, 0),
            PrimitiveValue(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            Zero(0, 0),
            One(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
            StructStart(0, 0),
            AttributPosition(0, 0),
            AttributRotation(0, 0),
            AttributShapes(0, 0),
            StructEnd(0, 0),
            ArrayEnd(0, 0),
            StructEnd(0, 0),
        ];
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
