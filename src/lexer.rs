use self::State::*;
use crate::{
    error_handling::Error::{self, LexerUnexpectedChar},
    token::{Token, Value, Value::*},
};

macro_rules! transition {
    ($input:expr, $( $expected_input:expr => $next_state:expr ),* $(,)?) =>
    {
        match $input {
            $( $expected_input => $next_state, )*
            _ => (UnexpectedChar(vec![$($expected_input),*]), None),
        }
    }
}

pub fn to_tokens(source_code: &str) -> Result<Vec<Token>, Error> {
    let mut output: Vec<Token> = vec![];
    let mut current_state = Start;
    for (line_number, line_content) in source_code.lines().enumerate() {
        for (offset, char) in line_content.chars().enumerate() {
            let (next_state, value) = current_state.next_state(char);

            if let UnexpectedChar(expected_chars) = next_state {
                return Result::Err(LexerUnexpectedChar {
                    error_char: char,
                    expected_chars,
                    error_line_content: line_content.to_string(),
                    error_line_number: line_number,
                    error_offset: offset,
                });
            }
            if let Some(value) = value {
                output.push(Token::new_from_end(line_number, offset + 1, value));
            }

            current_state = next_state;
        }
    }
    Ok(output)
}

#[derive(Debug)]
#[allow(unused, non_camel_case_types)]
enum State {
    Start,
    String_r,
    String_re,
    String_g,
    String_gr,
    String_gre,
    String_gree,
    String_b,
    String_bl,
    String_blu,
    String_p,
    String_po,
    String_pos,
    String_posi,
    String_posit,
    String_positi,
    String_positio,
    String_ro,
    String_rot,
    String_rota,
    String_rotat,
    String_rotati,
    String_rotatio,
    String_w,
    String_wi,
    String_wid,
    String_widt,
    String_bo,
    String_bor,
    String_bord,
    String_borde,
    String_border,
    String_border_,
    String_border_c,
    String_border_co,
    String_border_col,
    String_border_colo,
    String_f,
    String_fi,
    String_fil,
    String_fill,
    String_fill_,
    String_fill_c,
    String_fill_co,
    String_fill_col,
    String_fill_colo,
    String_v,
    String_ve,
    String_ver,
    String_vert,
    String_verti,
    String_vertic,
    String_vertice,
    String_vi,
    String_vis,
    String_visi,
    String_visib,
    String_visibl,
    String_visible,
    String_visible_,
    String_visible_e,
    String_visible_ex,
    String_visible_ext,
    String_visible_exte,
    String_visible_exten,
    String_ba,
    String_bac,
    String_back,
    String_backg,
    String_backgr,
    String_backgro,
    String_backgrou,
    String_backgroun,
    String_background,
    String_background_,
    String_background_c,
    String_background_co,
    String_background_col,
    String_background_colo,
    String_c,
    String_ch,
    String_chi,
    String_chil,
    String_child,
    String_childr,
    String_childre,
    UnexpectedChar(Vec<char>),
}

