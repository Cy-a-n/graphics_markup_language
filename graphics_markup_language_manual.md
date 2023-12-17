# The great Graphics Markup Language manual

## Coordinate System

The outer bounds of the two dimensional coordinate system of the graphics markup language ranges from $(-(2^{16}-1)=-32767|-(2^{16}-1)=-32767)$ to $(2^{16}-1=32767|2^{16}-1=32767)$. The visible, the visible area, starts at $(-(2^{15}-1)=-16384)$. The inner bounds can be defined by setting the

### Basic drawable elements

| Name    | Parameter                                                               | Parameter as regular expression                                                                                                                                                                                                                                                                                                                                                         | Description                                                                            |
| ------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| i16     | value                                                                   | `=[+-]?[01]{1,15}`                                                                                                                                                                                                                                                                                                                                                                      | A value between $-2^{16}+1=-32767$ and $2^{16}-1=32767$ in binary. Default value is 0. |
| u8      | value                                                                   | `=[01]{1,8}`                                                                                                                                                                                                                                                                                                                                                                            | A value between $0$ and $2^{8}-1=255$ in binary. Default value is 0.                   |
| Color   | red, green, blue : u8                                                   | `{red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?}`                                                                                                                                                                                                                                                                                                                                 |
| Point   | x, y : i16                                                              | `{x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?}`                                                                                                                                                                                                                                                                                                                                            |
| Polygon | width: i16, border_color: Color, fill_color : Color, vertices : Point\* | `{width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(=\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?}`                                                                                                                                                         |
| Main    | visible_extent : Point, background_color : Color, shapes : Polygon\*    | `^{visible_extent({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?background_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?shapes(=\[({width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(=\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?})*\])?}$` |

### Tokenization

The tokenization is done with the following mealy machine:

| State                  | Input | Next state             | Output                    |
| ---------------------- | ----- | ---------------------- | ------------------------- |
| Start                  | " "   | Start                  |                           |
| Start                  | "="   | Start                  | PrimitiveValue            |
| Start                  | "-"   | Start                  | Negative                  |
| Start                  | "+"   | Start                  | Positive                  |
| Start                  | "0"   | Start                  | Zero                      |
| Start                  | "1"   | Start                  | One                       |
| Start                  | "{"   | Start                  | StructStart               |
| Start                  | "}"   | Start                  | StructEnd                 |
| Start                  | "["   | Start                  | ArrayStart                |
| Start                  | "]"   | Start                  | ArrayEnd                  |
| Start                  | "r"   | String_r               |                           |
| String_r               | "e"   | String_re              |                           |
| String_re              | "d"   | Start                  | Attribut_red              |
| Start                  | "g"   | String_g               |                           |
| String_g               | "r"   | String_gr              |                           |
| String_gr              | "e"   | String_gre             |                           |
| String_gre             | "e"   | String_gree            |                           |
| String_gree            | "n"   | Start                  | Attribut_green            |
| Start                  | "b"   | String_b               |                           |
| String_b               | "l"   | String_bl              |                           |
| String_bl              | "u"   | String_blu             |                           |
| String_blu             | "e"   | Start                  | Attribut_blue             |
| Start                  | "x"   | Start                  | Attribut_x                |
| Start                  | "y"   | Start                  | Attribut_y                |
| Start                  | "w"   | String_w               |                           |
| String_w               | "i"   | String_wi              |                           |
| String_wi              | "d"   | String_wid             |                           |
| String_wid             | "t"   | String_widt            |                           |
| String_widt            | "h"   | Start                  | Attribut_width            |
| String_b               | "o"   | String_bo              |                           |
| String_bo              | "r"   | String_bor             |                           |
| String_bor             | "d"   | String_bord            |                           |
| String_bord            | "e"   | String_borde           |                           |
| String_borde           | "r"   | String_border          |                           |
| String_border          | "\_"  | String_border\_        |                           |
| String_border\_        | "c"   | String_border_c        |                           |
| String_border_c        | "o"   | String_border_co       |                           |
| String_border_co       | "l"   | String_border_col      |                           |
| String_border_col      | "o"   | String_border_colo     |                           |
| String_border_colo     | "r"   | Start                  | Attribut_border_color     |
| Start                  | "f"   | String_f               |                           |
| String_f               | "i"   | String_fi              |                           |
| String_fi              | "l"   | String_fil             |                           |
| String_fil             | "l"   | String_fill            |                           |
| String_fill            | "\_"  | String_fill\_          |                           |
| String_fill\_          | "c"   | String_fill_c          |                           |
| String_fill_c          | "o"   | String_fill_co         |                           |
| String_fill_co         | "l"   | String_fill_col        |                           |
| String_fill_col        | "o"   | String_fill_colo       |                           |
| String_fill_col        | "o"   | String_fill_colo       |                           |
| String_fill_colo       | "r"   | Start                  | Attribut_fill_color       |
| Start                  | "v"   | String_v               |                           |
| String_v               | "e"   | String_ve              |                           |
| String_ve              | "r"   | String_ver             |                           |
| String_ver             | "t"   | String_vert            |                           |
| String_vert            | "i"   | String_verti           |                           |
| String_verti           | "c"   | String_vertic          |                           |
| String_vertic          | "e"   | String_vertice         |                           |
| String_vertice         | "s"   | Start                  | Attribut_vertices         |
| Start                  | "v"   | String_v               |                           |
| String_v               | "i"   | String_vi              |                           |
| String_vi              | "s"   | String_vis             |                           |
| String_vis             | "i"   | String_visi            |                           |
| String_visi            | "b"   | String_visib           |                           |
| String_visib           | "l"   | String_visibl          |                           |
| String_visibl          | "s"   | String_visible         |                           |
| String_visible         | "\_"  | String_visible\_       |                           |
| String_visible\_       | "e"   | String_visible_e       |                           |
| String_visible_e       | "x"   | String_visible_ex      |                           |
| String_visible_ex      | "t"   | String_visible_ext     |                           |
| String_visible_ext     | "e"   | String_visible_exte    |                           |
| String_visible_exte    | "n"   | String_visible_exten   |                           |
| String_visible_exten   | "t"   | Start                  | Attribut_visible_extent   |
| String_b               | "a"   | String_ba              |                           |
| String_ba              | "c"   | String_bac             |                           |
| String_bac             | "k"   | String_back            |                           |
| String_back            | "g"   | String_backg           |                           |
| String_backg           | "r"   | String_backgr          |                           |
| String_backgr          | "o"   | String_backgro         |                           |
| String_backgro         | "u"   | String_backgrou        |                           |
| String_backgrou        | "n"   | String_backgroun       |                           |
| String_backgroun       | "d"   | String_background      |                           |
| String_background      | "\_"  | String_background\_    |                           |
| String_background\_    | "c"   | String_background_c    |                           |
| String_background_c    | "o"   | String_background_co   |                           |
| String_background_co   | "l"   | String_background_col  |                           |
| String_background_col  | "o"   | String_background_colo |                           |
| String_background_colo | "r"   | Start                  | Attribut_background_color |
| Start                  | "s"   | String_s               |                           |
| String_s               | "h"   | String_sh              |                           |
| String_sh              | "a"   | String_sha             |                           |
| String_sha             | "p"   | String_shap            |                           |
| String_shap            | "e"   | String_shape           |                           |
| String_shape           | "s"   | Start                  | Attribut_shapes           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
|                        |       |                        |                           |
