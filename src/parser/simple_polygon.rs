use super::color::Color;
use super::point::Point;
use super::polygon::Polygon;

pub(super) fn simplify(root_polygon: Polygon) -> Vec<SimplePolygon> {
    let mut output: Vec<SimplePolygon> = vec![];

    let Polygon {
        position,
        rotation_rad,
        width,
        border_color,
        fill_color,
        mut vertices,
        children,
    } = root_polygon;

    for mut child in children {
        child.position.rotate(rotation_rad);
        child.position.add(&position);
        child.rotation_rad += rotation_rad;
        output.extend(simplify(child));
    }

    // Rotate the vertices
    for vertex in &mut vertices {
        vertex.rotate(rotation_rad);
        vertex.add(&position);
    }

    output.push(SimplePolygon {
        width,
        border_color,
        fill_color,
        vertices,
    });

    output
}

#[derive(Debug, PartialEq)]
pub struct SimplePolygon {
    pub width: i16,
    pub border_color: Color,
    pub fill_color: Color,
    pub vertices: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn minimum() {
        let polygon = Polygon::default();
        let expected = SimplePolygon {
            width: 0,
            border_color: Color::default(),
            fill_color: Color::default(),
            vertices: vec![],
        };
        let actual = simplify(polygon);
        assert_eq!(actual, vec![expected]);
    }

    #[test]
    fn maximum() {
        let polygon = Polygon {
            position: Point { x: 1, y: 1 },
            rotation_rad: PI,
            width: 1,
            border_color: Color {
                red: 1,
                green: 1,
                blue: 1,
            },
            fill_color: Color {
                red: 2,
                green: 2,
                blue: 2,
            },
            vertices: vec![Point { x: 2, y: 1 }, Point { x: 1, y: 2 }],
            children: vec![
                Polygon {
                    position: Point { x: 1, y: 1 },
                    rotation_rad: 0.0,
                    border_color: Color::default(),
                    fill_color: Color::default(),
                    width: 0,
                    vertices: vec![Point { x: 1, y: 1 }],
                    children: vec![],
                },
                Polygon {
                    position: Point { x: 1, y: 1 },
                    rotation_rad: PI,
                    border_color: Color::default(),
                    fill_color: Color::default(),
                    width: 0,
                    vertices: vec![Point { x: 2, y: 2 }],
                    children: vec![Polygon {
                        position: Point { x: 1, y: 1 },
                        rotation_rad: 0.0,
                        border_color: Color::default(),
                        fill_color: Color::default(),
                        width: 0,
                        vertices: vec![Point { x: 3, y: 3 }],
                        children: vec![],
                    }],
                },
            ],
        };
        let expected = vec![
            SimplePolygon {
                width: 0,
                border_color: Color::default(),
                fill_color: Color::default(),
                vertices: vec![Point { x: -1, y: -1 }],
            },
            SimplePolygon {
                width: 0,
                border_color: Color::default(),
                fill_color: Color::default(),
                vertices: vec![Point { x: 4, y: 4 }],
            },
            SimplePolygon {
                width: 0,
                border_color: Color::default(),
                fill_color: Color::default(),
                vertices: vec![Point { x: 2, y: 2 }],
            },
            SimplePolygon {
                width: 1,
                border_color: Color {
                    red: 1,
                    green: 1,
                    blue: 1,
                },
                fill_color: Color {
                    red: 2,
                    green: 2,
                    blue: 2,
                },
                vertices: vec![Point { x: -1, y: 0 }, Point { x: 0, y: -1 }],
            },
        ];
        let actual = simplify(polygon);
        assert_eq!(actual, expected);
    }
}
