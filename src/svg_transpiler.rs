use crate::parser::{Color, Point, SimplePolygon};

pub fn to_svg(polygons: Vec<SimplePolygon>) -> String {
    let mut output = String::from(
        "<svg version=\"1.1\" width=\"100%\" height=\"100%\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-255 -255 510 510\">",
    );
    for polygon in polygons.into_iter().rev() {
        let SimplePolygon {
            width,
            border_color:
                Color {
                    red: border_red,
                    green: border_green,
                    blue: border_blue,
                },
            fill_color:
                Color {
                    red: fill_red,
                    green: fill_green,
                    blue: fill_blue,
                },
            vertices,
        } = polygon;

        match &vertices[..] {
            [] => continue,
            [Point { x, y }] => {
                output.push_str(&format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"{width}\" fill=\"#{border_red:02x}{border_green:02x}{border_blue:02x}\"/>"
                ));
            }
            [Point { x: x1, y: y1 }, Point { x: x2, y: y2 }] => {
                output.push_str(&format!("<line x1=\"{x1}\" x2=\"{x2}\" y1=\"{y1}\" y2=\"{y2}\" stroke=\"#{border_red:02x}{border_green:02x}{border_blue:02x}\" stroke-width=\"{width}\" stroke-linecap=\"round\"/>"))
            }
            _ => {
                let mut vertices_str = String::from("");
                for Point { x, y } in vertices {
                    vertices_str.push_str(&format!(" {x},{y} "));
                }

                output.push_str(&format!("<polygon points=\"{vertices_str}\" fill=\"#{fill_red:02x}{fill_green:02x}{fill_blue:02x}\" stroke=\"#{border_red:02x}{border_green:02x}{border_blue:02x}\" stroke-width=\"{width}\" stroke-linejoin=\"round\"/>"));
            }
        }
    }
    output.push_str("</svg>");
    output
}
