use self::State::*;
use crate::{
    error_handling::Error::{self, LexerInvalidChar},
    token::{Token, Value, Value::*},
};

macro_rules! lexer_branch {
    ($input:expr, $( $expected_input:expr => $next_state:expr ),* $(,)?) =>
    {
        match $input {
            $( $expected_input => $next_state, )*
            _ => (Err(vec![$($expected_input),*]), None),
        }
    }
}

pub fn to_tokens(source_code: &str) -> Result<Vec<Token>, Error> {
    let mut output: Vec<Token> = vec![];
    let mut current_state = Start;
    for (line_number, line_content) in source_code.lines().enumerate() {
        for (offset, char) in source_code.chars().enumerate() {
            let (next_state, value) = current_state.next_state(char);

            if let Err(expected_chars) = next_state {
                return Result::Err(LexerInvalidChar {
                    error_char: char,
                    expected_chars,
                    error_line_content: line_content.to_string(),
                    error_line_number: line_number,
                    error_offset: offset,
                });
            }
            if let Some(value) = value {
                output.push(Token::new(line_number, offset, value));
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
    String_s,
    String_sh,
    String_sha,
    String_shap,
    String_shape,
    Err(Vec<char>),
}

impl State {
    fn next_state(&self, char: char) -> (Self, Option<Value>) {
        match self {
            Start => {
                if char.is_whitespace() {
                    return (Start, None);
                }
                lexer_branch!(char,
                    '=' => (Start, Some(EqualsChar)),
                    '-' => (Start, Some(NegativeSign)),
                    '+' => (Start, Some(PositiveSign)),
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
                )
            }
            String_r => lexer_branch!(char,
                'e' => (String_re, None),
                'o' => (String_ro, None),
            ),
            String_re => lexer_branch!(char,
                'd' => (Start, Some(AttributRed)),
            ),
            String_g => lexer_branch!(char,
                'r' => (String_gr, None),
            ),
            String_gr => lexer_branch!(char,
                'e' => (String_gre, None),
            ),
            String_gre => lexer_branch!(char,
                'e' => (String_gree, None),
            ),
            String_gree => lexer_branch!(char,
                'n' => (Start, Some(AttributGreen)),
            ),
            String_b => lexer_branch!(char,
                'l' => (String_bl, None),
                'o' => (String_bo, None),
                'a' => (String_ba, None),
            ),
            String_bl => lexer_branch!(char,
                'u' => (String_blu, None),
            ),
            String_blu => lexer_branch!(char,
                'e' => (Start, Some(AttributBlue)),
            ),
            String_p => lexer_branch!(char,
                'o' => (String_po, None),
            ),
            String_po => lexer_branch!(char,
                's' => (String_pos, None),
            ),
            String_pos => lexer_branch!(char,
                'i' => (String_posi, None),
            ),
            String_posi => lexer_branch!(char,
                't' => (String_posit, None),
            ),
            String_posit => lexer_branch!(char,
                'i' => (String_positi, None),
            ),
            String_positi => lexer_branch!(char,
                'o' => (String_positio, None),
            ),
            String_positio => lexer_branch!(char,
                'n' => (Start, Some(AttributPosition)),
            ),
            String_ro => lexer_branch!(char,
                't' => (String_rot, None),
            ),
            String_rot => lexer_branch!(char,
                'a' => (String_rota, None),
            ),
            String_rota => lexer_branch!(char,
                't' => (String_rotat, None),
            ),
            String_rotat => lexer_branch!(char,
                'i' => (String_rotati, None),
            ),
            String_rotati => lexer_branch!(char,
                'o' => (String_rotatio, None),
            ),
            String_rotatio => lexer_branch!(char,
                'n' => (Start, Some(AttributRotation)),
            ),
            String_w => lexer_branch!(char,
                'i' => (String_wi, None),
            ),
            String_wi => lexer_branch!(char,
                'd' => (String_wid, None),
            ),
            String_wid => lexer_branch!(char,
                't' => (String_widt, None),),
            String_widt => lexer_branch!(char,
                'h' => (Start, Some(AttributWidth)),),
            String_bo => lexer_branch!(char,
                'r' => (String_bor, None),),
            String_bor => lexer_branch!(char,
                'd' => (String_bord, None),),
            String_bord => lexer_branch!(char,
                'e' => (String_borde, None),),
            String_borde => lexer_branch!(char,
                'r' => (String_border, None),),
            String_border => lexer_branch!(char,
                '_' => (String_border_, None),),
            String_border_ => lexer_branch!(char,
                'c' => (String_border_c, None),),
            String_border_c => lexer_branch!(char,
                'o' => (String_border_co, None),),
            String_border_co => lexer_branch!(char,
                'l' => (String_border_col, None),),
            String_border_col => lexer_branch!(char,
                'o' => (String_border_colo, None),),
            String_border_colo => lexer_branch!(char,
                'r' => (Start, Some(AttributBorderColor)),),
            String_f => lexer_branch!(char,
                'i' => (String_fi, None),),
            String_fi => lexer_branch!(char,
                'l' => (String_fil, None),),
            String_fil => lexer_branch!(char,
                'l' => (String_fill, None),),
            String_fill => lexer_branch!(char,
                '_' => (String_fill_, None),),
            String_fill_ => lexer_branch!(char,
                'c' => (String_fill_c, None),),
            String_fill_c => lexer_branch!(char,
                'o' => (String_fill_co, None),),
            String_fill_co => lexer_branch!(char,
                'l' => (String_fill_col, None),),
            String_fill_col => lexer_branch!(char,
                'o' => (String_fill_colo, None),),
            String_fill_colo => lexer_branch!(char,
                'r' => (Start, Some(AttributFillColor)),),
            String_v => lexer_branch!(char,
                'e' => (String_ve, None),
                'i' => (String_vi, None),),
            String_ve => lexer_branch!(char,
                'r' => (String_ver, None),),
            String_ver => lexer_branch!(char,
                't' => (String_vert, None),),
            String_vert => lexer_branch!(char,
                'i' => (String_verti, None),),
            String_verti => lexer_branch!(char,
                'c' => (String_vertic, None),),
            String_vertic => lexer_branch!(char,
                'e' => (String_vertice, None),),
            String_vertice => lexer_branch!(char,
                's' => (Start, Some(AttributVertices)),),
            String_vi => lexer_branch!(char,
                's' => (String_vis, None),),
            String_vis => lexer_branch!(char,
                'i' => (String_visi, None),),
            String_visi => lexer_branch!(char,
                'b' => (String_visib, None),),
            String_visib => lexer_branch!(char,
                'l' => (String_visibl, None),),
            String_visibl => lexer_branch!(char,
                'e' => (String_visible, None),),
            String_visible => lexer_branch!(char,
                '_' => (String_visible_, None),),
            String_visible_ => lexer_branch!(char,
                'e' => (String_visible_e, None),),
            String_visible_e => lexer_branch!(char,
                'x' => (String_visible_ex, None),),
            String_visible_ex => lexer_branch!(char,
                't' => (String_visible_ext, None),),
            String_visible_ext => lexer_branch!(char,
                'e' => (String_visible_exte, None),),
            String_visible_exte => lexer_branch!(char,
                'n' => (String_visible_exten, None),),
            String_visible_exten => lexer_branch!(char,
                't' => (Start, Some(AttributVisibleExtent)),),
            String_ba => lexer_branch!(char,
                'c' => (String_bac, None),),
            String_bac => lexer_branch!(char,
                'k' => (String_back, None),),
            String_back => lexer_branch!(char,
                'g' => (String_backg, None),),
            String_backg => lexer_branch!(char,
                'r' => (String_backgr, None),),
            String_backgr => lexer_branch!(char,
                'o' => (String_backgro, None),),
            String_backgro => lexer_branch!(char,
                'u' => (String_backgrou, None),),
            String_backgrou => lexer_branch!(char,
                'n' => (String_backgroun, None),),
            String_backgroun => lexer_branch!(char,
                'd' => (String_background, None),),
            String_background => lexer_branch!(char,
                '_' => (String_background_, None),),
            String_background_ => lexer_branch!(char,
                'c' => (String_background_c, None),),
            String_background_c => lexer_branch!(char,
                'o' => (String_background_co, None),),
            String_background_co => lexer_branch!(char,
                'l' => (String_background_col, None),),
            String_background_col => lexer_branch!(char,
                'o' => (String_background_colo, None),),
            String_background_colo => lexer_branch!(char,
                'r' => (Start, Some(AttributBackgroundColor)),),
            String_s => lexer_branch!(char,
                'h' => (String_sh, None),),
            String_sh => lexer_branch!(char,
                'a' => (String_sha, None),),
            String_sha => lexer_branch!(char,
                'p' => (String_shap, None),),
            String_shap => lexer_branch!(char,
                'e' => (String_shape, None),),
            String_shape => lexer_branch!(char,
                's' => (Start, Some(AttributShapes)),),
            Err(_) => {
                panic!("BUG: The `next_state` method should never be called on the `Err` state.")
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
            "`actual_tokens_iter` ended before `expected_tokens_iter`.\n`expected_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.",
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
            "`expected_tokens_iter` ended before `actual_tokens_iter`.\n`actual_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.",
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
        Result::Err(format!("`actual_token` does not equal `current_token`.\n `actual_token`: `{:?}`, `expected_token`: `{:?}`,\n`index`: {},\n`actual_tokens`: `{:?}`,\n`expected_tokens`: `{:?}`.", actual_token, expected_token,
        index,
        actual_tokens,
        expected_tokens))
    }

    #[test]
    fn test_all_tokens() -> Result<(), String> {
        let actual_tokens = to_tokens("=- + 0 1 { } [ ] red green blue x y position rotation width border_color fill_color vertices visible_extent background_color shapes");
        let expected_tokens = vec![
            EqualsChar,
            NegativeSign,
            PositiveSign,
            Zero,
            One,
            StructStart,
            StructEnd,
            ArrayStart,
            ArrayEnd,
            AttributRed,
            AttributGreen,
            AttributBlue,
            AttributX,
            AttributY,
            AttributPosition,
            AttributRotation,
            AttributWidth,
            AttributBorderColor,
            AttributFillColor,
            AttributVertices,
            AttributVisibleExtent,
            AttributBackgroundColor,
            AttributShapes,
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
            Ok(_) => Result::Err("`to_tokens` should return an error.".to_string()),
            Result::Err(_) => Ok(()),
        }
    }
}
