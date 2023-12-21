# The great Graphics Markup Language manual

## Coordinate System

The outer bounds of the two dimensional coordinate system of the graphics markup language ranges from $(-(2^{16}-1)=-32767|-(2^{16}-1)=-32767)$ to $(2^{16}-1=32767|2^{16}-1=32767)$. The visible, the visible area, starts at $(-(2^{15}-1)=-16384)$. The inner bounds can be defined by setting the

### Basic drawable elements

| Name    | Parameter                                                                                                | Parameter as regular expression                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Description                                                                            |
| ------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| i16     | value                                                                                                    | `=[+-]?[01]{1,15}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | A value between $-2^{16}+1=-32767$ and $2^{16}-1=32767$ in binary. Default value is 0. |
| u8      | value                                                                                                    | `=[01]{1,8}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | A value between $0$ and $2^{8}-1=255$ in binary. Default value is 0.                   |
| Color   | red, green, blue : u8                                                                                    | `{red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| Point   | x, y : i16                                                                                               | `{x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Polygon | position : Point, rotation : u8, width: i16, border_color: Color, fill_color : Color, vertices : Point\* | `{position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Group   | postion : Point, rotation : u8, shapes : Polygon\*                                                       | `{position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?shapes(\[({position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?})*\])?}`                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Main    | visible_extent : Point, background_color : Color, shapes : (Polygon \| Group)\*                          | `^{visible_extent({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?background_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?shapes(\[({position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?}\|{position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?shapes(\[({position({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?rotation(=[01]{1,8})?width(=[+-]?[01]{1,15})?border_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})*\])?})*\])?})*\])?}$` |

### Tokenization

The tokenization is done with the following mealy machine:

| State                  | Input | Next state             | Output                  |
| ---------------------- | ----- | ---------------------- | ----------------------- |
| Start                  | " "   | Start                  |                         |
| Start                  | "="   | Start                  | PrimitiveValue          |
| Start                  | "-"   | Start                  | Negative                |
| Start                  | "+"   | Start                  | Positive                |
| Start                  | "0"   | Start                  | Zero                    |
| Start                  | "1"   | Start                  | One                     |
| Start                  | "{"   | Start                  | StructStart             |
| Start                  | "}"   | Start                  | StructEnd               |
| Start                  | "["   | Start                  | ArrayStart              |
| Start                  | "]"   | Start                  | ArrayEnd                |
| Start                  | "r"   | String_r               |                         |
| String_r               | "e"   | String_re              |                         |
| String_re              | "d"   | Start                  | MainAttributRed         |
| Start                  | "g"   | String_g               |                         |
| String_g               | "r"   | String_gr              |                         |
| String_gr              | "e"   | String_gre             |                         |
| String_gre             | "e"   | String_gree            |                         |
| String_gree            | "n"   | Start                  | AttributGreen           |
| Start                  | "b"   | String_b               |                         |
| String_b               | "l"   | String_bl              |                         |
| String_bl              | "u"   | String_blu             |                         |
| String_blu             | "e"   | Start                  | AttributBlue            |
| Start                  | "x"   | Start                  | AttributX               |
| Start                  | "y"   | Start                  | AttributY               |
| Start                  | "p"   | String_p               |                         |
| String_p               | "o"   | String_po              |                         |
| String_po              | "s"   | String_pos             |                         |
| String_pos             | "i"   | String_posi            |                         |
| String_posi            | "t"   | String_posit           |                         |
| String_posit           | "i"   | String_positi          |                         |
| String_positi          | "o"   | String_positio         |                         |
| String_positio         | "n"   | Start                  | AttributPosition        |
| String_r               | "o"   | String_ro              |                         |
| String_ro              | "t"   | String_rot             |                         |
| String_rota            | "t"   | String_rotat           |                         |
| String_rotat           | "i"   | String_rotati          |                         |
| String_rotati          | "o"   | String_rotatio         |                         |
| String_rotatio         | "n"   | Start                  | AttributRotation        |
| Start                  | "w"   | String_w               |                         |
| String_w               | "i"   | String_wi              |                         |
| String_wi              | "d"   | String_wid             |                         |
| String_wid             | "t"   | String_widt            |                         |
| String_widt            | "h"   | Start                  | AttributWidth           |
| String_b               | "o"   | String_bo              |                         |
| String_bo              | "r"   | String_bor             |                         |
| String_bor             | "d"   | String_bord            |                         |
| String_bord            | "e"   | String_borde           |                         |
| String_borde           | "r"   | String_border          |                         |
| String_border          | "\_"  | String_border\_        |                         |
| String_border\_        | "c"   | String_border_c        |                         |
| String_border_c        | "o"   | String_border_co       |                         |
| String_border_co       | "l"   | String_border_col      |                         |
| String_border_col      | "o"   | String_border_colo     |                         |
| String_border_colo     | "r"   | Start                  | AttributBorderColor     |
| Start                  | "f"   | String_f               |                         |
| String_f               | "i"   | String_fi              |                         |
| String_fi              | "l"   | String_fil             |                         |
| String_fil             | "l"   | String_fill            |                         |
| String_fill            | "\_"  | String_fill\_          |                         |
| String_fill\_          | "c"   | String_fill_c          |                         |
| String_fill_c          | "o"   | String_fill_co         |                         |
| String_fill_co         | "l"   | String_fill_col        |                         |
| String_fill_col        | "o"   | String_fill_colo       |                         |
| String_fill_col        | "o"   | String_fill_colo       |                         |
| String_fill_colo       | "r"   | Start                  | AttributFillColor       |
| Start                  | "v"   | String_v               |                         |
| String_v               | "e"   | String_ve              |                         |
| String_ve              | "r"   | String_ver             |                         |
| String_ver             | "t"   | String_vert            |                         |
| String_vert            | "i"   | String_verti           |                         |
| String_verti           | "c"   | String_vertic          |                         |
| String_vertic          | "e"   | String_vertice         |                         |
| String_vertice         | "s"   | Start                  | AttributVertices        |
| String_vi              | "s"   | String_vis             |                         |
| String_vis             | "i"   | String_visi            |                         |
| String_visi            | "b"   | String_visib           |                         |
| String_visib           | "l"   | String_visibl          |                         |
| String_visibl          | "s"   | String_visible         |                         |
| String_visible         | "\_"  | String_visible\_       |                         |
| String_visible\_       | "e"   | String_visible_e       |                         |
| String_visible_e       | "x"   | String_visible_ex      |                         |
| String_visible_ex      | "t"   | String_visible_ext     |                         |
| String_visible_ext     | "e"   | String_visible_exte    |                         |
| String_visible_exte    | "n"   | String_visible_exten   |                         |
| String_visible_exten   | "t"   | Start                  | AttributVisibleExtent   |
| String_ba              | "c"   | String_bac             |                         |
| String_bac             | "k"   | String_back            |                         |
| String_back            | "g"   | String_backg           |                         |
| String_backg           | "r"   | String_backgr          |                         |
| String_backgr          | "o"   | String_backgro         |                         |
| String_backgro         | "u"   | String_backgrou        |                         |
| String_backgrou        | "n"   | String_backgroun       |                         |
| String_backgroun       | "d"   | String_background      |                         |
| String_background      | "\_"  | String_background\_    |                         |
| String_background\_    | "c"   | String_background_c    |                         |
| String_background_c    | "o"   | String_background_co   |                         |
| String_background_co   | "l"   | String_background_col  |                         |
| String_background_col  | "o"   | String_background_colo |                         |
| String_background_colo | "r"   | Start                  | AttributBackgroundColor |
| Start                  | "s"   | String_s               |                         |
| String_s               | "h"   | String_sh              |                         |
| String_sh              | "a"   | String_sha             |                         |
| String_sha             | "p"   | String_shap            |                         |
| String_shap            | "e"   | String_shape           |                         |
| String_shape           | "s"   | Start                  | AttributShapes          |

### Parsing

The tokens are parsed into a dynamic array of simple polygons. For this purpose I will use an extended kind of mealy machine that is able to parse consecutive number tokens (Zero and One) directly into numbers, instead of having a path for every possible number.

| State                                           | Input                   | Next state                                      | Action                                              |
| ----------------------------------------------- | ----------------------- | ----------------------------------------------- | --------------------------------------------------- |
| Start                                           | StructStart             | MainStart                                       |                                                     |
| MainStart                                       | AttributVisibleExtent   | MainVisibleExtent                               |                                                     |
| MainVisibleExtent                               | AttributBackgroundColor | MainBackgroundColor                             | MainVisibleExtentDefault                            |
| MainVisibleExtent                               | StructStart             | MainVisibleExtentStart                          |                                                     |
| MainVisibleExtentStart                          | AttributX               | MainVisibleExtentX                              |                                                     |
| MainVisibleExtentX                              | AttributY               | MainVisibleExtentY                              | MainVisibleExtentXDefault                           |
| MainVisibleExtentX                              | PrimitiveValue          | MainVisibleExtentXEnd                           | ParseMainVisibleExtentX                             |
| MainVisibleExtentXEnd                           | AttributY               | MainVisibleExtentY                              |                                                     |
| MainVisibleExtentY                              | StructEnd               | MainVisibleExtentEnd                            | MainVisibleExtentYDefault                           |
| MainVisibleExtentY                              | PrimitiveValue          | MainVisibleExtentYEnd                           | ParseMainVisibleExtentY                             |
| MainVisibleExtentYEnd                           | StructEnd               | MainVisibleExtentEnd                            |                                                     |
| MainVisibleExtentEnd                            | AttributBackgroundColor | MainBackgroundColor                             |                                                     |
| MainBackgroundColor                             | AttributShapes          | MainShapes                                      | MainBackgroundColorDefault                          |
| MainBackgroundColor                             | StructStart             | MainBackgroundColorStart                        |                                                     |
| MainBackgroundColorStart                        | AttributRed             | MainBackgroundColorRed                          |                                                     |
| MainBackgroundColorRed                          | AttributGreen           | MainBackgroundColorGreen                        | MainBackgroundColorRedDefault                       |
| MainBackgroundColorRed                          | PrimitiveValue          | MainBackgroundColorRedEnd                       | ParseMainBackgroundColorRed                         |
| MainBackgroundColorRedEnd                       | AttributGreen           | MainBackgroundColorGreen                        |                                                     |
| MainBackgroundColorGreen                        | AttributBlue            | MainBackgroundColorBlue                         | MainBackgroundColorGreenDefault                     |
| MainBackgroundColorGreen                        | PrimitiveValue          | MainBackgroundColorGreenEnd                     | ParseMainBackgroundColorGreen                       |
| MainBackgroundColorGreenEnd                     | AttributBlue            | MainBackgroundColorBlue                         |                                                     |
| MainBackgroundColorBlue                         | StructEnd               | MainBackgroundColorEnd                          | MainBackgroundColorBlueDefault                      |
| MainBackgroundColorBlue                         | PrimitiveValue          | MainBackgroundColorBlueEnd                      | ParseMainBackgroundColorBlue                        |
| MainBackgroundColorBlueEnd                      | StructEnd               | MainBackgroundColorEnd                          |                                                     |
| MainBackgroundColorEnd                          | AttributShapes          | MainShapes                                      |                                                     |
| MainShapes                                      | StructEnd               | MainEnd                                         | MainShapesDefault                                   |
| MainShapes                                      | ArrayStart              | MainShapesStart                                 |                                                     |
| MainShapesStart                                 | ArrayEnd                | MainShapesEnd                                   |                                                     |
| MainShapesStart                                 | StructStart             | MainShapesElementStart                          |                                                     |
| MainShapesElementStart                          | AttributPosition        | MainShapesElementPosition                       |                                                     |
| MainShapesElementPosition                       | AttributRotation        | MainShapesElementRotation                       | MainShapesElementPositionDefault                    |
| MainShapesElementPosition                       | StructStart             | MainShapesElementPositionStart                  |                                                     |
| MainShapesElementPositionStart                  | AttributX               | MainShapesElementPositionX                      |                                                     |
| MainShapesElementPositionX                      | AttributY               | MainShapesElementPositionY                      | MainShapesElementPositionXDefault                   |
| MainShapesElementPositionX                      | PrimitiveValue          | MainShapesElementPositionXEnd                   | ParseMainShapesElementPositionX                     |
| MainShapesElementPositionXEnd                   | AttributY               | MainShapesElementPositionY                      |                                                     |
| MainShapesElementPositionY                      | StructEnd               | MainShapesElementPositionEnd                    | MainShapesElementPositionYDefault                   |
| MainShapesElementPositionY                      | PrimitiveValue          | MainShapesElementPositionYEnd                   | ParseMainShapesElementPositionY                     |
| MainShapesElementPositionYEnd                   | StructEnd               | MainShapesElementPositionEnd                    |                                                     |
| MainShapesElementPositionEnd                    | AttributRotation        | MainShapesElementRotation                       |                                                     |
| MainShapesElementRotation                       | AttributWidth           | MainShapesPolygonWidth                          | MainShapesElementRotationDefault                    |
| MainShapesElementRotation                       | AttributShapes          | MainShapesGroupShapes                           | MainShapesElementRotationDefault                    |
| MainShapesElementRotation                       | PrimitiveValue          | MainShapesElementRotationEnd                    | ParseMainShapesElementRotation                      |
| MainShapesElementRotationEnd                    | AttributWidth           | MainShapesPolygonWidth                          |                                                     |
| MainShapesElementRotationEnd                    | AttributShapes          | MainShapesGroupShapes                           |                                                     |
| MainShapesPolygonWidth                          | AttributBorderColor     | MainShapesPolygonBorderColor                    | MainShapesPolygonWidthDefault                       |
| MainShapesPolygonWidth                          | PrimitiveValue          | MainShapesPolygonWidthEnd                       | ParseMainShapesPolygonWidth                         |
| MainShapesPolygonWidthEnd                       | AttributBorderColor     | MainShapesPolygonBorderColor                    |                                                     |
| MainShapesPolygonBorderColor                    | AttributFillColor       | MainShapesPolygonFillColor                      | MainShapesPolygonBorderColorDefault                 |
| MainShapesPolygonBorderColor                    | StructStart             | MainShapesPolygonBorderColorStart               |                                                     |
| MainShapesPolygonBorderColorStart               | AttributRed             | MainShapesPolygonBorderColorRed                 |                                                     |
| MainShapesPolygonBorderColorRed                 | AttributGreen           | MainShapesPolygonBorderColorGreen               | MainShapesPolygonBorderColorRedDefault              |
| MainShapesPolygonBorderColorRed                 | PrimitiveValue          | MainShapesPolygonBorderColorRedEnd              | ParseMainShapesPolygonBorderColorRed                |
| MainShapesPolygonBorderColorRedEnd              | AttributGreen           | MainShapesPolygonBorderColorGreen               |                                                     |
| MainShapesPolygonBorderColorGreen               | AttributBlue            | MainShapesPolygonBorderColorBlue                | MainShapesPolygonBorderColorGreenDefault            |
| MainShapesPolygonBorderColorGreen               | PrimitiveValue          | MainShapesPolygonBorderColorGreenEnd            | ParseMainShapesPolygonBorderColorGreen              |
| MainShapesPolygonBorderColorGreenEnd            | AttributBlue            | MainShapesPolygonBorderColorBlue                |                                                     |
| MainShapesPolygonBorderColorBlue                | StructEnd               | MainShapesPolygonBorderColorEnd                 | MainShapesPolygonBorderColorBlueDefault             |
| MainShapesPolygonBorderColorBlue                | PrimitiveValue          | MainShapesPolygonBorderColorBlueEnd             | ParseMainShapesPolygonBorderColorBlue               |
| MainShapesPolygonBorderColorBlueEnd             | StructEnd               | MainShapesPolygonBorderColorEnd                 |                                                     |
| MainShapesPolygonBorderColorEnd                 | AttributFillColor       | MainShapesPolygonFillColor                      |                                                     |
| MainShapesPolygonFillColor                      | AttributVertices        | MainShapesPolygonVertices                       | MainShapesPolygonFillColorDefault                   |
| MainShapesPolygonFillColor                      | StructStart             | MainShapesPolygonFillColorStart                 |                                                     |
| MainShapesPolygonFillColorStart                 | AttributRed             | MainShapesPolygonFillColorRed                   |                                                     |
| MainShapesPolygonFillColorRed                   | AttributGreen           | MainShapesPolygonFillColorGreen                 | MainShapesPolygonFillColorRedDefault                |
| MainShapesPolygonFillColorRed                   | PrimitiveValue          | MainShapesPolygonFillColorRedEnd                | ParseMainShapesPolygonFillColorRed                  |
| MainShapesPolygonFillColorRedEnd                | AttributGreen           | MainShapesPolygonFillColorGreen                 |                                                     |
| MainShapesPolygonFillColorGreen                 | AttributBlue            | MainShapesPolygonFillColorBlue                  | MainShapesPolygonFillColorGreenDefault              |
| MainShapesPolygonFillColorGreen                 | PrimitiveValue          | MainShapesPolygonFillColorGreenEnd              | ParseMainShapesPolygonFillColorGreen                |
| MainShapesPolygonFillColorGreenEnd              | AttributBlue            | MainShapesPolygonFillColorBlue                  |                                                     |
| MainShapesPolygonFillColorBlue                  | StructEnd               | MainShapesPolygonFillColorEnd                   | MainShapesPolygonFillColorBlueDefault               |
| MainShapesPolygonFillColorBlue                  | PrimitiveValue          | MainShapesPolygonFillColorBlueEnd               | ParseMainShapesPolygonFillColorBlue                 |
| MainShapesPolygonFillColorBlueEnd               | StructEnd               | MainShapesPolygonFillColorEnd                   |                                                     |
| MainShapesPolygonFillColorEnd                   | AttributVertices        | MainShapesPolygonVertices                       |                                                     |
| MainShapesPolygonVertices                       | StructEnd               | MainShapesStart                                 | MainShapesPolygonVerticesDefault                    |
| MainShapesPolygonVertices                       | ArrayStart              | MainShapesPolygonVerticesStart                  |                                                     |
| MainShapesPolygonVerticesStart                  | ArrayEnd                | MainShapesPolygonVerticesEnd                    |                                                     |
| MainShapesPolygonVerticesStart                  | StructStart             | MainShapesPolygonVerticesVertexStart            |                                                     |
| MainShapesPolygonVerticesVertexStart            | AttributX               | MainShapesPolygonVerticesVertexX                |                                                     |
| MainShapesPolygonVerticesVertexX                | AttributY               | MainShapesPolygonVerticesVertexY                | MainShapesPolygonVerticesVertexXDefault             |
| MainShapesPolygonVerticesVertexX                | PrimitiveValue          | MainShapesPolygonVerticesVertexXEnd             | ParseMainShapesPolygonVerticesVertexX               |
| MainShapesPolygonVerticesVertexXEnd             | AttributY               | MainShapesPolygonVerticesVertexY                |                                                     |
| MainShapesPolygonVerticesVertexY                | StructEnd               | MainShapesPolygonVerticesStart                  | MainShapesPolygonVerticesVertexYDefault             |
| MainShapesPolygonVerticesVertexY                | PrimitiveValue          | MainShapesPolygonVerticesVertexYEnd             | ParseMainShapesPolygonVerticesVertexY               |
| MainShapesPolygonVerticesVertexYEnd             | StructEnd               | MainShapesPolygonVerticesStart                  |                                                     |
| MainShapesPolygonVerticesEnd                    | StructEnd               | MainShapesStart                                 |                                                     |
| MainShapesGroupShapes                           | StructEnd               | MainShapesStart                                 | MainShapesGroupShapesDefault                        |
| MainShapesGroupShapes                           | ArrayStart              | MainShapesGroupShapesStart                      |                                                     |
| MainShapesGroupShapesStart                      | ArrayEnd                | MainShapesGroupShapesEnd                        |                                                     |
| MainShapesGroupShapesStart                      | StructStart             | MainShapesGroupShapesPolygonStart               |                                                     |
| MainShapesGroupShapesPolygonStart               | AttributPosition        | MainShapesGroupShapesPolygonPosition            |                                                     |
| MainShapesGroupShapesPolygonPosition            | AttributRotation        | MainShapesGroupShapesPolygonRotation            | MainShapesGroupShapesPolygonPositionDefault         |
| MainShapesGroupShapesPolygonPosition            | StructStart             | MainShapesGroupShapesPolygonPositionStart       |                                                     |
| MainShapesGroupShapesPolygonPositionStart       | AttributX               | MainShapesGroupShapesPolygonPositionX           |                                                     |
| MainShapesGroupShapesPolygonPositionX           | AttributY               | MainShapesGroupShapesPolygonPositionY           | MainShapesGroupShapesPolygonPositionXDefault        |
| MainShapesGroupShapesPolygonPositionX           | PrimitiveValue          | MainShapesGroupShapesPolygonPositionXEnd        | ParseMainShapesGroupShapesPolygonPositionX          |
| MainShapesGroupShapesPolygonPositionXEnd        | AttributY               | MainShapesGroupShapesPolygonPositionY           |                                                     |
| MainShapesGroupShapesPolygonPositionY           | StructEnd               | MainShapesGroupShapesPolygonPositionEnd         | MainShapesGroupShapesPolygonPositionYDefault        |
| MainShapesGroupShapesPolygonPositionY           | PrimitiveValue          | MainShapesGroupShapesPolygonPositionYEnd        | ParseMainShapesGroupShapesPolygonPositionY          |
| MainShapesGroupShapesPolygonPositionYEnd        | StructEnd               | MainShapesGroupShapesPolygonPositionEnd         |                                                     |
| MainShapesGroupShapesPolygonPositionEnd         | AttributRotation        | MainShapesGroupShapesPolygonRotation            |                                                     |
| MainShapesGroupShapesPolygonRotation            | AttributWidth           | MainShapesGroupShapesPolygonWidth               | MainShapesGroupShapesPolygonRotationDefault         | // |
| MainShapesGroupShapesPolygonRotation            | PrimitiveValue          | MainShapesGroupShapesPolygonRotationEnd         | ParseMainShapesGroupShapesPolygonRotation           |
| MainShapesGroupShapesPolygonRotationEnd         | AttributWidth           | MainShapesGroupShapesPolygonWidth               |                                                     |
| MainShapesGroupShapesPolygonWidth               | AttributBorderColor     | MainShapesGroupShapesPolygonBorderColor         | MainShapesGroupShapesPolygonWidthDefault            |
| MainShapesGroupShapesPolygonWidth               | PrimitiveValue          | MainShapesGroupShapesPolygonWidthEnd            | ParseMainShapesGroupShapesPolygonWidth              |
| MainShapesGroupShapesPolygonWidthEnd            | AttributBorderColor     | MainShapesGroupShapesPolygonBorderColor         |                                                     |
| MainShapesGroupShapesPolygonBorderColor         | AttributFillColor       | MainShapesGroupShapesPolygonFillColor           | MainShapesGroupShapesPolygonBorderColorDefault      |
| MainShapesGroupShapesPolygonBorderColor         | StructStart             | MainShapesGroupShapesPolygonBorderColorStart    |                                                     |
| MainShapesGroupShapesPolygonBorderColorStart    | AttributRed             | MainShapesGroupShapesPolygonBorderColorRed      |                                                     |
| MainShapesGroupShapesPolygonBorderColorRed      | AttributGreen           | MainShapesGroupShapesPolygonBorderColorGreen    | MainShapesGroupShapesPolygonBorderColorRedDefault   |
| MainShapesGroupShapesPolygonBorderColorRed      | PrimitiveValue          | MainShapesGroupShapesPolygonBorderColorRedEnd   | ParseMainShapesGroupShapesPolygonBorderColorRed     |
| MainShapesGroupShapesPolygonBorderColorRedEnd   | AttributGreen           | MainShapesGroupShapesPolygonBorderColorGreen    |                                                     |
| MainShapesGroupShapesPolygonBorderColorGreen    | AttributBlue            | MainShapesGroupShapesPolygonBorderColorBlue     | MainShapesGroupShapesPolygonBorderColorGreenDefault |
| MainShapesGroupShapesPolygonBorderColorGreen    | PrimitiveValue          | MainShapesGroupShapesPolygonBorderColorGreenEnd | ParseMainShapesGroupShapesPolygonBorderColorGreen   |
| MainShapesGroupShapesPolygonBorderColorGreenEnd | AttributBlue            | MainShapesGroupShapesPolygonBorderColorBlue     |                                                     |
| MainShapesGroupShapesPolygonBorderColorBlue     | StructEnd               | MainShapesGroupShapesPolygonBorderColorEnd      | MainShapesGroupShapesPolygonBorderColorBlueDefault  |
| MainShapesGroupShapesPolygonBorderColorBlue     | PrimitiveValue          | MainShapesGroupShapesPolygonBorderColorBlueEnd  | ParseMainShapesGroupShapesPolygonBorderColorBlue    |
| MainShapesGroupShapesPolygonBorderColorBlueEnd  | StructEnd               | MainShapesGroupShapesPolygonBorderColorEnd      |                                                     |
| MainShapesGroupShapesPolygonBorderColorEnd      | AttributFillColor       | MainShapesGroupShapesPolygonFillColor           |                                                     |
| MainShapesGroupShapesPolygonFillColor           | AttributVertices        | MainShapesGroupShapesPolygonVertices            | MainShapesGroupShapesPolygonFillColorDefault        |
| MainShapesGroupShapesPolygonFillColor           | StructStart             | MainShapesGroupShapesPolygonFillColorStart      |                                                     |
| MainShapesGroupShapesPolygonFillColorStart      | AttributRed             | MainShapesGroupShapesPolygonFillColorRed        |                                                     |
| MainShapesGroupShapesPolygonFillColorRed        | AttributGreen           | MainShapesGroupShapesPolygonFillColorGreen      | MainShapesGroupShapesPolygonFillColorRedDefault     |
| MainShapesGroupShapesPolygonFillColorRed        | PrimitiveValue          | MainShapesGroupShapesPolygonFillColorRedEnd     | ParseMainShapesGroupShapesPolygonFillColorRed       |
| MainShapesGroupShapesPolygonFillColorRedEnd     | AttributGreen           | MainShapesGroupShapesPolygonFillColorGreen      |                                                     |
| MainShapesGroupShapesPolygonFillColorGreen      | AttributBlue            | MainShapesGroupShapesPolygonFillColorBlue       | MainShapesGroupShapesPolygonFillColorGreenDefault   |
| MainShapesGroupShapesPolygonFillColorGreen      | PrimitiveValue          | MainShapesGroupShapesPolygonFillColorGreenEnd   | ParseMainShapesGroupShapesPolygonFillColorGreen     |
| MainShapesGroupShapesPolygonFillColorGreenEnd   | AttributBlue            | MainShapesGroupShapesPolygonFillColorBlue       |                                                     |
| MainShapesGroupShapesPolygonFillColorBlue       | StructEnd               | MainShapesGroupShapesPolygonFillColorEnd        | MainShapesGroupShapesPolygonFillColorBlueDefault    |
| MainShapesGroupShapesPolygonFillColorBlue       | PrimitiveValue          | MainShapesGroupShapesPolygonFillColorBlueEnd    | ParseMainShapesGroupShapesPolygonFillColorBlue      |
| MainShapesGroupShapesPolygonFillColorBlueEnd    | StructEnd               | MainShapesGroupShapesPolygonFillColorEnd        |                                                     |
| MainShapesGroupShapesPolygonFillColorEnd        | AttributVertices        | MainShapesGroupShapesPolygonVertices            |                                                     |
| MainShapesGroupShapesPolygonVertices            | StructEnd               | MainShapesStart                                 | MainShapesGroupShapesPolygonVerticesDefault         |
| MainShapesGroupShapesPolygonVertices            | ArrayStart              | MainShapesGroupShapesPolygonVerticesStart       |                                                     |
| MainShapesGroupShapesPolygonVerticesStart       | ArrayEnd                | MainShapesGroupShapesPolygonVerticesEnd         |                                                     |
| MainShapesGroupShapesPolygonVerticesStart       | StructStart             | MainShapesGroupShapesPolygonVerticesVertexStart |                                                     |
| MainShapesGroupShapesPolygonVerticesVertexStart | AttributX               | MainShapesGroupShapesPolygonVerticesVertexX     |                                                     |
| MainShapesGroupShapesPolygonVerticesVertexX     | AttributY               | MainShapesGroupShapesPolygonVerticesVertexY     | MainShapesGroupShapesPolygonVerticesVertexXDefault  |
| MainShapesGroupShapesPolygonVerticesVertexX     | PrimitiveValue          | MainShapesGroupShapesPolygonVerticesVertexXEnd  | ParseMainShapesGroupShapesPolygonVerticesVertexX    |
| MainShapesGroupShapesPolygonVerticesVertexXEnd  | AttributY               | MainShapesGroupShapesPolygonVerticesVertexY     |                                                     |
| MainShapesGroupShapesPolygonVerticesVertexY     | StructEnd               | MainShapesGroupShapesPolygonVerticesStart       | MainShapesGroupShapesPolygonVerticesVertexYDefault  |
| MainShapesGroupShapesPolygonVerticesVertexY     | PrimitiveValue          | MainShapesGroupShapesPolygonVerticesVertexYEnd  | ParseMainShapesGroupShapesPolygonVerticesVertexY    |
| MainShapesGroupShapesPolygonVerticesVertexYEnd  | StructEnd               | MainShapesGroupShapesPolygonVerticesVertexEnd   |                                                     |
| MainShapesGroupShapesPolygonVerticesVertexEnd   | StructEnd               | MainShapesGroupShapesPolygonVerticesStart       |                                                     |
| MainShapesGroupShapesPolygonVerticesEnd         | StructEnd               | MainShapesGroupShapesStart                      |                                                     |
| MainShapesGroupShapesPolygonEnd                 | StructEnd               | MainShapesGroupShapesPolygonEnd                 |                                                     |
| MainShapesGroupShapesEnd                        | StructEnd               | MainShapesStart                                 |                                                     |
| MainShapesEnd                                   | StructEnd               | MainEnd                                         |                                                     |
