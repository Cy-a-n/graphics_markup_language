# Graphics Markup Language (GML)

## Overview

The Graphics Markup Language (GML) is a lightweight markup language designed for describing 2D graphics, providing a simple and expressive way to represent shapes such as circles, lines, and polygons. Unlike XML-based formats, GML is tailored specifically for graphics and excels at grouping shapes into objects for easy manipulation, movement, and rotation.

## Features

-   **Supports Basic Shapes:**
    GML supports three fundamental shapes - circles, lines, and polygons. These primitives form the building blocks for creating more complex graphics.

-   **Grouping:**
    GML allows easy grouping of shapes into objects. This feature simplifies the organization of elements, making it convenient to manipulate multiple shapes as a single entity.

-   **Transformation:**
    GML enables the transformation of shapes and groups. You can easily move and rotate objects within the graphics canvas.

## Syntax

The syntax of the language can be described by a context-free grammar:

D => 0 | 1  
S = + | -
U8 => = D | = DD | = DD | = DDD | = DDDD | = DDDDD | = DDDDDD | = DDDDDDD | = DDDDDDDD  
I16 => = S D | = S DD | = S DD | = S DDD | = S DDDD | = S DDDDD | = S DDDDDD | = S DDDDDDD | = S DDDDDDDD | = S DDDDDDDDD | = S DDDDDDDDDD | = S DDDDDDDDDDD | = S DDDDDDDDDDDD | = S DDDDDDDDDDDDD | = S DDDDDDDDDDDDDD | = S DDDDDDDDDDDDDDD  
Y => y | y I16  
X => x | x I16  
Point => { X Y }  
Red => red | red U8  
Green => green | green U8  
Blue => blue | blue U8  
Color => { Red Green Blue }  
Position => position | position Point  
Rotation => rotation | rotation U8  
Width => width | width I16  
BorderColor => border_color | border_color Color  
FillColor => fill_color | fill_color Color  
VerticesArray => $\epsilon$ | Position VerticesArray  
Vertices => vertices | vertices [ VerticesArray ]  
ChildrenArray => $\epsilon$ | Polygon ChildrenArray  
Children => children | children [ ChildrenArray ]  
Polygon => { Position Rotation Width BorderColor FillColor Vertices Children }

The simplest syntactically correct .gcml file would thus be:

```
{
    position
    rotation
    width
    border_color
    fill_color
    vertices
    children
}
```

For further examples check out the `./examples/` directory.
