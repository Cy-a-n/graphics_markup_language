#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    #[allow(unused)]
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
    #[allow(unused)]
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
    pub children: Vec<Polygon>,
}

impl Polygon {
    #[allow(unused)]
    pub fn default() -> Self {
        Polygon {
            position: Point::default(),
            rotation: 0,
            width: 0,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![],
            children: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Main {
    pub visible_extent: Point,
    pub background_color: Color,
    pub shapes: Vec<Polygon>,
}

impl Main {
    #[allow(unused)]
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
