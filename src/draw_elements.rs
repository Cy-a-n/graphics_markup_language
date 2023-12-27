#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, PartialEq)]
pub struct Polygon {
    pub position: Point,
    pub rotation: u8,
    pub width: i16,
    pub border_color: Color,
    pub fill_color: Color,
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn default() -> Self {
        Polygon {
            position: Point::default(),
            rotation: 0,
            width: 0,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Group {
    pub position: Point,
    pub rotation: u8,
    pub shapes: Vec<Polygon>,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Polygon(Polygon),
    Group(Group),
}

#[derive(Debug, PartialEq)]
pub struct AmbiguousElement {
    pub position: Point,
    pub rotation: u8,
}

impl AmbiguousElement {
    pub fn default() -> Self {
        AmbiguousElement {
            position: Point::default(),
            rotation: 0,
        }
    }

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

    pub fn into_group(self) -> Group {
        Group {
            position: self.position,
            rotation: self.rotation,
            shapes: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Main {
    pub visible_extent: Point,
    pub background_color: Color,
    pub shapes: Vec<Shape>,
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
