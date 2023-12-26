use crate::token::Token;
use crate::token::Token::*;
use State::*;

pub fn to_tokens(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_state = Start;

    for input in source_code.chars() {
        let (next_state, output) = current_state.next_state(&input);

        if let Err(expected) = next_state {
            panic!(
                "Lexer error: Expected {:?} in state {:?}, got {}.",
                expected, current_state, input
            )
        }

        if let Some(token) = output {
            tokens.push(token);
        }

        current_state = next_state;
    }

    tokens
}

#[derive(Debug)]
enum State {
    Start,
    #[allow(non_camel_case_types)]
    String_r,
    #[allow(non_camel_case_types)]
    String_re,
    #[allow(non_camel_case_types)]
    String_g,
    #[allow(non_camel_case_types)]
    String_gr,
    #[allow(non_camel_case_types)]
    String_gre,
    #[allow(non_camel_case_types)]
    String_gree,
    #[allow(non_camel_case_types)]
    String_b,
    #[allow(non_camel_case_types)]
    String_bl,
    #[allow(non_camel_case_types)]
    String_blu,
    #[allow(non_camel_case_types)]
    String_p,
    #[allow(non_camel_case_types)]
    String_po,
    #[allow(non_camel_case_types)]
    String_pos,
    #[allow(non_camel_case_types)]
    String_posi,
    #[allow(non_camel_case_types)]
    String_posit,
    #[allow(non_camel_case_types)]
    String_positi,
    #[allow(non_camel_case_types)]
    String_positio,
    #[allow(non_camel_case_types)]
    String_ro,
    #[allow(non_camel_case_types)]
    String_rot,
    #[allow(non_camel_case_types)]
    String_rota,
    #[allow(non_camel_case_types)]
    String_rotat,
    #[allow(non_camel_case_types)]
    String_rotati,
    #[allow(non_camel_case_types)]
    String_rotatio,
    #[allow(non_camel_case_types)]
    String_w,
    #[allow(non_camel_case_types)]
    String_wi,
    #[allow(non_camel_case_types)]
    String_wid,
    #[allow(non_camel_case_types)]
    String_widt,
    #[allow(non_camel_case_types)]
    String_bo,
    #[allow(non_camel_case_types)]
    String_bor,
    #[allow(non_camel_case_types)]
    String_bord,
    #[allow(non_camel_case_types)]
    String_borde,
    #[allow(non_camel_case_types)]
    String_border,
    #[allow(non_camel_case_types)]
    String_border_,
    #[allow(non_camel_case_types)]
    String_border_c,
    #[allow(non_camel_case_types)]
    String_border_co,
    #[allow(non_camel_case_types)]
    String_border_col,
    #[allow(non_camel_case_types)]
    String_border_colo,
    #[allow(non_camel_case_types)]
    String_f,
    #[allow(non_camel_case_types)]
    String_fi,
    #[allow(non_camel_case_types)]
    String_fil,
    #[allow(non_camel_case_types)]
    String_fill,
    #[allow(non_camel_case_types)]
    String_fill_,
    #[allow(non_camel_case_types)]
    String_fill_c,
    #[allow(non_camel_case_types)]
    String_fill_co,
    #[allow(non_camel_case_types)]
    String_fill_col,
    #[allow(non_camel_case_types)]
    String_fill_colo,
    #[allow(non_camel_case_types)]
    String_v,
    #[allow(non_camel_case_types)]
    String_ve,
    #[allow(non_camel_case_types)]
    String_ver,
    #[allow(non_camel_case_types)]
    String_vert,
    #[allow(non_camel_case_types)]
    String_verti,
    #[allow(non_camel_case_types)]
    String_vertic,
    #[allow(non_camel_case_types)]
    String_vertice,
    #[allow(non_camel_case_types)]
    String_vi,
    #[allow(non_camel_case_types)]
    String_vis,
    #[allow(non_camel_case_types)]
    String_visi,
    #[allow(non_camel_case_types)]
    String_visib,
    #[allow(non_camel_case_types)]
    String_visibl,
    #[allow(non_camel_case_types)]
    String_visible,
    #[allow(non_camel_case_types)]
    String_visible_,
    #[allow(non_camel_case_types)]
    String_visible_e,
    #[allow(non_camel_case_types)]
    String_visible_ex,
    #[allow(non_camel_case_types)]
    String_visible_ext,
    #[allow(non_camel_case_types)]
    String_visible_exte,
    #[allow(non_camel_case_types)]
    String_visible_exten,
    #[allow(non_camel_case_types)]
    String_ba,
    #[allow(non_camel_case_types)]
    String_bac,
    #[allow(non_camel_case_types)]
    String_back,
    #[allow(non_camel_case_types)]
    String_backg,
    #[allow(non_camel_case_types)]
    String_backgr,
    #[allow(non_camel_case_types)]
    String_backgro,
    #[allow(non_camel_case_types)]
    String_backgrou,
    #[allow(non_camel_case_types)]
    String_backgroun,
    #[allow(non_camel_case_types)]
    String_background,
    #[allow(non_camel_case_types)]
    String_background_,
    #[allow(non_camel_case_types)]
    String_background_c,
    #[allow(non_camel_case_types)]
    String_background_co,
    #[allow(non_camel_case_types)]
    String_background_col,
    #[allow(non_camel_case_types)]
    String_background_colo,
    #[allow(non_camel_case_types)]
    String_s,
    #[allow(non_camel_case_types)]
    String_sh,
    #[allow(non_camel_case_types)]
    String_sha,
    #[allow(non_camel_case_types)]
    String_shap,
    #[allow(non_camel_case_types)]
    String_shape,
    Err(Vec<char>),
}