impl State {
    fn next_state(&self, char: char) -> (Self, Option<Value>) {
        match self {
            Start => {
                if char.is_whitespace() {
                    return (Start, None);
                }
                transition!(char,
                    '=' => (Start, Some(Equals)),
                    '-' => (Start, Some(Plus)),
                    '+' => (Start, Some(Minus)),
                    '0' => (Start, Some(Zero)),
                    '1' => (Start, Some(One)),
                    '{' => (Start, Some(LeftBrace)),
                    '}' => (Start, Some(RightBrace)),
                    '[' => (Start, Some(LeftBracket)),
                    ']' => (Start, Some(RightBracket)),
                    'r' => (String_r, None),
                    'g' => (String_g, None),
                    'b' => (String_b, None),
                    'x' => (Start, Some(X)),
                    'y' => (Start, Some(Y)),
                    'p' => (String_p, None),
                    'w' => (String_w, None),
                    'f' => (String_f, None),
                    'v' => (String_v, None),
                    'c' => (String_c, None),
                )
            }
            String_r => transition!(char,
                'e' => (String_re, None),
                'o' => (String_ro, None),
            ),
            String_re => transition!(char,
                'd' => (Start, Some(Red)),
            ),
            String_g => transition!(char,
                'r' => (String_gr, None),
            ),
            String_gr => transition!(char,
                'e' => (String_gre, None),
            ),
            String_gre => transition!(char,
                'e' => (String_gree, None),
            ),
            String_gree => transition!(char,
                'n' => (Start, Some(Green)),
            ),
            String_b => transition!(char,
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
            ),
            String_bl => transition!(char,
                'u' => (String_blu, None),
            ),
            String_blu => transition!(char,
                'e' => (Start, Some(Blue)),
            ),
            String_p => transition!(char,
                'o' => (String_po, None),
            ),
            String_po => transition!(char,
                's' => (String_pos, None),
            ),
            String_pos => transition!(char,
                'i' => (String_posi, None),
            ),
            String_posi => transition!(char,
                't' => (String_posit, None),
            ),
            String_posit => transition!(char,
                'i' => (String_positi, None),
            ),
            String_positi => transition!(char,
                'o' => (String_positio, None),
            ),
            String_positio => transition!(char,
                'n' => (Start, Some(Position)),
            ),
            String_ro => transition!(char,
                't' => (String_rot, None),
            ),
            String_rot => transition!(char,
                'a' => (String_rota, None),
            ),
            String_rota => transition!(char,
                't' => (String_rotat, None),
            ),
            String_rotat => transition!(char,
                'i' => (String_rotati, None),
            ),
            String_rotati => transition!(char,
                'o' => (String_rotatio, None),
            ),
            String_rotatio => transition!(char,
                'n' => (Start, Some(Rotation)),
            ),
            String_w => transition!(char,
                'i' => (String_wi, None),
            ),
            String_wi => transition!(char,
                'd' => (String_wid, None),
            ),
            String_wid => transition!(char,
                't' => (String_widt, None),),
            String_widt => transition!(char,
                'h' => (Start, Some(Width)),),
            String_bo => transition!(char,
                'r' => (String_bor, None),),
            String_bor => transition!(char,
                'd' => (String_bord, None),),
            String_bord => transition!(char,
                'e' => (String_borde, None),),
            String_borde => transition!(char,
                'r' => (String_border, None),),
            String_border => transition!(char,
                '_' => (String_border_, None),),
            String_border_ => transition!(char,
                'c' => (String_border_c, None),),
            String_border_c => transition!(char,
                'o' => (String_border_co, None),),
            String_border_co => transition!(char,
                'l' => (String_border_col, None),),
            String_border_col => transition!(char,
                'o' => (String_border_colo, None),),
            String_border_colo => transition!(char,
                'r' => (Start, Some(BorderColor)),),
            String_f => transition!(char,
                'i' => (String_fi, None),),
            String_fi => transition!(char,
                'l' => (String_fil, None),),
            String_fil => transition!(char,
                'l' => (String_fill, None),),
            String_fill => transition!(char,
                '_' => (String_fill_, None),),
            String_fill_ => transition!(char,
                'c' => (String_fill_c, None),),
            String_fill_c => transition!(char,
                'o' => (String_fill_co, None),),
            String_fill_co => transition!(char,
                'l' => (String_fill_col, None),),
            String_fill_col => transition!(char,
                'o' => (String_fill_colo, None),),
            String_fill_colo => transition!(char,
                'r' => (Start, Some(FillColor)),),
            String_v => transition!(char,
                'e' => (String_ve, None),
                'i' => (String_vi, None),),
            String_ve => transition!(char,
                'r' => (String_ver, None),),
            String_ver => transition!(char,
                't' => (String_vert, None),),
            String_vert => transition!(char,
                'i' => (String_verti, None),),
            String_verti => transition!(char,
                'c' => (String_vertic, None),),
            String_vertic => transition!(char,
                'e' => (String_vertice, None),),
            String_vertice => transition!(char,
                's' => (Start, Some(Vertices)),),
            String_vi => transition!(char,
                's' => (String_vis, None),),
            String_vis => transition!(char,
                'i' => (String_visi, None),),
            String_visi => transition!(char,
                'b' => (String_visib, None),),
            String_visib => transition!(char,
                'l' => (String_visibl, None),),
            String_visibl => transition!(char,
                'e' => (String_visible, None),),
            String_visible => transition!(char,
                '_' => (String_visible_, None),),
            String_visible_ => transition!(char,
                'e' => (String_visible_e, None),),
            String_visible_e => transition!(char,
                'x' => (String_visible_ex, None),),
            String_visible_ex => transition!(char,
                't' => (String_visible_ext, None),),
            String_visible_ext => transition!(char,
                'e' => (String_visible_exte, None),),
            String_visible_exte => transition!(char,
                'n' => (String_visible_exten, None),),
            String_visible_exten => transition!(char,
                't' => (Start, Some(VisibleExtent)),),
            String_ba => transition!(char,
                'c' => (String_bac, None),),
            String_bac => transition!(char,
                'k' => (String_back, None),),
            String_back => transition!(char,
                'g' => (String_backg, None),),
            String_backg => transition!(char,
                'r' => (String_backgr, None),),
            String_backgr => transition!(char,
                'o' => (String_backgro, None),),
            String_backgro => transition!(char,
                'u' => (String_backgrou, None),),
            String_backgrou => transition!(char,
                'n' => (String_backgroun, None),),
            String_backgroun => transition!(char,
                'd' => (String_background, None),),
            String_background => transition!(char,
                '_' => (String_background_, None),),
            String_background_ => transition!(char,
                'c' => (String_background_c, None),),
            String_background_c => transition!(char,
                'o' => (String_background_co, None),),
            String_background_co => transition!(char,
                'l' => (String_background_col, None),),
            String_background_col => transition!(char,
                'o' => (String_background_colo, None),),
            String_background_colo => transition!(char,
                'r' => (Start, Some(BackgroundColor)),),
            String_c => transition!(char,
                'h' => (String_ch, None),
            ),
            String_ch => transition!(char,
                'i' => (String_chi, None),
            ),
            String_chi => transition!(char,
                'l' => (String_chil, None),
            ),
            String_chil => transition!(char,
                'd' => (String_child, None),
            ),
            String_child => transition!(char,
                'r' => (String_childr, None),
            ),
            String_childr => transition!(char,
                'e' => (String_childre, None),
            ),
            String_childre => transition!(char,
                'n' => (Start, Some(Children)),
            ),
            UnexpectedChar(_) => {
                panic!("BUG: The 'next_state' method should never be called on the 'Err' state.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Value;

    use super::*;
    pub fn is_same_variant<T>(v1: &T, v2: &T) -> bool {
        use std::mem;

        mem::discriminant(v1) == mem::discriminant(v2)
    }

    fn err_actual_tokens_iter_ends(
        expected_token: &Value,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<Value>,
    ) -> Result<(), String> {
        Result::Err(format!(
            "'actual_tokens_iter' ended before 'expected_tokens_iter'.\n'expected_token': '{:?}',\n'index': {},\n'actual_tokens': '{:?}',\n'expected_tokens': '{:?}'.",
            expected_token,
            index,
            actual_tokens,
            expected_tokens
        ))
    }

    fn err_expected_tokens_iter_ends(
        actual_token: &Token,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<Value>,
    ) -> Result<(), String> {
        Result::Err(format!(
            "'expected_tokens_iter' ended before 'actual_tokens_iter'.\n'actual_token': '{:?}',\n'index': {},\n'actual_tokens': '{:?}',\n'expected_tokens': '{:?}'.",
            &actual_token,
            index,
            actual_tokens,
            expected_tokens
        ))
    }

    fn err_token_mismatch(
        actual_token: &Token,
        expected_token: &Value,
        index: &usize,
        actual_tokens: &Vec<Token>,
        expected_tokens: &Vec<Value>,
    ) -> Result<(), String> {
        Result::Err(format!("'actual_token' does not equal 'current_token'.\n 'actual_token': '{:?}', 'expected_token': '{:?}',\n'index': {},\n'actual_tokens': '{:?}',\n'expected_tokens': '{:?}'.", actual_token, expected_token,
        index,
        actual_tokens,
        expected_tokens))
    }

    #[test]
    fn test_all_tokens() -> Result<(), String> {
        let actual_tokens = to_tokens("=- + 0 1 { } [ ] red green blue x y position rotation width border_color fill_color vertices visible_extent background_color children");
        let expected_tokens = vec![
            Equals,
            Plus,
            Minus,
            Zero,
            One,
            LeftBrace,
            RightBrace,
            LeftBracket,
            RightBracket,
            Red,
            Green,
            Blue,
            X,
            Y,
            Position,
            Rotation,
            Width,
            BorderColor,
            FillColor,
            Vertices,
            VisibleExtent,
            BackgroundColor,
            Children,
        ];

        let actual_tokens = match actual_tokens {
            Ok(tokens) => tokens,
            Result::Err(error) => return Result::Err(error.error_message()),
        };
        let mut actual_tokens_iter = actual_tokens.iter();
        let mut expected_tokens_iter = expected_tokens.iter();

        let mut index: usize = 0;
        loop {
            let actual_token = actual_tokens_iter.next();
            let expected_token = expected_tokens_iter.next();

            match (actual_token, expected_token) {
                (None, None) => return Ok(()),
                (None, Some(expected_token)) => {
                    return err_actual_tokens_iter_ends(
                        expected_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), None) => {
                    return err_expected_tokens_iter_ends(
                        actual_token,
                        &index,
                        &actual_tokens,
                        &expected_tokens,
                    )
                }
                (Some(actual_token), Some(expected_token)) => {
                    if !is_same_variant(actual_token.value(), expected_token) {
                        return err_token_mismatch(
                            actual_token,
                            expected_token,
                            &index,
                            &actual_tokens,
                            &expected_tokens,
                        );
                    }
                }
            };
            index += 1;
        }
    }

    #[test]
    fn test_invalid_token() -> Result<(), String> {
        let result = to_tokens("invalid");
        match result {
            Ok(_) => Result::Err("'to_tokens' should return an error.".to_string()),
            Result::Err(_) => Ok(()),
        }
    }
}
