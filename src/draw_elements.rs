pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }
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

pub struct AmbiguousElement {
    position: Point,
    rotation: u8,
}

impl AmbiguousElement {
    pub fn into_polygon(self) -> Polygon {
        Polygon {
            position: self.position,
            rotation: self.rotation,
            width: 0,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![],
        }
    }
}

pub struct Main {
    visible_extent: Point,
    background_color: Color,
    shapes: Vec<Shape>,
}

impl Main {
    pub fn default() -> Self {
        Self {
            visible_extent: Point {
                x: i16::MAX / 2,
                y: i16::MAX / 2,
            },
            background_color: Color::default(),
            shapes: vec![],
        }
    }
}