impl State {
    fn next_state(&self, input: &char) -> (Self, Option<Token>) {
        match self {
            Err(_) => {
                panic!("The `next_state`-method should never be called on State::Err")
            }
            Start => match input {
                c if c.is_whitespace() => (Start, None),
                '=' => (Start, Some(PrimitiveValue)),
                '-' => (Start, Some(NegativValue)),
                '+' => (Start, Some(PositiveValue)),
                '0' => (Start, Some(Zero)),
                '1' => (Start, Some(One)),
                '{' => (Start, Some(StructStart)),
                '}' => (Start, Some(StructEnd)),
                '[' => (Start, Some(ArrayStart)),
                ']' => (Start, Some(ArrayEnd)),
                'r' => (String_r, None),
                'g' => (String_g, None),
                'b' => (String_b, None),
                'x' => (Start, Some(AttributX)),
                'y' => (Start, Some(AttributY)),
                'p' => (String_p, None),
                'w' => (String_w, None),
                'f' => (String_f, None),
                'v' => (String_v, None),
                's' => (String_s, None),
                _ => (
                    Err(vec![
                        ' ', '=', '-', '+', '0', '1', '{', '}', '[', ']', 'r', 'g', 'b', 'x', 'y',
                        'p', 'w', 'f', 'v', 's',
                    ]),
                    None,
                ),
            },
            String_r => match input {
                'e' => (String_re, None),
                'o' => (String_ro, None),
                _ => (Err(vec!['e', 'o']), None),
            },
            String_re => match input {
                'd' => (Start, Some(AttributRed)),
                _ => (Err(vec!['d']), None),
            },
            String_g => match input {
                'r' => (String_gr, None),
                _ => (Err(vec!['r']), None),
            },
            String_gr => match input {
                'e' => (String_gre, None),
                _ => (Err(vec!['e']), None),
            },
            String_gre => match input {
                'e' => (String_gree, None),
                _ => (Err(vec!['e']), None),
            },
            String_gree => match input {
                'n' => (Start, Some(AttributGreen)),
                _ => (Err(vec!['n']), None),
            },
            String_b => match input {
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
                _ => (Err(vec!['l', 'o', 'a']), None),
            },
            String_bl => match input {
                'u' => (String_blu, None),
                _ => (Err(vec!['u']), None),
            },
            String_blu => match input {
                'e' => (Start, Some(AttributBlue)),
                _ => (Err(vec!['e']), None),
            },
            String_p => match input {
                'o' => (String_po, None),
                _ => (Err(vec!['o']), None),
            },
            String_po => match input {
                's' => (String_pos, None),
                _ => (Err(vec!['s']), None),
            },
            String_pos => match input {
                'i' => (String_posi, None),
                _ => (Err(vec!['i']), None),
            },
            String_posi => match input {
                't' => (String_posit, None),
                _ => (Err(vec!['t']), None),
            },
            String_posit => match input {
                'i' => (String_positi, None),
                _ => (Err(vec!['i']), None),
            },
            String_positi => match input {
                'o' => (String_positio, None),
                _ => (Err(vec!['o']), None),
            },
            String_positio => match input {
                'n' => (Start, Some(AttributPosition)),
                _ => (Err(vec!['n']), None),
            },
            String_ro => match input {
                't' => (String_rot, None),
                _ => (Err(vec!['t']), None),
            },
            String_rot => match input {
                'a' => (String_rota, None),
                _ => (Err(vec!['a']), None),
            },
            String_rota => match input {
                't' => (String_rotat, None),
                _ => (Err(vec!['t']), None),
            },
            String_rotat => match input {
                'i' => (String_rotati, None),
                _ => (Err(vec!['i']), None),
            },
            String_rotati => match input {
                'o' => (String_rotatio, None),
                _ => (Err(vec!['o']), None),
            },
            String_rotatio => match input {
                'n' => (Start, Some(AttributRotation)),
                _ => (Err(vec!['n']), None),
            },
            String_w => match input {
                'i' => (String_wi, None),
                _ => (Err(vec!['i']), None),
            },
            String_wi => match input {
                'd' => (String_wid, None),
                _ => (Err(vec!['d']), None),
            },
            String_wid => match input {
                't' => (String_widt, None),
                _ => (Err(vec!['t']), None),
            },
            String_widt => match input {
                'h' => (Start, Some(AttributWidth)),
                _ => (Err(vec!['h']), None),
            },
            String_bo => match input {
                'r' => (String_bor, None),
                _ => (Err(vec!['r']), None),
            },
            String_bor => match input {
                'd' => (String_bord, None),
                _ => (Err(vec!['d']), None),
            },
            String_bord => match input {
                'e' => (String_borde, None),
                _ => (Err(vec!['e']), None),
            },
            String_borde => match input {
                'r' => (String_border, None),
                _ => (Err(vec!['r']), None),
            },
            String_border => match input {
                '_' => (String_border_, None),
                _ => (Err(vec!['_']), None),
            },
            String_border_ => match input {
                'c' => (String_border_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_border_c => match input {
                'o' => (String_border_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_co => match input {
                'l' => (String_border_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_border_col => match input {
                'o' => (String_border_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_border_colo => match input {
                'r' => (Start, Some(AttributBorderColor)),
                _ => (Err(vec!['r']), None),
            },
            String_f => match input {
                'i' => (String_fi, None),
                _ => (Err(vec!['i']), None),
            },
            String_fi => match input {
                'l' => (String_fil, None),
                _ => (Err(vec!['l']), None),
            },
            String_fil => match input {
                'l' => (String_fill, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill => match input {
                '_' => (String_fill_, None),
                _ => (Err(vec!['_']), None),
            },
            String_fill_ => match input {
                'c' => (String_fill_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_fill_c => match input {
                'o' => (String_fill_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_co => match input {
                'l' => (String_fill_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_fill_col => match input {
                'o' => (String_fill_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_fill_colo => match input {
                'r' => (Start, Some(AttributFillColor)),
                _ => (Err(vec!['r']), None),
            },
            String_v => match input {
                'e' => (String_ve, None),
                'i' => (String_vi, None),
                _ => (Err(vec!['e', 'i']), None),
            },
            String_ve => match input {
                'r' => (String_ver, None),
                _ => (Err(vec!['r']), None),
            },
            String_ver => match input {
                't' => (String_vert, None),
                _ => (Err(vec!['t']), None),
            },
            String_vert => match input {
                'i' => (String_verti, None),
                _ => (Err(vec!['i']), None),
            },
            String_verti => match input {
                'c' => (String_vertic, None),
                _ => (Err(vec!['c']), None),
            },
            String_vertic => match input {
                'e' => (String_vertice, None),
                _ => (Err(vec!['e']), None),
            },
            String_vertice => match input {
                's' => (Start, Some(AttributVertices)),
                _ => (Err(vec!['s']), None),
            },
            String_vi => match input {
                's' => (String_vis, None),
                _ => (Err(vec!['s']), None),
            },
            String_vis => match input {
                'i' => (String_visi, None),
                _ => (Err(vec!['i']), None),
            },
            String_visi => match input {
                'b' => (String_visib, None),
                _ => (Err(vec!['b']), None),
            },
            String_visib => match input {
                'l' => (String_visibl, None),
                _ => (Err(vec!['l']), None),
            },
            String_visibl => match input {
                'e' => (String_visible, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible => match input {
                '_' => (String_visible_, None),
                _ => (Err(vec!['_']), None),
            },
            String_visible_ => match input {
                'e' => (String_visible_e, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_e => match input {
                'x' => (String_visible_ex, None),
                _ => (Err(vec!['x']), None),
            },
            String_visible_ex => match input {
                't' => (String_visible_ext, None),
                _ => (Err(vec!['t']), None),
            },
            String_visible_ext => match input {
                'e' => (String_visible_exte, None),
                _ => (Err(vec!['e']), None),
            },
            String_visible_exte => match input {
                'n' => (String_visible_exten, None),
                _ => (Err(vec!['n']), None),
            },
            String_visible_exten => match input {
                't' => (Start, Some(AttributVisibleExtent)),
                _ => (Err(vec!['t']), None),
            },
            String_ba => match input {
                'c' => (String_bac, None),
                _ => (Err(vec!['c']), None),
            },
            String_bac => match input {
                'k' => (String_back, None),
                _ => (Err(vec!['k']), None),
            },
            String_back => match input {
                'g' => (String_backg, None),
                _ => (Err(vec!['g']), None),
            },
            String_backg => match input {
                'r' => (String_backgr, None),
                _ => (Err(vec!['r']), None),
            },
            String_backgr => match input {
                'o' => (String_backgro, None),
                _ => (Err(vec!['o']), None),
            },
            String_backgro => match input {
                'u' => (String_backgrou, None),
                _ => (Err(vec!['u']), None),
            },
            String_backgrou => match input {
                'n' => (String_backgroun, None),
                _ => (Err(vec!['n']), None),
            },
            String_backgroun => match input {
                'd' => (String_background, None),
                _ => (Err(vec!['d']), None),
            },
            String_background => match input {
                '_' => (String_background_, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_ => match input {
                'c' => (String_background_c, None),
                _ => (Err(vec!['c']), None),
            },
            String_background_c => match input {
                'o' => (String_background_co, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_co => match input {
                'l' => (String_background_col, None),
                _ => (Err(vec!['l']), None),
            },
            String_background_col => match input {
                'o' => (String_background_colo, None),
                _ => (Err(vec!['o']), None),
            },
            String_background_colo => match input {
                'r' => (Start, Some(AttributBackgroundColor)),
                _ => (Err(vec!['r']), None),
            },
            String_s => match input {
                'h' => (String_sh, None),
                _ => (Err(vec!['h']), None),
            },
            String_sh => match input {
                'a' => (String_sha, None),
                _ => (Err(vec!['a']), None),
            },
            String_sha => match input {
                'p' => (String_shap, None),
                _ => (Err(vec!['p']), None),
            },
            String_shap => match input {
                'e' => (String_shape, None),
                _ => (Err(vec!['e']), None),
            },
            String_shape => match input {
                's' => (Start, Some(AttributShapes)),
                _ => (Err(vec!['s']), None),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_source_code() {
        let source_code = "{visible_extentbackground_colorshapes}";
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ];

        assert_eq!(to_tokens(source_code), expected_tokens);
    }

    #[test]
    fn custom_visible_extent() {
        let source_code = "{visible_extent{x=+10001111y=-001010101111111}background_colorshapes}";
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            One,
            Zero,
            Zero,
            Zero,
            One,
            One,
            One,
            One,
            AttributY,
            PrimitiveValue,
            NegativValue,
            Zero,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            AttributShapes,
            StructEnd,
        ];

        assert_eq!(to_tokens(source_code), expected_tokens);
    }

    #[test]
    fn custom_background_color() {
        let source_code =
            "{visible_extentbackground_color{red=11111111green=00000000blue=0}shapes}";
        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            Zero,
            AttributBlue,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributShapes,
            StructEnd,
        ];

        assert_eq!(to_tokens(source_code), expected_tokens);
    }

    #[test]
    fn custom_full() {
        let source_code = r#"{
            visible_extent {
                x = -10
                y = 11111
            }
            background_color {
                red = 1
                green = 0
                blue = 11
            }
            shapes [
                {
                    position
                    rotation
                    width
                    border_color
                    fill_color
                    vertices
                }
                {
                    position {
                        x = +0
                        y = -0
                    }
                    rotation = 0
                    shapes [ 
                        { 
                            position { 
                                x = 1 
                                y = 0
                            }
                            rotation = 11
                            width = -111111111111111
                            border_color { 
                                red = 1
                                green = 0
                                blue = 
                                11
                            }
                            fill_color {
                                red = 11111111
                                green = 0
                                blue = 10
                            }
                            vertices [ 
                                {
                                    x = -10
                                    y = 10
                                }
                                {
                                    x = -1
                                    y=1
                                }
                            ]
                        }
                        {
                            position
                            rotation
                            width
                            border_color
                            fill_color
                            vertices
                        }
                    ]
                }
                {
                    position {
                        x = +0
                        y = -1
                    }
                    rotation = 1
                    width = -10
                    border_color {
                        red = 1
                        green = 0
                        blue = 101
                    }
                    fill_color {
                        red = 0
                        green = 1
                        blue = 10
                    }
                    vertices [
                        {
                            
                            x = 1010
                            y = 10101
                        }
                        {
                            x = 101010
                            y = 1010101
                        }
                    ]
                }
                {
                    position
                    rotation
                    shapes
                }
            ]
        }"#;

        let expected_tokens = vec![
            StructStart,
            AttributVisibleExtent,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            StructEnd,
            AttributBackgroundColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            Zero,
            AttributShapes,
            ArrayStart,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            AttributY,
            PrimitiveValue,
            Zero,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativValue,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            NegativValue,
            One,
            AttributY,
            PrimitiveValue,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            StructStart,
            AttributX,
            PrimitiveValue,
            PositiveValue,
            Zero,
            AttributY,
            PrimitiveValue,
            NegativValue,
            One,
            StructEnd,
            AttributRotation,
            PrimitiveValue,
            One,
            AttributWidth,
            PrimitiveValue,
            NegativValue,
            One,
            Zero,
            AttributBorderColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            One,
            AttributGreen,
            PrimitiveValue,
            Zero,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            One,
            StructEnd,
            AttributFillColor,
            StructStart,
            AttributRed,
            PrimitiveValue,
            Zero,
            AttributGreen,
            PrimitiveValue,
            One,
            AttributBlue,
            PrimitiveValue,
            One,
            Zero,
            StructEnd,
            AttributVertices,
            ArrayStart,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            StructStart,
            AttributX,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            AttributY,
            PrimitiveValue,
            One,
            Zero,
            One,
            Zero,
            One,
            Zero,
            One,
            StructEnd,
            ArrayEnd,
            StructEnd,
            StructStart,
            AttributPosition,
            AttributRotation,
            AttributShapes,
            StructEnd,
            ArrayEnd,
            StructEnd,
        ];

        assert_eq!(to_tokens(source_code), expected_tokens);
    }
}
