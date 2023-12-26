mod i16_parser;
mod u8_parser;

use std::vec::IntoIter;

use self::ParserState::*;
use crate::draw_elements::*;
use crate::token::Token;
use crate::token::Token::*;
use i16_parser::parse_i16;
use u8_parser::parse_u8;

pub fn to_main_draw_element(tokens: Vec<Token>) -> Main {
    let mut tokens = tokens.into_iter();
    let mut state = Start;

    loop {
        state = state.next_state(&mut tokens);

        if let MainEnd(main) = state {
            return main;
        }
    }
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
            MainVisibleExtentX(mut main) => match token {
                AttributY => MainVisibleExtentY(main),
                PrimitiveValue => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainVisibleExtentXEnd(main)
                }
                _ => Err,
            },
            MainVisibleExtentXEnd(main) => match token {
                AttributY => MainVisibleExtentY(main),
                _ => Err,
            },
            MainVisibleExtentY(mut main) => match token {
                StructEnd => MainVisibleExtentEnd(main),
                PrimitiveValue => {
                    main.visible_extent.y = parse_i16(tokens);
                    MainVisibleExtentYEnd(main)
                }
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
            MainBackgroundColorRed(mut main) => match token {
                AttributGreen => MainBackgroundColorGreen(main),
                PrimitiveValue => {
                    main.background_color.red = parse_u8(tokens);
                    MainBackgroundColorRedEnd(main)
                }
                _ => Err,
            },
            MainBackgroundColorRedEnd(main) => match token {
                AttributGreen => MainBackgroundColorGreen(main),
                _ => Err,
            },
            MainBackgroundColorGreen(mut main) => match token {
                AttributBlue => MainBackgroundColorBlue(main),
                PrimitiveValue => {
                    main.background_color.green = parse_u8(tokens);
                    MainBackgroundColorGreenEnd(main)
                }
                _ => Err,
            },
            MainBackgroundColorGreenEnd(main) => match token {
                AttributBlue => MainBackgroundColorBlue(main),
                _ => Err,
            },
            MainBackgroundColorBlue(mut main) => match token {
                StructEnd => MainBackgroundColorEnd(main),
                PrimitiveValue => {
                    main.background_color.blue = parse_u8(tokens);
                    MainBackgroundColorBlueEnd(main)
                }
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
                StructStart => MainShapesElementStart(main, AmbiguousElement::default()),
                _ => Err,
            },
            MainShapesElementStart(main, ambiguous_element) => match token {
                AttributPosition => MainShapesElementPosition(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementPosition(main, ambiguous_element) => match token {
                AttributRotation => MainShapesElementRotation(main, ambiguous_element),
                StructStart => MainShapesElementPositionStart(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementPositionStart(main, ambiguous_element) => match token {
                AttributX => MainShapesElementPositionX(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementPositionX(main, mut ambiguous_element) => match token {
                AttributY => MainShapesElementPositionY(main, ambiguous_element),
                PrimitiveValue => {
                    ambiguous_element.position.x = parse_i16(tokens);
                    MainShapesElementPositionXEnd(main, ambiguous_element)
                }
                _ => Err,
            },
            MainShapesElementPositionXEnd(main, ambiguous_element) => match token {
                AttributY => MainShapesElementPositionY(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementPositionY(mut main, ambiguous_element) => match token {
                StructEnd => MainShapesElementPositionEnd(main, ambiguous_element),
                PrimitiveValue => {
                    main.visible_extent.x = parse_i16(tokens);
                    MainShapesElementPositionYEnd(main, ambiguous_element)
                }
                _ => Err,
            },
            MainShapesElementPositionYEnd(main, ambiguous_element) => match token {
                StructEnd => MainShapesElementPositionEnd(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementPositionEnd(main, ambiguous_element) => match token {
                AttributRotation => MainShapesElementRotation(main, ambiguous_element),
                _ => Err,
            },
            MainShapesElementRotation(main, mut ambiguous_element) => match token {
                AttributWidth => MainShapesPolygonWidth(main, ambiguous_element.into_polygon()),
                AttributShapes => MainShapesGroupShapes(main, ambiguous_element.into_group()),
                PrimitiveValue => {
                    ambiguous_element.rotation = parse_u8(tokens);
                    MainShapesElementRotationEnd(main, ambiguous_element)
                }
                _ => Err,
            },
            MainShapesElementRotationEnd(main, ambiguous_element) => match token {
                AttributWidth => MainShapesPolygonWidth(main, ambiguous_element.into_polygon()),
                AttributShapes => MainShapesGroupShapes(main, ambiguous_element.into_group()),
                _ => Err,
            },
            MainShapesPolygonWidth(main, mut polygon) => match token {
                AttributBorderColor => MainShapesPolygonBorderColor(main, polygon),
                PrimitiveValue => {
                    polygon.width = parse_i16(tokens);
                    MainShapesPolygonWidthEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonWidthEnd(main, polygon) => match token {
                AttributBorderColor => MainShapesPolygonBorderColor(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColor(main, polygon) => match token {
                AttributFillColor => MainShapesPolygonFillColor(main, polygon),
                StructStart => MainShapesPolygonBorderColorStart(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColorStart(main, polygon) => match token {
                AttributRed => MainShapesPolygonBorderColorRed(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColorRed(main, mut polygon) => match token {
                AttributGreen => MainShapesPolygonBorderColorGreen(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.red = parse_u8(tokens);
                    MainShapesPolygonBorderColorRedEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonBorderColorRedEnd(main, polygon) => match token {
                AttributGreen => MainShapesPolygonBorderColorGreen(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColorGreen(main, mut polygon) => match token {
                AttributBlue => MainShapesPolygonBorderColorBlue(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.green = parse_u8(tokens);
                    MainShapesPolygonBorderColorGreenEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonBorderColorGreenEnd(main, polygon) => match token {
                AttributBlue => MainShapesPolygonBorderColorBlue(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColorBlue(main, mut polygon) => match token {
                StructEnd => MainShapesPolygonBorderColorEnd(main, polygon),
                PrimitiveValue => {
                    polygon.border_color.blue = parse_u8(tokens);
                    MainShapesPolygonBorderColorBlueEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonBorderColorBlueEnd(main, polygon) => match token {
                StructEnd => MainShapesPolygonBorderColorEnd(main, polygon),
                _ => Err,
            },
            MainShapesPolygonBorderColorEnd(main, polygon) => match token {
                AttributFillColor => MainShapesPolygonFillColor(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColor(main, polygon) => match token {
                AttributVertices => MainShapesPolygonVertices(main, polygon),
                StructStart => MainShapesPolygonFillColorStart(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColorStart(main, polygon) => match token {
                AttributRed => MainShapesPolygonFillColorRed(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColorRed(main, mut polygon) => match token {
                AttributGreen => MainShapesPolygonFillColorGreen(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesPolygonFillColorRedEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonFillColorRedEnd(main, polygon) => match token {
                AttributGreen => MainShapesPolygonFillColorGreen(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColorGreen(main, mut polygon) => match token {
                AttributBlue => MainShapesPolygonFillColorBlue(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.green = parse_u8(tokens);
                    MainShapesPolygonFillColorGreenEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonFillColorGreenEnd(main, polygon) => match token {
                AttributBlue => MainShapesPolygonFillColorBlue(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColorBlue(main, mut polygon) => match token {
                StructEnd => MainShapesPolygonFillColorEnd(main, polygon),
                PrimitiveValue => {
                    polygon.fill_color.blue = parse_u8(tokens);
                    MainShapesPolygonFillColorBlueEnd(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonFillColorBlueEnd(main, polygon) => match token {
                StructEnd => MainShapesPolygonFillColorEnd(main, polygon),
                _ => Err,
            },
            MainShapesPolygonFillColorEnd(main, polygon) => match token {
                AttributVertices => MainShapesPolygonVertices(main, polygon),
                _ => Err,
            },
            MainShapesPolygonVertices(mut main, polygon) => match token {
                StructEnd => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                ArrayStart => MainShapesPolygonVerticesStart(main, polygon),
                _ => Err,
            },
            MainShapesPolygonVerticesStart(main, polygon) => match token {
                ArrayEnd => MainShapesPolygonVerticesEnd(main, polygon),
                StructStart => MainShapesPolygonVerticesVertexStart(main, polygon),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexStart(main, polygon) => match token {
                AttributX => MainShapesPolygonVerticesVertexX(main, polygon, Point::default()),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexX(main, polygon, mut vertex) => match token {
                AttributY => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                PrimitiveValue => {
                    vertex.x = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex)
                }
                _ => Err,
            },
            MainShapesPolygonVerticesVertexXEnd(main, polygon, vertex) => match token {
                AttributY => MainShapesPolygonVerticesVertexY(main, polygon, vertex),
                _ => Err,
            },
            MainShapesPolygonVerticesVertexY(main, mut polygon, mut vertex) => match token {
                StructEnd => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                PrimitiveValue => {
                    vertex.y = parse_i16(tokens);
                    MainShapesPolygonVerticesVertexYEnd(main, polygon, vertex)
                }
                _ => Err,
            },
            MainShapesPolygonVerticesVertexYEnd(main, mut polygon, vertex) => match token {
                StructEnd => {
                    polygon.vertices.push(vertex);
                    MainShapesPolygonVerticesStart(main, polygon)
                }
                _ => Err,
            },
            MainShapesPolygonVerticesEnd(mut main, polygon) => match token {
                StructEnd => {
                    main.shapes.push(Shape::Polygon(polygon));
                    MainShapesStart(main)
                }
                _ => Err,
            },
            MainShapesGroupShapes(mut main, group) => match token {
                StructEnd => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                } // TODO: Add default group
                ArrayStart => MainShapesGroupShapesStart(main, group),
                _ => Err,
            },
            MainShapesGroupShapesStart(main, group) => match token {
                ArrayEnd => MainShapesGroupShapesEnd(main, group),
                StructStart => MainShapesGroupShapesPolygonStart(main, group, Polygon::default()),
                _ => Err,
            },
            MainShapesGroupShapesPolygonStart(main, group, polygon) => match token {
                AttributPosition => MainShapesGroupShapesPolygonPosition(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPosition(main, group, polygon) => match token {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonPositionStart(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionStart(main, group, polygon) => match token {
                AttributX => MainShapesGroupShapesPolygonPositionX(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionX(main, group, mut polygon) => match token {
                AttributY => MainShapesGroupShapesPolygonPositionY(main, group, polygon),
                PrimitiveValue => {
                    polygon.position.x = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionXEnd(main, group, polygon) => match token {
                AttributY => MainShapesGroupShapesPolygonPositionY(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionY(main, group, mut polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main, group, polygon),
                PrimitiveValue => {
                    polygon.position.y = parse_i16(tokens);
                    MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionYEnd(main, group, polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonPositionEnd(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonPositionEnd(main, group, polygon) => match token {
                AttributRotation => MainShapesGroupShapesPolygonRotation(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonRotation(main, group, mut polygon) => match token {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main, group, polygon),
                PrimitiveValue => {
                    polygon.rotation = parse_u8(tokens);
                    MainShapesGroupShapesPolygonRotationEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonRotationEnd(main, group, polygon) => match token {
                AttributWidth => MainShapesGroupShapesPolygonWidth(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonWidth(main, group, mut polygon) => match token {
                AttributBorderColor => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                PrimitiveValue => {
                    polygon.width = parse_i16(tokens);
                    MainShapesGroupShapesPolygonWidthEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonWidthEnd(main, group, polygon) => match token {
                AttributBorderColor => {
                    MainShapesGroupShapesPolygonBorderColor(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColor(main, group, polygon) => match token {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorStart(main, group, polygon) => match token {
                AttributRed => MainShapesGroupShapesPolygonBorderColorRed(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorRed(main, group, mut polygon) => match token {
                AttributGreen => MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon),
                PrimitiveValue => {
                    polygon.border_color.red = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorRedEnd(main, group, polygon) => match token {
                AttributGreen => MainShapesGroupShapesPolygonBorderColorGreen(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorGreen(main, group, mut polygon) => match token {
                AttributBlue => MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon),
                PrimitiveValue => {
                    polygon.border_color.green = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorGreenEnd(main, group, polygon) => match token {
                AttributBlue => MainShapesGroupShapesPolygonBorderColorBlue(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorBlue(main, group, mut polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon),
                PrimitiveValue => {
                    polygon.border_color.blue = parse_u8(tokens);
                    MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorBlueEnd(main, group, polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonBorderColorEnd(main, group, polygon) => match token {
                AttributFillColor => MainShapesGroupShapesPolygonFillColor(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColor(main, group, polygon) => match token {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonFillColorStart(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorStart(main, group, polygon) => match token {
                AttributRed => MainShapesGroupShapesPolygonFillColorRed(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorRed(main, group, mut polygon) => match token {
                AttributGreen => MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon),
                PrimitiveValue => {
                    polygon.fill_color.red = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorRedEnd(main, group, polygon) => match token {
                AttributGreen => MainShapesGroupShapesPolygonFillColorGreen(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorGreen(main, group, mut polygon) => match token {
                AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon),
                PrimitiveValue => {
                    polygon.fill_color.green = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorGreenEnd(main, group, polygon) => match token {
                AttributBlue => MainShapesGroupShapesPolygonFillColorBlue(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorBlue(main, group, mut polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon),
                PrimitiveValue => {
                    polygon.fill_color.blue = parse_u8(tokens);
                    MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon)
                }
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorBlueEnd(main, group, polygon) => match token {
                StructEnd => MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonFillColorEnd(main, group, polygon) => match token {
                AttributVertices => MainShapesGroupShapesPolygonVertices(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVertices(main, mut group, polygon) => match token {
                StructEnd => {
                    group.shapes.push(polygon);
                    MainShapesGroupShapesStart(main, group)
                }
                ArrayStart => MainShapesGroupShapesPolygonVerticesStart(main, group, polygon),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesStart(main, group, polygon) => match token {
                ArrayEnd => MainShapesGroupShapesPolygonVerticesEnd(main, group, polygon),
                StructStart => MainShapesGroupShapesPolygonVerticesVertexStart(
                    main,
                    group,
                    polygon,
                    Point::default(),
                ),
                _ => Err,
            },
            MainShapesGroupShapesPolygonVerticesVertexStart(main, group, polygon, vertex) => {
                match token {
                    AttributX => {
                        MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, vertex)
                    }
                    _ => Err,
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexX(main, group, polygon, mut vertex) => {
                match token {
                    AttributY => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    PrimitiveValue => {
                        vertex.x = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex)
                    }
                    _ => Err,
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexXEnd(main, group, polygon, vertex) => {
                match token {
                    AttributY => {
                        MainShapesGroupShapesPolygonVerticesVertexY(main, group, polygon, vertex)
                    }
                    _ => Err,
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexY(main, group, mut polygon, mut vertex) => {
                match token {
                    StructEnd => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    PrimitiveValue => {
                        vertex.y = parse_i16(tokens);
                        MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, polygon, vertex)
                    }
                    _ => Err,
                }
            }
            MainShapesGroupShapesPolygonVerticesVertexYEnd(main, group, mut polygon, vertex) => {
                match token {
                    StructEnd => {
                        polygon.vertices.push(vertex);
                        MainShapesGroupShapesPolygonVerticesStart(main, group, polygon)
                    }
                    _ => Err,
                }
            }
            MainShapesGroupShapesPolygonVerticesEnd(main, mut group, polygon) => match token {
                StructEnd => {
                    group.shapes.push(polygon);
                    MainShapesGroupShapesStart(main, group)
                }
                _ => Err,
            },
            MainShapesGroupShapesEnd(mut main, group) => match token {
                StructEnd => {
                    main.shapes.push(Shape::Group(group));
                    MainShapesStart(main)
                }
                _ => Err,
            },
            MainShapesEnd(main) => match token {
                StructEnd => MainEnd(main),
                _ => Err,
            },
            MainEnd(_) => match token {
                _ => Err,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_tokens() {}
}
