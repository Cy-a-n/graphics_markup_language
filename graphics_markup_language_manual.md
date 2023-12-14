# The great Graphics Markup Language manual

## Coordinate System

The outer bounds of the two dimensional coordinate system of the graphics markup language ranges from $(-(2^{16}-1)=-32767|-(2^{16}-1)=-32767)$ to $(2^{16}-1=32767|2^{16}-1=32767)$. The canvas, the visible area, starts at $(-(2^{15}-1)=-16384)$. The inner bounds can be defined by setting the

### Basic drawable elements

| Name    | Parameter                                                                   | Parameter as regular expression                                                                                                                                                                                                                                                                                                                                                           | Description                                                                            |
| ------- | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| i16     | value                                                                       | `=[+-]?[01]{1,15}`                                                                                                                                                                                                                                                                                                                                                                        | A value between $-2^{16}+1=-32767$ and $2^{16}-1=32767$ in binary. Default value is 0. |
| u8      | value                                                                       | `=[01]{1,8}`                                                                                                                                                                                                                                                                                                                                                                              | A value between $0$ and $2^{8}-1=255$ in binary. Default value is 0.                   |
| Color   | red, green, blue : u8                                                       | `{red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?}`                                                                                                                                                                                                                                                                                                                                   |
| Point   | x, y : i16                                                                  | `{x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?}`                                                                                                                                                                                                                                                                                                                                              |
| Polygon | thickness: i16, outline_color: Color, fill_color : Color, vertices+ : Point | `{thickness(=[+-]?[01]{1,15})?outline_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(=\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})+\])?}`                                                                                                                                                      |
| Main    | canvas_size : Point, background_color : Color, shapes\* : Polygon           | `^{canvas_size({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})?background_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?shapes(=\[({thickness(=[+-]?[01]{1,15})?outline_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?fill_color({red(=[01]{1,8})?green(=[01]{1,8})?blue(=[01]{1,8})?})?vertices(=\[({x(=[+-]?[01]{1,15})?y(=[+-]?[01]{1,15})?})+\])?})*\])?}$` |

### Tokenization

The tokenization is done with the following mealy machine:

| State | Input | Next state | Output   |
| ----- | ----- | ---------- | -------- |
| Start | "="   | Start      | Number   |
| Start | "-"   | Start      | Negative |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
|       |       |            |          |
