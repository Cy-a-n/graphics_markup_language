pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
pub struct Point {
    x: i16,
    y: i16,
}

pub struct Polygon {
    position: Point,
    rotation: u8,
    width: i16,
    border_color: Color,
    fill_color: Color,
    vertices: Vec<Point>,
}
pub struct Group {
    position: Point,
    rotation: u8,
    shapes: Vec<Polygon>,
}

pub enum Shape {
    Polygon(Polygon),
    Group(Group),
}

pub struct Main {
    visible_extent: Point,
    background_color: Color,
    shapes: [Shape],
}
