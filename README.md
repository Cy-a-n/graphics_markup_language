# Graphics Markup Language (GCML)

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

## Installation

### Compiling from source

#### Step 1: Install Rust

Follow the official Rust installation guide to install Rust on your system. You can find available installation methods [here](https://forge.rust-lang.org/infra/other-installation-methods.html).

#### Step 2: Download the Source Code

##### Option 1: Using Git (Recommended)
Clone the repository using Git:

```bash
git clone --branch vX.X.X git@github.com:Cy-a-n/graphics_markup_language.git
```
i.e.: 
```bash
git clone --branch v0.1.2 git@github.com:Cy-a-n/graphics_markup_language.git
```

##### Option 2: Downloading Zip Archive
1. Visit the releases page on the repository [here](https://github.com/Cy-a-n/graphics_markup_language/releases).
2. Download the source code as a zip archive.
3. Extract the contents of the zip archive to your desired location.

#### Step 3: Build the Program

Navigate to the project directory:

```bash
cd your-rust-program
```

Build the program using cargo:

```bash
cargo build --release
```

This command compiles the program in release mode, optimizing for performance.

#### Step 4: Find the Executable

Once the build process is complete, you can find the executable in the target/release/ directory within your project folder.

- On Linux/macOS:
  ```./target/release/your-rust-program```

- On Windows:
  ```.\target\release\your-rust-program.exe```

Congratulations! You have successfully compiled and built the Rust program from source. If there are any additional steps specific to your program or platform, please refer to the program's documentation.

### Downloading prebuild executables

One can also download prebuild executables in the releases tab [here](https://github.com/Cy-a-n/graphics_markup_language/releases).

### Adding the Executable to the PATH Variable

Regardless of how you obtained the executable, you may want to add it to your system's `PATH` variable for convenient command-line access. Follow these general steps:

#### **On Windows:**

1. Copy the path to the directory containing the executable.

2. Open the Start menu and search for "Environment Variables."

3. Click on "Edit the system environment variables."

4. In the System Properties window, click the "Environment Variables" button.

5. In the "System Variables" section, select the "Path" variable and click "Edit."

6. Click "New" and paste the path you copied.

7. Click "OK" to close each window.

Now, you should be able to run the executable from the command prompt or PowerShell without specifying its full path.

#### **On macOS and Linux:**

1. Open a terminal window.

2. Edit your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`, `~/.bash_profile`, or `~/.profile`). Use a text editor like `nano` or `vim`.

3. Add the following line at the end of the file, replacing `/path/to/executable` with the actual path:

```bash
export PATH=$PATH:/path/to/executable
```

4. Save and close the file.

5. Run `source ~/.bashrc` (or the corresponding file for your shell) to apply the changes immediately.

Now, you should be able to run the executable from any terminal window without specifying its full path.

Remember to replace "/path/to/executable" with the actual path to the directory containing your executable.

## Usage

To compile source code to SVG using the executable, use the following command:

```bash
name_of_the_executable ./path/to/source/code [./path/to/output/file]
```

- Replace `name_of_the_executable` with the actual name of the executable, i.e. `graphics_markup_language`.

After compilation, open the resulting SVG file using an image viewer of your choice. Browsers are often the best choice for rendering SVG files.
