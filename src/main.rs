mod draw_elements;
mod lexer;
mod parser;
mod token;

fn main() {
    let source_code = "{visible_extent{x=+1y=-0}background_color{red=11111111green=00000000blue=10101010}shapes[{position{x=-1y=1}rotation=0101shapes[{position{x=+111111111111111y=-000000000000000}rotation=11111111width=-111111100000010border_color{red=1green=0blue=101}fill_color{red=1010green=10101blue=101010}vertices[{x=-1y=1}{x=-1y=1}{x=-1y=1}]}{position{x=+111111111111111y=-000000000000000}rotation=11111111width=-111111100000010border_color{red=1green=0blue=101}fill_color{red=1010green=10101blue=101010}vertices[{x=-1y=1}{x=-1y=1}{x=-1y=1}]}]}{position{x=+111111111111111y=-000000000000000}rotation=11111111width=-111111100000010border_color{red=1green=0blue=101}fill_color{red=1010green=10101blue=101010}vertices[{x=-1y=1}{x=-1y=1}{x=-1y=1}]}{position{x=-1y=1}rotation=0101shapes[{position{x=+111111111111111y=-000000000000000}rotation=11111111width=-111111100000010border_color{red=1green=0blue=101}fill_color{red=1010green=10101blue=101010}vertices[{x=-1y=1}{x=-1y=1}{x=-1y=1}]}{position{x=+111111111111111y=-000000000000000}rotation=11111111width=-111111100000010border_color{red=1green=0blue=101}fill_color{red=1010green=10101blue=101010}vertices[{x=-1y=1}{x=-1y=1}{x=-1y=1}]}]}]}";

    println!("{:?}", parser::to_polygons(lexer::tokenize(source_code)));
}
